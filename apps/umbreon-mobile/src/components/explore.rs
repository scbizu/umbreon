use crate::state::MediaKind;
use dioxus::prelude::*;

use super::player::NowPlayingPane;
use super::timeline::TimelinePane;

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
