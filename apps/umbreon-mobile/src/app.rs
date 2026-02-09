use crate::components::{MemoryPane, NavigationBar, NowPlayingPane, TimelinePane};
use crate::state::{self, AppContext, NavSection};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn AppRoot() -> Element {
    let nav = use_signal(|| NavSection::Timeline);
    let feed_items = use_signal(state::mock_feed_items);
    let live_streams = use_signal(state::mock_live_streams);
    let now_playing = use_signal(state::mock_initial_session);
    let memory_panel = use_signal(state::mock_memory_panel);

    let ctx_seed = AppContext {
        nav,
        feed_items,
        live_streams,
        now_playing,
        memory_panel,
    };

    let ctx = use_context_provider({
        let provided = ctx_seed.clone();
        move || provided
    });

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
