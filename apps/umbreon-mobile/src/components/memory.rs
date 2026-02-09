use crate::state::use_app_context;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn MemoryPane() -> Element {
    let ctx = use_app_context();
    let memory = ctx.memory_panel.read().clone();

    rsx! {
        section { class: "memory-pane",
            header { h2 { "In-app Memory" } }
            p { class: "status", "Synced: {memory.synced}" }
            if let Some(ts) = memory.last_synced {
                p { class: "status", "Last sync: {ts}" }
            }
            h3 { "Highlights" }
            if memory.highlights.is_empty() {
                p { class: "empty-state", "No highlights captured yet." }
            } else {
                ul {
                    for note in memory.highlights.into_iter() {
                        li { "{note}" }
                    }
                }
            }
        }
    }
}
