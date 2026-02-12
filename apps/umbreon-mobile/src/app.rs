use crate::components::{MemoryPane, NavigationBar, NowPlayingPane, TimelinePane};
use crate::state::{self, AppContext, FeedItem, FeedSourceKind, NavSection, ThemeMode};
use crate::storage;
use chrono::{DateTime, NaiveDateTime, Utc};
use dioxus::prelude::*;
use feed_rs::model::FeedType;
use serde::Deserialize;
use std::collections::HashMap;

const BASE_STYLES: &str = r#"
@import url("https://fonts.googleapis.com/icon?family=Material+Icons");

:root {
  font-family: "Roboto", "Inter", system-ui, -apple-system, sans-serif;
  letter-spacing: 0.2px;
}

.umbreon-shell {
  height: 100vh;
  display: flex;
  background: var(--md-sys-color-background);
  color: var(--md-sys-color-on-background);
  overflow: hidden;
}

.umbreon-shell.theme-dark {
  --md-sys-color-background: #101418;
  --md-sys-color-surface: #171b20;
  --md-sys-color-surface-container: #1f242a;
  --md-sys-color-surface-container-high: #252a31;
  --md-sys-color-outline: #3b414a;
  --md-sys-color-outline-variant: #2b3138;
  --md-sys-color-primary: #9ec3ff;
  --md-sys-color-on-primary: #0f1d35;
  --md-sys-color-secondary: #c7d2e4;
  --md-sys-color-on-surface: #e2e6ed;
  --md-sys-color-on-surface-variant: #b6bdc8;
}

.umbreon-shell.theme-light {
  --md-sys-color-background: #f8f9fc;
  --md-sys-color-surface: #ffffff;
  --md-sys-color-surface-container: #f1f3f8;
  --md-sys-color-surface-container-high: #e8ecf3;
  --md-sys-color-outline: #c4c7cf;
  --md-sys-color-outline-variant: #e2e6ee;
  --md-sys-color-primary: #2f5cc8;
  --md-sys-color-on-primary: #ffffff;
  --md-sys-color-secondary: #566071;
  --md-sys-color-on-surface: #1b1f24;
  --md-sys-color-on-surface-variant: #4f5b6b;
}

.umbreon-sidebar {
  width: 252px;
  padding: 24px 16px;
  background: var(--md-sys-color-surface);
  border-right: 1px solid var(--md-sys-color-outline-variant);
  display: flex;
  flex-direction: column;
  gap: 20px;
  transition: width 0.2s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  flex: 0 0 auto;
  height: 100vh;
  max-height: 100vh;
  overflow: hidden;
}

.umbreon-sidebar.collapsed {
  width: 72px;
  padding: 20px 8px;
}

.umbreon-brand-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.umbreon-brand {
  font-size: 20px;
  font-weight: 600;
}

.umbreon-sidebar.collapsed .umbreon-brand-text,
.umbreon-sidebar.collapsed .nav-label,
.umbreon-sidebar.collapsed .theme-toggle-text {
  display: none;
}

.collapse-toggle {
  border: none;
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  border-radius: 12px;
  width: 40px;
  height: 40px;
  cursor: pointer;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12);
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.collapse-toggle .material-icons {
  font-size: 20px;
  line-height: 1;
}

.umbreon-nav {
  display: flex;
  flex-direction: column;
  gap: 10px;
  flex: 1;
  overflow: auto;
}

.nav-btn {
  padding: 12px 16px;
  border-radius: 14px;
  border: 1px solid transparent;
  background: transparent;
  color: var(--md-sys-color-on-surface);
  text-align: left;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: background 0.2s ease, border-color 0.2s ease, color 0.2s ease;
}

.nav-icon {
  width: 22px;
  text-align: center;
  font-size: 16px;
  line-height: 1;
}

.nav-icon.material-icons {
  font-size: 20px;
}

.nav-btn.active {
  background: var(--md-sys-color-surface-container-high);
  border-color: var(--md-sys-color-outline-variant);
  color: var(--md-sys-color-primary);
}

.nav-btn:hover {
  background: var(--md-sys-color-surface-container);
}

.sidebar-footer {
  margin-top: auto;
  margin-bottom: 32px;
  display: flex;
}

.sidebar-footer .nav-btn {
  width: 100%;
}

.theme-toggle {
  padding: 12px 16px;
  border-radius: 14px;
  border: 1px solid var(--md-sys-color-outline-variant);
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
}

.theme-icon {
  font-size: 18px;
}

.theme-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
}

.theme-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.theme-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--md-sys-color-outline-variant);
  border-radius: 999px;
  transition: background 0.2s ease;
}

.theme-slider::before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  top: 3px;
  background: var(--md-sys-color-surface);
  border-radius: 50%;
  transition: transform 0.2s ease;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
}

.theme-switch input:checked + .theme-slider {
  background: var(--md-sys-color-primary);
}

.theme-switch input:checked + .theme-slider::before {
  transform: translateX(20px);
}

.umbreon-content {
  flex: 1;
  padding: 32px;
  display: flex;
  flex-direction: column;
  gap: 24px;
  min-width: 0;
  height: 100vh;
  overflow: hidden;
}

.now-playing .stream,
.live-stream-list .url {
  color: var(--md-sys-color-on-surface-variant);
  word-break: break-all;
}

.now-playing,
.live-stream-list,
.memory-pane,
.player-pane,
.settings-pane {
  background: var(--md-sys-color-surface);
  border: 1px solid var(--md-sys-color-outline-variant);
  border-radius: 18px;
  padding: 18px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.08);
}

.settings-pane {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.settings-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.settings-label {
  font-size: 13px;
  color: var(--md-sys-color-on-surface-variant);
}

.settings-input {
  padding: 12px 14px;
  border-radius: 12px;
  border: 1px solid var(--md-sys-color-outline-variant);
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
}

.settings-theme-toggle {
  justify-content: flex-start;
}

.settings-sync {
  border: none;
  border-radius: 12px;
  padding: 10px 14px;
  background: var(--md-sys-color-primary);
  color: var(--md-sys-color-on-primary);
  display: inline-flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  width: fit-content;
}

.settings-status {
  font-size: 13px;
  color: var(--md-sys-color-on-surface-variant);
}

.timeline-pane {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding-right: 6px;
  width: 100%;
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.feed-card {
  display: flex;
  align-items: flex-start;
  gap: 14px;
  padding: 16px 18px;
  border-bottom: 1px solid var(--md-sys-color-outline-variant);
  background: var(--md-sys-color-surface);
  cursor: pointer;
  transition: background 0.2s ease;
  width: 100%;
}

.feed-card:hover {
  background: var(--md-sys-color-surface-container);
}

.feed-card:last-child {
  border-bottom: none;
}

.post-avatar {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background: var(--md-sys-color-surface-container-high);
  color: var(--md-sys-color-on-surface);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  overflow: hidden;
}

.post-avatar-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.post-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 6px;
  min-width: 0;
  max-width: 100%;
  width: 100%;
}

.post-header {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  flex-wrap: wrap;
}

.post-name {
  font-weight: 600;
}

.post-handle,
.post-time,
.post-dot {
  color: var(--md-sys-color-on-surface-variant);
}

.post-text {
  color: var(--md-sys-color-on-surface);
  line-height: 1.6;
  max-width: 100%;
  max-height: calc(1.6em * 30);
  overflow: hidden;
  word-break: break-word;
  overflow-wrap: anywhere;
  display: block;
  position: relative;
}

.post-text::after {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  height: 32px;
  background: linear-gradient(to bottom, rgba(0, 0, 0, 0), var(--md-sys-color-surface));
  pointer-events: none;
}

.post-text * {
  max-width: 100%;
}

.post-text p {
  margin: 0 0 8px;
}

.post-text a {
  word-break: break-all;
}

.post-text pre {
  margin: 8px 0;
  padding: 10px 12px;
  border-radius: 10px;
  background: var(--md-sys-color-surface-container);
  overflow-x: auto;
  font-size: 12px;
}

.post-title {
  margin: 0;
  color: var(--md-sys-color-on-surface-variant);
}

.post-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 12px;
  color: var(--md-sys-color-secondary);
}

.post-link {
  color: var(--md-sys-color-primary);
}

.post-actions {
  display: flex;
  gap: 12px;
  margin-top: 6px;
}

.post-action {
  border: none;
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  cursor: pointer;
  font-size: 14px;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 12px;
  text-decoration: none;
}

.post-action:hover {
  background: var(--md-sys-color-surface-container-high);
}

.material-icons {
  font-family: 'Material Icons';
  font-weight: normal;
  font-style: normal;
  font-size: 18px;
  line-height: 1;
  display: inline-block;
}

.feed-modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(15, 20, 28, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  z-index: 1000;
  overflow: auto;
}

.feed-modal {
  width: min(720px, 92vw);
  max-height: calc(100vh - 48px);
  background: var(--md-sys-color-surface);
  border-radius: 20px;
  padding: 24px;
  border: 1px solid var(--md-sys-color-outline-variant);
  box-shadow: 0 16px 40px rgba(0, 0, 0, 0.25);
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow: auto;
}

.feed-modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.feed-modal-close {
  border: none;
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  border-radius: 12px;
  padding: 8px 12px;
  cursor: pointer;
}

.feed-modal-summary {
  margin: 0;
  color: var(--md-sys-color-on-surface-variant);
  line-height: 1.6;
}

.feed-modal-meta {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 13px;
  color: var(--md-sys-color-on-surface-variant);
}

.feed-modal-link {
  color: var(--md-sys-color-primary);
}

.empty-state {
  color: var(--md-sys-color-on-surface-variant);
}

.timeline-footer {
  padding: 8px 0 4px;
  font-size: 12px;
  color: var(--md-sys-color-on-surface-variant);
  text-align: center;
}

.timeline-spacer {
  height: 0;
}
"#;

#[derive(Debug, Deserialize)]
struct GistConfig {
    feeds: Option<HashMap<String, FeedConfig>>,
}

#[derive(Debug, Deserialize)]
struct FeedConfig {
    name: Option<String>,
    url: String,
}

fn parse_timestamp_atom(value: &str) -> Option<i64> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
        return Some(dt.timestamp());
    }
    if let Ok(dt) = DateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S %Z") {
        return Some(dt.timestamp());
    }
    if let Some(replaced) = value.strip_suffix(" UTC") {
        let patched = format!("{} +0000", replaced);
        if let Ok(dt) = DateTime::parse_from_str(&patched, "%Y-%m-%d %H:%M:%S %z") {
            return Some(dt.timestamp());
        }
    }
    if let Some(replaced) = value.strip_suffix(" GMT") {
        let patched = format!("{} +0000", replaced);
        if let Ok(dt) = DateTime::parse_from_str(&patched, "%Y-%m-%d %H:%M:%S %z") {
            return Some(dt.timestamp());
        }
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S") {
        return Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc).timestamp());
    }
    None
}

fn parse_timestamp_rss(value: &str) -> Option<i64> {
    if let Ok(dt) = DateTime::parse_from_rfc2822(value) {
        return Some(dt.timestamp());
    }
    parse_timestamp_atom(value)
}

fn parse_timestamp_for_feed(feed_type: &FeedType, value: &str) -> Option<i64> {
    match feed_type {
        FeedType::Atom => parse_timestamp_atom(value),
        FeedType::RSS2 => parse_timestamp_rss(value),
        _ => parse_timestamp_atom(value),
    }
}

async fn load_feeds_from_gist(url: &str) -> Result<Vec<FeedItem>, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|err| format!("failed to load gist: {err}"))?
        .text()
        .await
        .map_err(|err| format!("failed to read gist: {err}"))?;
    let config: GistConfig =
        toml::from_str(&response).map_err(|err| format!("invalid toml: {err}"))?;
    let feeds = config
        .feeds
        .ok_or_else(|| "no [feeds] section found".to_string())?;

    let mut items = Vec::new();

    for (key, feed) in feeds {
        if feed.url.trim().is_empty() {
            continue;
        }
        let feed_text = reqwest::get(&feed.url)
            .await
            .map_err(|err| format!("failed to load feed {}: {err}", feed.url))?
            .text()
            .await
            .map_err(|err| format!("failed to read feed {}: {err}", feed.url))?;
        let parsed = feed_rs::parser::parse(feed_text.as_bytes())
            .map_err(|err| format!("failed to parse feed {}: {err}", feed.url))?;
        let avatar_url = parsed
            .logo
            .clone()
            .or(parsed.icon.clone())
            .map(|image| image.uri);
        let author = feed.name.clone().unwrap_or_else(|| key.clone());
        let feed_type = parsed.feed_type;

        for entry in parsed.entries {
            let title = entry
                .title
                .as_ref()
                .map(|value| value.content.clone())
                .unwrap_or_default();
            let summary = entry
                .content
                .as_ref()
                .and_then(|value| value.body.clone())
                .or_else(|| entry.summary.as_ref().map(|value| value.content.clone()))
                .unwrap_or_else(|| title.clone());
            let summary = ammonia::Builder::default()
                .add_tags(["pre", "code", "p", "br"])
                .clean(&summary)
                .to_string();
            let published_at: String = entry
                .published
                .or(entry.updated)
                .map(|value| value.to_string())
                .ok_or_else(|| {
                    format!(
                        "missing published/updated date in feed {} entry {}",
                        feed.url,
                        if entry.id.is_empty() {
                            title.clone()
                        } else {
                            entry.id.clone()
                        }
                    )
                })?;
            let published_ts =
                parse_timestamp_for_feed(&feed_type, &published_at).ok_or_else(|| {
                    format!(
                        "invalid date '{}' in feed {} entry {}",
                        published_at,
                        feed.url,
                        if entry.id.is_empty() {
                            title.clone()
                        } else {
                            entry.id.clone()
                        }
                    )
                })?;
            let link = entry
                .links
                .first()
                .map(|link| link.href.clone())
                .unwrap_or_default();
            let suffix = items.len();
            let id = if entry.id.is_empty() {
                format!("{key}-{suffix}")
            } else {
                entry.id.clone()
            };

            items.push(FeedItem {
                id,
                title,
                summary,
                source: FeedSourceKind::Atom,
                published_at,
                published_ts,
                link,
                author: author.clone(),
                avatar_url: avatar_url.clone(),
            });
        }
    }

    if items.is_empty() {
        return Err("no feed entries found".to_string());
    }

    items.sort_by(|a, b| b.published_ts.cmp(&a.published_ts));

    Ok(items)
}

#[allow(non_snake_case)]
pub fn AppRoot() -> Element {
    let stored_settings = storage::load_settings();
    let initial_theme = stored_settings.theme.unwrap_or(ThemeMode::Light);
    let initial_gist_url = stored_settings.gist_url.unwrap_or_else(|| {
        "https://gist.githubusercontent.com/scbizu/2fea15bd4748c057f01ccec8c2ca2990/raw/66f246525ebafb0bca79c7253e6d45a19eb62e9e/umbreon_app_settings.toml".to_string()
    });
    let initial_memory_server_url = stored_settings
        .memory_server_url
        .unwrap_or_else(|| "http://localhost:8787".to_string());
    let initial_feed_items = {
        let cached = storage::load_feed_items();
        if cached.is_empty() {
            state::mock_feed_items()
        } else {
            cached
        }
    };

    let nav = use_signal(|| NavSection::Timeline);
    let theme = use_signal(|| initial_theme);
    let sidebar_collapsed = use_signal(|| false);
    let feed_items = use_signal(|| initial_feed_items);
    let live_streams = use_signal(state::mock_live_streams);
    let now_playing = use_signal(state::mock_initial_session);
    let memory_panel = use_signal(state::mock_memory_panel);
    let gist_url = use_signal(|| initial_gist_url);
    let memory_server_url = use_signal(|| initial_memory_server_url);
    let settings_status = use_signal(|| None::<String>);

    let ctx_seed = AppContext {
        nav,
        theme,
        sidebar_collapsed,
        feed_items,
        live_streams,
        now_playing,
        memory_panel,
        gist_url,
        memory_server_url,
        settings_status,
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

    let mut gist_url = ctx.gist_url;
    let mut memory_server_url = ctx.memory_server_url;
    let mut settings_status = ctx.settings_status;
    let mut theme = ctx.theme;
    let mode = *theme.read();
    let feed_items = ctx.feed_items;

    rsx! {
        link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/icon?family=Material+Icons"
        }
        style { "{BASE_STYLES}" }
        main { class: "umbreon-shell {theme_class}",
            NavigationBar {}
            section { class: "umbreon-content",
                match active_nav {
                    NavSection::Timeline => rsx!(TimelinePane {}),
                    NavSection::Live => rsx!(NowPlayingPane { mode: state::MediaKind::Live }),
                    NavSection::Vod => rsx!(NowPlayingPane { mode: state::MediaKind::Vod }),
                    NavSection::Memory => rsx!(MemoryPane {}),
                    NavSection::Settings => rsx!(
                        div { class: "settings-pane",
                            h2 { "设置" }
                            p { "Sync feeds from a remote Gist (TOML)." }
                            div { class: "settings-field",
                                label { class: "settings-label", "Gist URL" }
                                input {
                                    class: "settings-input",
                                    r#type: "url",
                                    placeholder: "https://gist.githubusercontent.com/.../config.toml",
                                    value: "{gist_url.read()}",
                                    oninput: move |evt| {
                                        let value = evt.value();
                                        *gist_url.write() = value.clone();
                                        storage::store_gist_url(&value);
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
                            button {
                                class: "settings-sync",
                                onclick: move |_| {
                                    let url = gist_url.read().trim().to_string();
                                    let memory_url = memory_server_url.read().trim().to_string();
                                    storage::store_gist_url(&url);
                                    storage::store_memory_server_url(&memory_url);
                                    if url.is_empty() {
                                        *settings_status.write() = Some("Please enter a Gist URL.".to_string());
                                        return;
                                    }
                                    *settings_status.write() = Some("Syncing feeds...".to_string());
                                    let mut feed_items = feed_items.clone();
                                    let mut settings_status = settings_status.clone();
                                    spawn(async move {
                                        match load_feeds_from_gist(&url).await {
                                            Ok(items) => {
                                                let mut status = "Feeds updated.".to_string();
                                                if let Err(err) = storage::store_feed_items(&items) {
                                                    status = format!("Feeds updated, but cache failed: {err}");
                                                }
                                                *feed_items.write() = items;
                                                *settings_status.write() = Some(status);
                                            }
                                            Err(err) => {
                                                *settings_status.write() = Some(err);
                                            }
                                        }
                                    });
                                },
                                span { class: "material-icons", "sync" }
                                span { "Sync feeds" }
                            }
                            if let Some(message) = settings_status.read().clone() {
                                p { class: "settings-status", "{message}" }
                            }
                        }
                    ),
                }
            }
        }
    }
}
