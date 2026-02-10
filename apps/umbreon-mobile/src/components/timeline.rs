use crate::state::{use_app_context, FeedItem, FeedSourceKind};
use dioxus::prelude::*;
use tracing::info;
use url::Url;

#[allow(non_snake_case)]
pub fn TimelinePane() -> Element {
    let ctx = use_app_context();
    let items = ctx.feed_items.read().clone();
    let mut selected = use_signal(|| None::<FeedItem>);

    rsx! {
        section { class: "timeline-pane",
            if items.is_empty() {
                p { class: "empty-state", "No feed entries yet. Configure sources via GitHub Gist." }
            } else {
                for item in items.into_iter() {
                    FeedCard {
                        item: item.clone(),
                        on_open: move |card| {
                            *selected.write() = Some(card);
                        }
                    }
                }
            }
        }
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

#[component]
fn FeedCard(item: FeedItem, on_open: EventHandler<FeedItem>) -> Element {
    let source_label = match item.source {
        FeedSourceKind::Atom => "ATOM",
        FeedSourceKind::RssHub => "RSSHub",
        FeedSourceKind::Custom => "Crawler",
    };

    let card_item = item.clone();
    let fallback = item.author.chars().next().unwrap_or('?');
    let host = Url::parse(&item.link)
        .ok()
        .and_then(|url| url.host_str().map(|value| value.to_string()))
        .unwrap_or_else(|| "source".to_string());

    rsx! {
        article {
            class: "feed-card",
            key: "{item.id}",
            onclick: move |_| {
                on_open.call(card_item.clone());
            },
            div { class: "post-avatar",
                if let Some(avatar) = item.avatar_url.clone() {
                    img { class: "post-avatar-img", src: "{avatar}" }
                } else {
                    span { class: "post-avatar-fallback", "{fallback}" }
                }
            }
            div { class: "post-body",
                header { class: "post-header",
                    span { class: "post-name", "{item.author}" }
                    span { class: "post-handle", "@{host}" }
                    span { class: "post-dot", "·" }
                    time { class: "post-time", "{item.published_at}" }
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
                        span { "Read more" }
                    }
                    button {
                        class: "post-action",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            info!("add to memory: {}", item.id);
                        },
                        span { class: "material-icons", "bookmark_add" }
                        span { "Add to memory" }
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
