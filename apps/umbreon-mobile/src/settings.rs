use crate::state::{ThemeMode, use_app_context};
use crate::storage;
use crate::timeline;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn SettingsPane() -> Element {
    let ctx = use_app_context();
    let mut feed_server_url = ctx.feed_server_url;
    let mut memory_server_url = ctx.memory_server_url;
    let settings_status = ctx.settings_status;
    let mut theme = ctx.theme;
    let mode = *theme.read();
    let feed_items = ctx.feed_items;

    rsx! {
        div { class: "settings-pane",
            h2 { "设置" }
            p { "Sync feeds from your feed server (Atom)." }
            div { class: "settings-field",
                label { class: "settings-label", "Feed Server" }
                input {
                    class: "settings-input",
                    r#type: "url",
                    placeholder: "https://feed-aggregator-worker.scnace.workers.dev",
                    value: "{feed_server_url.read()}",
                    oninput: move |evt| {
                        let value = evt.value();
                        *feed_server_url.write() = value.clone();
                        storage::store_feed_server_url(&value);
                    }
                }
                button {
                    class: "settings-sync",
                    onclick: move |_| {
                        let url = feed_server_url.read().trim().to_string();
                        let memory_url = memory_server_url.read().trim().to_string();
                        storage::store_feed_server_url(&url);
                        storage::store_memory_server_url(&memory_url);
                        timeline::trigger_feed_sync(url, feed_items.clone(), settings_status.clone());
                    },
                    span { class: "material-icons", "sync" }
                    span { "Sync feeds" }
                }
            }
            div { class: "settings-field",
                label { class: "settings-label", "Memory Server" }
                input {
                    class: "settings-input",
                    r#type: "url",
                    placeholder: "http://localhost:8787",
                    value: "{memory_server_url.read()}",
                    oninput: move |evt| {
                        let value = evt.value();
                        *memory_server_url.write() = value.clone();
                        storage::store_memory_server_url(&value);
                    }
                }
            }
            div { class: "settings-field",
                label { class: "settings-label", "Theme" }
                div { class: "theme-toggle settings-theme-toggle",
                    span { class: "material-icons theme-icon", "light_mode" }
                    label { class: "theme-switch",
                        input {
                            r#type: "checkbox",
                            checked: mode == ThemeMode::Dark,
                            onchange: move |_| {
                                let current = *theme.read();
                                let next = current.toggle();
                                *theme.write() = next;
                                storage::store_theme(next);
                            }
                        }
                        span { class: "theme-slider" }
                    }
                    span { class: "material-icons theme-icon", "dark_mode" }
                }
            }
            if let Some(message) = settings_status.read().clone() {
                p { class: "settings-status", "{message}" }
            }
        }
    }
}
