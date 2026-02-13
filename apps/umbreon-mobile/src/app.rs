use crate::components::{ExplorePane, MemoryPane, NavigationBar};
use crate::settings::SettingsPane;
use crate::state::{self, AppContext, NavSection, ThemeMode};
use crate::storage;
use crate::style::BASE_STYLES;
use crate::timeline;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn AppRoot() -> Element {
    let stored_settings = storage::load_settings();
    let initial_theme = stored_settings.theme.unwrap_or(ThemeMode::Light);
    let initial_memory_server_url = stored_settings
        .memory_server_url
        .as_deref()
        .unwrap_or("http://localhost:8787")
        .to_string();
    let initial_llm_endpoint = stored_settings
        .llm_endpoint
        .as_deref()
        .unwrap_or("https://api.openai.com/v1")
        .to_string();
    let initial_llm_api_key = stored_settings
        .llm_api_key
        .as_ref()
        .cloned()
        .unwrap_or_default();
    let initial_llm_model = stored_settings
        .llm_model
        .as_ref()
        .cloned()
        .unwrap_or_default();
    let initial_llm_models = stored_settings
        .llm_models
        .as_ref()
        .cloned()
        .unwrap_or_default();

    let feed_bootstrap = timeline::init_feed_bootstrap(&stored_settings);
    let should_auto_sync_stale_cache = feed_bootstrap.should_auto_sync_stale_cache;
    let initial_feed_items = feed_bootstrap.feed_items;
    let initial_feed_server_url = feed_bootstrap.feed_server_url;

    let nav = use_signal(|| NavSection::Explore);
    let theme = use_signal(|| initial_theme);
    let feed_items = use_signal(|| initial_feed_items);
    let live_streams = use_signal(state::mock_live_streams);
    let now_playing = use_signal(state::mock_initial_session);
    let memory_panel = use_signal(state::mock_memory_panel);
    let feed_server_url = use_signal(|| initial_feed_server_url);
    let memory_server_url = use_signal(|| initial_memory_server_url);
    let llm_endpoint = use_signal(|| initial_llm_endpoint);
    let llm_api_key = use_signal(|| initial_llm_api_key);
    let llm_model = use_signal(|| initial_llm_model);
    let llm_models = use_signal(|| initial_llm_models);
    let settings_status = use_signal(|| None::<String>);
    let toast = use_signal(|| None::<state::ToastMessage>);

    let ctx_seed = AppContext {
        nav,
        theme,
        feed_items,
        live_streams,
        now_playing,
        memory_panel,
        feed_server_url,
        memory_server_url,
        llm_endpoint,
        llm_api_key,
        llm_model,
        llm_models,
        settings_status,
        toast,
    };

    let ctx = use_context_provider({
        let provided = ctx_seed.clone();
        move || provided
    });

    let active_nav = *ctx.nav.read();
    let theme_class = match *ctx.theme.read() {
        ThemeMode::Dark => "theme-dark",
        ThemeMode::Light => "theme-light",
    };
    let header_title = match active_nav {
        NavSection::Dialogue => "对话",
        NavSection::Explore => "探索",
        NavSection::Memory => "记忆",
        NavSection::Settings => "设置",
    };

    let feed_server_url = ctx.feed_server_url;
    let settings_status = ctx.settings_status;
    let feed_items = ctx.feed_items;
    let mut auto_sync_once = use_signal(|| false);

    use_effect(move || {
        if *auto_sync_once.read() || !should_auto_sync_stale_cache {
            return;
        }
        *auto_sync_once.write() = true;
        let url = feed_server_url.read().trim().to_string();
        timeline::trigger_feed_sync(url, feed_items.clone(), settings_status.clone());
    });

    let toast_class = ctx
        .toast
        .read()
        .as_ref()
        .map(|toast| match toast.kind {
            state::ToastKind::Success => "toast toast-success",
            state::ToastKind::Error => "toast toast-error",
        })
        .unwrap_or("toast");
    let mut toast_signal = ctx.toast;

    rsx! {
        link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/icon?family=Material+Icons"
        }
        style { "{BASE_STYLES}" }
        main { class: "umbreon-shell {theme_class}",
            header { class: "umbreon-header", "{header_title}" }
            section { class: "umbreon-content",
                match active_nav {
                    NavSection::Dialogue => rsx!(
                        div { class: "settings-pane",
                            h2 { "对话" }
                            p { "对话列表还在路上。" }
                        }
                    ),
                    NavSection::Explore => rsx!(ExplorePane {}),
                    NavSection::Memory => rsx!(MemoryPane {}),
                    NavSection::Settings => rsx!(SettingsPane {}),
                }
            }
            NavigationBar {}
            if let Some(toast) = ctx.toast.read().clone() {
                div { class: "{toast_class}",
                    span { "{toast.text}" }
                    button {
                        class: "toast-close",
                        onclick: move |_| {
                            *toast_signal.write() = None;
                        },
                        span { class: "material-icons", "close" }
                    }
                }
            }
        }
    }
}
