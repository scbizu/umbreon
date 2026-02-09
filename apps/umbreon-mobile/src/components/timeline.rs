use crate::state::{use_app_context, FeedItem, FeedSourceKind};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn TimelinePane() -> Element {
    let ctx = use_app_context();
    let items = ctx.feed_items.read().clone();

    rsx! {
        section { class: "timeline-pane",
            if items.is_empty() {
                p { class: "empty-state", "No feed entries yet. Configure sources via GitHub Gist." }
            } else {
                for item in items.into_iter() {
                    FeedCard { item }
                }
            }
        }
    }
}

#[component]
fn FeedCard(item: FeedItem) -> Element {
    let source_label = match item.source {
        FeedSourceKind::Atom => "ATOM",
        FeedSourceKind::RssHub => "RSSHub",
        FeedSourceKind::Custom => "Crawler",
    };

    rsx! {
        article { class: "feed-card", key: "{item.id}",
            header {
                span { class: "feed-source", "{source_label}" }
                time { class: "feed-time", "{item.published_at}" }
            }
            h2 { class: "feed-title", "{item.title}" }
            p { class: "feed-summary", "{item.summary}" }
            a {
                class: "feed-link",
                href: item.link.clone(),
                target: "_blank",
                "Open"
            }
        }
    }
}
