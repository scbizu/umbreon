use crate::general_ai_client;
use crate::state::{ThemeMode, ToastKind, ToastMessage, use_app_context};
use crate::storage;
use crate::timeline;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn SettingsPane() -> Element {
    let ctx = use_app_context();
    let mut feed_server_url = ctx.feed_server_url;
    let mut memory_server_url = ctx.memory_server_url;
    let mut llm_endpoint = ctx.llm_endpoint;
    let mut llm_api_key = ctx.llm_api_key;
    let mut llm_model = ctx.llm_model;
    let llm_models = ctx.llm_models;
    let settings_status = ctx.settings_status;
    let mut toast = ctx.toast;
    let mut theme = ctx.theme;
    let mode = *theme.read();
    let feed_items = ctx.feed_items;
    let can_fetch_models =
        !llm_endpoint.read().trim().is_empty() && !llm_api_key.read().trim().is_empty();
    let can_test = can_fetch_models && !llm_model.read().trim().is_empty();
    let has_models = !llm_models.read().is_empty();
    let mut is_fetching_models = use_signal(|| false);
    let mut is_testing_model = use_signal(|| false);
    let fetching_models = *is_fetching_models.read();
    let testing_model = *is_testing_model.read();

    rsx! {
        div { class: "settings-pane",
            div { class: "settings-field",
                label { class: "settings-label", "Feed Server" }
                div { class: "settings-row",
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
                        class: "settings-sync settings-sync-icon",
                        onclick: move |_| {
                            let url = feed_server_url.read().trim().to_string();
                            let memory_url = memory_server_url.read().trim().to_string();
                            storage::store_feed_server_url(&url);
                            storage::store_memory_server_url(&memory_url);
                            timeline::trigger_feed_sync(url, feed_items.clone(), settings_status.clone());
                        },
                        span { class: "material-icons", "sync" }
                    }
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
                label { class: "settings-label", "通用模型" }
                label { class: "settings-label", "LLM Endpoint" }
                input {
                    class: "settings-input",
                    r#type: "url",
                    placeholder: "https://api.openai.com/v1",
                    value: "{llm_endpoint.read()}",
                    oninput: move |evt| {
                        let value = evt.value();
                        *llm_endpoint.write() = value.clone();
                        storage::store_llm_endpoint(&value);
                    }
                }
                label { class: "settings-label", "API Key" }
                input {
                    class: "settings-input",
                    r#type: "password",
                    placeholder: "API Key",
                    value: "{llm_api_key.read()}",
                    oninput: move |evt| {
                        let value = evt.value();
                        *llm_api_key.write() = value.clone();
                        storage::store_llm_api_key(&value);
                    }
                }
                label { class: "settings-label", "模型" }
                div { class: "settings-row",
                    select {
                        class: "settings-select",
                        value: "{llm_model.read()}",
                        onchange: move |evt| {
                            let value = evt.value();
                            *llm_model.write() = value.clone();
                            storage::store_llm_model(&value);
                        },
                        option { value: "", disabled: true, selected: !has_models,
                            if has_models { "请选择模型" } else { "请先拉取模型" }
                        }
                        for model in llm_models.read().iter() {
                            option { value: "{model}", "{model}" }
                        }
                    }
                    button {
                        class: if fetching_models {
                            "settings-action settings-action-icon is-loading"
                        } else {
                            "settings-action settings-action-icon"
                        },
                        disabled: !can_fetch_models || fetching_models,
                        onclick: move |_| {
                            if *is_fetching_models.read() {
                                return;
                            }
                            let endpoint = llm_endpoint.read().trim().to_string();
                            let api_key = llm_api_key.read().trim().to_string();
                            if endpoint.is_empty() || api_key.is_empty() {
                                *toast.write() = Some(ToastMessage {
                                    kind: ToastKind::Error,
                                    text: "请先填写 Endpoint 和 API Key。".to_string(),
                                });
                                return;
                            }
                            *is_fetching_models.write() = true;
                            let mut llm_models = llm_models.clone();
                            let mut llm_model = llm_model.clone();
                            let mut settings_status = settings_status.clone();
                            let mut toast = toast.clone();
                            let mut is_fetching_models = is_fetching_models.clone();
                            spawn(async move {
                                *settings_status.write() = Some("正在拉取模型...".to_string());
                                match general_ai_client::fetch_models(&endpoint, &api_key).await {
                                    Ok(models) => {
                                        if models.is_empty() {
                                            *toast.write() = Some(ToastMessage {
                                                kind: ToastKind::Error,
                                                text: "未获取到模型列表。".to_string(),
                                            });
                                            *settings_status.write() = Some("模型列表为空。".to_string());
                                        } else {
                                            if llm_model.read().trim().is_empty() {
                                                if let Some(first) = models.first() {
                                                    *llm_model.write() = first.clone();
                                                    storage::store_llm_model(first);
                                                }
                                            }
                                            storage::store_llm_models(&models);
                                            *llm_models.write() = models;
                                            *settings_status.write() = Some("模型列表已更新。".to_string());
                                            *toast.write() = Some(ToastMessage {
                                                kind: ToastKind::Success,
                                                text: "已更新模型列表。".to_string(),
                                            });
                                        }
                                    }
                                    Err(err) => {
                                        *settings_status.write() = Some("模型拉取失败。".to_string());
                                        *toast.write() = Some(ToastMessage {
                                            kind: ToastKind::Error,
                                            text: format!("模型拉取失败：{err}"),
                                        });
                                    }
                                }
                                *is_fetching_models.write() = false;
                            });
                        },
                        span { class: "material-icons", "refresh" }
                    }
                }
                p { class: "settings-hint", "提示：优先选择便宜模型以降低成本。" }
                div { class: "settings-actions",
                    button {
                        class: if testing_model {
                            "settings-action is-loading"
                        } else {
                            "settings-action"
                        },
                        disabled: !can_test || testing_model,
                        onclick: move |_| {
                            if *is_testing_model.read() {
                                return;
                            }
                            let endpoint = llm_endpoint.read().trim().to_string();
                            let api_key = llm_api_key.read().trim().to_string();
                            let model = llm_model.read().trim().to_string();
                            if endpoint.is_empty() || api_key.is_empty() || model.is_empty() {
                                *toast.write() = Some(ToastMessage {
                                    kind: ToastKind::Error,
                                    text: "请先填写 Endpoint、API Key 并选择模型。".to_string(),
                                });
                                return;
                            }
                            *is_testing_model.write() = true;
                            let mut settings_status = settings_status.clone();
                            let mut toast = toast.clone();
                            let mut is_testing_model = is_testing_model.clone();
                            spawn(async move {
                                *settings_status.write() = Some("正在测试模型...".to_string());
                                match general_ai_client::test_chat(
                                    &endpoint,
                                    &api_key,
                                    &model,
                                    "hello, I am umbreon",
                                )
                                .await
                                {
                                    Ok(_) => {
                                        *settings_status.write() = Some("模型测试成功。".to_string());
                                        *toast.write() = Some(ToastMessage {
                                            kind: ToastKind::Success,
                                            text: "测试成功。".to_string(),
                                        });
                                    }
                                    Err(err) => {
                                        *settings_status.write() = Some("模型测试失败。".to_string());
                                        *toast.write() = Some(ToastMessage {
                                            kind: ToastKind::Error,
                                            text: format!("测试失败：{err}"),
                                        });
                                    }
                                }
                                *is_testing_model.write() = false;
                            });
                        },
                        span { class: "material-icons", "science" }
                        span { if testing_model { "测试中" } else { "测试" } }
                    }
                }
            }
            div { class: "settings-field",
                div { class: "settings-row settings-row-spread",
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
            }
            if let Some(message) = settings_status.read().clone() {
                p { class: "settings-status", "{message}" }
            }
        }
    }
}
