use crate::state::{MediaKind, use_app_context};
use dioxus::prelude::*;

use super::player::NowPlayingPane;
use super::timeline::TimelinePane;
use crate::timeline::trigger_feed_sync;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExploreView {
    Menu,
    Timeline,
    Vod,
    Live,
}

#[allow(non_snake_case)]
pub fn ExplorePane() -> Element {
    let mut view = use_signal(|| ExploreView::Menu);
    let current = *view.read();
    let ctx = use_app_context();
    let feed_items = ctx.feed_items;
    let feed_server_url = ctx.feed_server_url;
    let settings_status = ctx.settings_status;
    let is_syncing = settings_status
        .read()
        .as_deref()
        .map(|status| status == "Syncing feeds...")
        .unwrap_or(false);

    rsx! {
        section { class: "explore-pane",
            if current != ExploreView::Menu {
                div { class: "explore-subheader",
                    button {
                        class: "explore-back",
                        onclick: move |_| {
                            *view.write() = ExploreView::Menu;
                        },
                        span { class: "material-icons", "chevron_left" }
                        span { "返回" }
                    }
                    if current == ExploreView::Timeline {
                        button {
                            class: if is_syncing { "explore-sync is-loading" } else { "explore-sync" },
                            disabled: is_syncing,
                            onclick: move |_| {
                                if is_syncing {
                                    return;
                                }
                                let url = feed_server_url.read().trim().to_string();
                                trigger_feed_sync(url, feed_items.clone(), settings_status.clone());
                            },
                            span { class: "material-icons", "refresh" }
                        }
                    }
                }
            }
            match current {
                ExploreView::Menu => rsx!(
                    div { class: "explore-card",
                        ExploreItem {
                            icon: "language",
                            label: "时间线",
                            on_open: move |_| {
                                *view.write() = ExploreView::Timeline;
                            }
                        }
                        ExploreItem {
                            icon: "play_circle_filled",
                            label: "追番",
                            on_open: move |_| {
                                *view.write() = ExploreView::Vod;
                            }
                        }
                        ExploreItem {
                            icon: "live_tv",
                            label: "直播",
                            on_open: move |_| {
                                *view.write() = ExploreView::Live;
                            }
                        }
                    }
                ),
                ExploreView::Timeline => rsx!(TimelinePane {}),
                ExploreView::Vod => rsx!(NowPlayingPane { mode: MediaKind::Vod }),
                ExploreView::Live => rsx!(NowPlayingPane { mode: MediaKind::Live }),
            }
        }
    }
}

#[component]
fn ExploreItem(icon: &'static str, label: &'static str, on_open: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "explore-item",
            onclick: move |_| {
                on_open.call(());
            },
            span { class: "explore-icon material-icons", "{icon}" }
            span { class: "explore-label", "{label}" }
            span { class: "explore-chevron material-icons", "chevron_right" }
        }
    }
}
