use crate::components::{MemoryPane, NavigationBar, NowPlayingPane, TimelinePane};
use crate::state::{AppContext, NavSection};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn AppRoot() -> Element {
    let ctx = use_context_provider(AppContext::new);
    let active_nav = *ctx.nav.read();

    rsx! {
        main { class: "umbreon-shell",
            NavigationBar {}
            section { class: "umbreon-content",
                match active_nav {
                    NavSection::Timeline => rsx!(TimelinePane {}),
                    NavSection::Live | NavSection::Vod => rsx!(NowPlayingPane {}),
                    NavSection::Memory => rsx!(MemoryPane {}),
                }
            }
        }
    }
}
