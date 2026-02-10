use crate::components::{MemoryPane, NavigationBar, NowPlayingPane, TimelinePane};
use crate::state::{self, AppContext, FeedItem, FeedSourceKind, NavSection, ThemeMode};
use dioxus::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

const BASE_STYLES: &str = r#"
@import url("https://fonts.googleapis.com/icon?family=Material+Icons");

:root {
  font-family: "Roboto", "Inter", system-ui, -apple-system, sans-serif;
  letter-spacing: 0.2px;
}

.umbreon-shell {
  min-height: 100vh;
  display: flex;
  background: var(--md-sys-color-background);
  color: var(--md-sys-color-on-background);
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
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.theme-toggle {
  padding: 12px 16px;
  border-radius: 14px;
  border: 1px solid var(--md-sys-color-outline-variant);
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.umbreon-sidebar.collapsed .theme-toggle {
  justify-content: center;
}

.umbreon-content {
  flex: 1;
  padding: 32px;
  display: flex;
  flex-direction: column;
  gap: 24px;
  min-width: 0;
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
}

.feed-card {
  display: flex;
  gap: 14px;
  padding: 16px 18px;
  border-bottom: 1px solid var(--md-sys-color-outline-variant);
  background: var(--md-sys-color-surface);
  cursor: pointer;
  transition: background 0.2s ease;
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
}

.post-header {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
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
  line-height: 1.5;
}

.post-text p {
  margin: 0 0 8px;
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
}

.feed-modal {
  width: min(720px, 92vw);
  background: var(--md-sys-color-surface);
  border-radius: 20px;
  padding: 24px;
  border: 1px solid var(--md-sys-color-outline-variant);
  box-shadow: 0 16px 40px rgba(0, 0, 0, 0.25);
  display: flex;
  flex-direction: column;
  gap: 16px;
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

async fn load_feeds_from_gist(url: &str) -> Result<Vec<FeedItem>, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|err| format!("failed to load gist: {err}"))?
        .text()
        .await
        .map_err(|err| format!("failed to read gist: {err}"))?;
    let config: GistConfig = toml::from_str(&response)
        .map_err(|err| format!("invalid toml: {err}"))?;
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
            let published_at = entry
                .published
                .or(entry.updated)
                .map(|value| value.to_string())
                .unwrap_or_else(|| "unknown".to_string());
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
                link,
                author: author.clone(),
                avatar_url: avatar_url.clone(),
            });
        }
    }

    if items.is_empty() {
        return Err("no feed entries found".to_string());
    }

    Ok(items)
}

#[allow(non_snake_case)]
pub fn AppRoot() -> Element {
    let nav = use_signal(|| NavSection::Timeline);
    let theme = use_signal(|| ThemeMode::Dark);
    let sidebar_collapsed = use_signal(|| false);
    let feed_items = use_signal(state::mock_feed_items);
    let live_streams = use_signal(state::mock_live_streams);
    let now_playing = use_signal(state::mock_initial_session);
    let memory_panel = use_signal(state::mock_memory_panel);
    let gist_url = use_signal(String::new);
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
    let mut settings_status = ctx.settings_status;
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
                            h2 { "Settings" }
                            p { "Sync feeds from a remote Gist (TOML)." }
                            div { class: "settings-field",
                                label { class: "settings-label", "Gist URL" }
                                input {
                                    class: "settings-input",
                                    r#type: "url",
                                    placeholder: "https://gist.githubusercontent.com/.../config.toml",
                                    value: "{gist_url.read()}",
                                    oninput: move |evt| {
                                        *gist_url.write() = evt.value();
                                    }
                                }
                            }
                            button {
                                class: "settings-sync",
                                onclick: move |_| {
                                    let url = gist_url.read().trim().to_string();
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
                                                *feed_items.write() = items;
                                                *settings_status.write() = Some("Feeds updated.".to_string());
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
