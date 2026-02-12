use crate::state::{FeedItem, FeedSourceKind, use_app_context};
use dioxus::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;
use url::Url;

const TIMELINE_ID: &str = "timeline-pane";

struct StackLangStamp {
    label: String,
    icon_url: Option<String>,
}

fn stacklang_stamp(tags: &[String]) -> Option<StackLangStamp> {
    let tag = tags.iter().find_map(|raw| {
        let trimmed = raw.trim();
        trimmed
            .strip_prefix("StackLang:")
            .or_else(|| trimmed.strip_prefix("stacklang:"))
            .map(|value| value.trim().to_string())
    })?;

    let mut icon_key = tag.trim().to_lowercase();
    if icon_key == "golang" {
        icon_key = "go".to_string();
    }
    let icon_url = if icon_key.is_empty() {
        None
    } else if icon_key == "rust" {
        Some("https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-plain.svg".to_string())
    } else {
        Some(format!(
            "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/{0}/{0}-original.svg",
            icon_key
        ))
    };

    Some(StackLangStamp {
        label: tag,
        icon_url,
    })
}

#[cfg(target_arch = "wasm32")]
fn get_timeline_element() -> Option<web_sys::Element> {
    let window = web_sys::window()?;
    let document = window.document()?;
    document.get_element_by_id(TIMELINE_ID)
}

#[cfg(target_arch = "wasm32")]
fn read_scroll_metrics() -> Option<(f64, f64)> {
    let element = get_timeline_element()?;
    let scroll_top = element.scroll_top() as f64;
    let client_height = element.client_height() as f64;
    Some((scroll_top, client_height))
}

#[cfg(target_arch = "wasm32")]
fn measure_card_height() -> Option<f64> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let card = document.query_selector(".feed-card").ok()??;
    Some(card.get_bounding_client_rect().height())
}

#[allow(non_snake_case)]
pub fn TimelinePane() -> Element {
    let ctx = use_app_context();
    let mut items = ctx.feed_items.read().clone();
    let now_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0);
    let cutoff_ts = now_ts.saturating_sub(60 * 24 * 60 * 60);
    items.retain(|item| item.published_ts >= cutoff_ts);
    items.sort_by(|a, b| b.published_ts.cmp(&a.published_ts));
    let total = items.len();
    let mut selected = use_signal(|| None::<FeedItem>);

    #[cfg(target_arch = "wasm32")]
    let timeline_body = render_virtual_timeline(items, total, selected);
    #[cfg(not(target_arch = "wasm32"))]
    let timeline_body = render_paginated_timeline(items, total, selected);

    rsx! {
        {timeline_body}
        if let Some(entry) = selected.read().clone() {
            FeedModal {
                item: entry,
                on_close: move |_| {
                    *selected.write() = None;
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn render_virtual_timeline(
    items: Vec<FeedItem>,
    total: usize,
    mut selected: Signal<Option<FeedItem>>,
) -> Element {
    let mut start_index = use_signal(|| 0usize);
    let mut viewport_height = use_signal(|| 600.0f64);
    let mut item_height = use_signal(|| 720.0f64);

    use_effect(move || {
        if let Some(height) = measure_card_height() {
            if (height - *item_height.read()).abs() > 1.0 {
                *item_height.write() = height.max(1.0);
            }
        }
    });

    let visible_count = ((*viewport_height.read() / *item_height.read()).ceil() as usize)
        .saturating_add(OVERSCAN * 2)
        .max(1);
    let start = (*start_index.read()).min(total);
    let end = (start + visible_count).min(total);
    let slice = items
        .into_iter()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect::<Vec<_>>();
    let top_spacer = (start as f64) * *item_height.read();
    let bottom_spacer = ((total - end) as f64) * *item_height.read();

    rsx! {
        section {
            id: "{TIMELINE_ID}",
            class: "timeline-pane",
            onscroll: move |_| {
                if let Some((scroll_top, client_height)) = read_scroll_metrics() {
                    let height = *item_height.read();
                    let next_start = (scroll_top / height).floor() as usize;
                    let next_start = next_start.saturating_sub(OVERSCAN);
                    *start_index.write() = next_start.min(total);
                    *viewport_height.write() = client_height;
                }
            },
            if slice.is_empty() {
                p { class: "empty-state", "No feed entries yet. Configure sources via GitHub Gist." }
            } else {
                div { class: "timeline-spacer", style: "height: {top_spacer}px" }
                for item in slice.into_iter() {
                    FeedCard {
                        item: item.clone(),
                        on_open: move |card| {
                            *selected.write() = Some(card);
                        }
                    }
                }
                div { class: "timeline-spacer", style: "height: {bottom_spacer}px" }
            }
            div { class: "timeline-footer",
                span { "Showing {end} / {total} feeds" }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn render_paginated_timeline(
    items: Vec<FeedItem>,
    total: usize,
    mut selected: Signal<Option<FeedItem>>,
) -> Element {
    let mut visible = use_signal(|| 30usize);
    if *visible.read() > total {
        *visible.write() = total;
    }
    let slice = items.into_iter().take(*visible.read()).collect::<Vec<_>>();

    rsx! {
        section {
            id: "{TIMELINE_ID}",
            class: "timeline-pane",
            onscroll: move |_| {
                let current = *visible.read();
                if current < total {
                    let next = (current + 30).min(total);
                    *visible.write() = next;
                }
            },
            if slice.is_empty() {
                p { class: "empty-state", "No feed entries yet. Configure sources via GitHub Gist." }
            } else {
                for item in slice.into_iter() {
                    FeedCard {
                        item: item.clone(),
                        on_open: move |card| {
                            *selected.write() = Some(card);
                        }
                    }
                }
            }
            div { class: "timeline-footer",
                span { "Loaded {visible.read()} / {total} feeds" }
            }
        }
    }
}

#[component]
fn FeedCard(item: FeedItem, on_open: EventHandler<FeedItem>) -> Element {
    let source_label = match item.source {
        FeedSourceKind::Atom => "ATOM",
        FeedSourceKind::RssHub => "RSSHub",
        FeedSourceKind::Custom => "Crawler",
    };

    let card_item = item.clone();
    let fallback = item.author.chars().next().unwrap_or('?');
    let host: String = Url::parse(item.link.as_str())
        .ok()
        .and_then(|url| url.host_str().map(str::to_owned))
        .unwrap_or_else(|| "source".to_string());

    let stamp = stacklang_stamp(&item.tags);
    rsx! {
        article {
            class: "feed-card",
            key: "{item.id}",
            onclick: move |_| {
                on_open.call(card_item.clone());
            },
            div { class: "post-avatar",
                span { class: "post-avatar-fallback", "{fallback}" }
            }
            div { class: "post-body",
                header { class: "post-header",
                    span { class: "post-name", "{item.author}" }
                    span { class: "post-handle", "@{host}" }
                    span { class: "post-dot", "·" }
                    time { class: "post-time", "{item.published_at}" }
                    if let Some(stamp) = stamp.as_ref() {
                        span { class: "feed-lang-badge",
                            if let Some(icon) = stamp.icon_url.as_ref() {
                                img { class: "feed-lang-icon", src: "{icon}", alt: "{stamp.label}" }
                            }
                            span { class: "feed-lang-label", "{stamp.label}" }
                        }
                    }
                }
                div { class: "post-text", dangerous_inner_html: "{item.summary}" }
                if !item.title.is_empty() {
                    p { class: "post-title", "{item.title}" }
                }
                div { class: "post-meta",
                    span { class: "post-source", "{source_label}" }
                }
                div { class: "post-actions",
                    a {
                        class: "post-action",
                        href: item.link.clone(),
                        target: "_blank",
                        onclick: move |evt| {
                            evt.stop_propagation();
                        },
                        span { class: "material-icons", "open_in_new" }
                        span { "唤魂" }
                    }
                    button {
                        class: "post-action",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            info!("add to memory: {}", item.id);
                        },
                        span { class: "material-icons", "bookmark_add" }
                        span { "铸魂" }
                    }
                }
            }
        }
    }
}

#[component]
fn FeedModal(item: FeedItem, on_close: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "feed-modal-backdrop",
            onclick: move |_| {
                on_close.call(());
            },
            div {
                class: "feed-modal",
                onclick: move |evt| {
                    evt.stop_propagation();
                },
                header { class: "feed-modal-header",
                    h2 { "{item.title}" }
                    button {
                        class: "feed-modal-close",
                        onclick: move |_| {
                            on_close.call(());
                        },
                        "Close"
                    }
                }
                div { class: "feed-modal-summary", dangerous_inner_html: "{item.summary}" }
                div { class: "feed-modal-meta",
                    span { "Published: {item.published_at}" }
                    span { "Source: {item.source:?}" }
                }
                a {
                    class: "feed-modal-link",
                    href: item.link,
                    target: "_blank",
                    "Open original"
                }
            }
        }
    }
}
