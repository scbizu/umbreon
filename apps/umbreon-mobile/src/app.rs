use crate::components::{ExplorePane, MemoryPane, NavigationBar};
use crate::state::{self, AppContext, FeedItem, FeedSourceKind, NavSection, ThemeMode};
use crate::storage;
use chrono::{DateTime, NaiveDateTime, Utc};
use dioxus::prelude::*;
use feed_rs::model::FeedType;

const BASE_STYLES: &str = r#"
@import url("https://fonts.googleapis.com/icon?family=Material+Icons");

:root {
  font-family: "Roboto", "Inter", system-ui, -apple-system, sans-serif;
  letter-spacing: 0.2px;
}

.umbreon-shell {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--md-sys-color-background);
  color: var(--md-sys-color-on-background);
  overflow: hidden;
}

.umbreon-header {
  padding: 16px 18px 10px;
  font-size: 20px;
  font-weight: 600;
  text-align: center;
  background: var(--md-sys-color-background);
  position: sticky;
  top: 0;
  z-index: 10;
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
  --md-sys-color-background: #f2f2f7;
  --md-sys-color-surface: #ffffff;
  --md-sys-color-surface-container: #eef0f4;
  --md-sys-color-surface-container-high: #e7ebf3;
  --md-sys-color-outline: #d7dbe4;
  --md-sys-color-outline-variant: #e6e9f0;
  --md-sys-color-primary: #2b63ff;
  --md-sys-color-on-primary: #ffffff;
  --md-sys-color-secondary: #657189;
  --md-sys-color-on-surface: #1c1c1e;
  --md-sys-color-on-surface-variant: #6b7280;
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
  padding: 8px 16px 88px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-width: 0;
  overflow: hidden;
  background: var(--md-sys-color-background);
}

.bottom-nav {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--md-sys-color-surface);
  border-top: 1px solid var(--md-sys-color-outline-variant);
  display: flex;
  align-items: center;
  justify-content: space-around;
  padding: 8px 12px 18px;
  gap: 8px;
  z-index: 20;
  box-shadow: 0 -8px 20px rgba(0, 0, 0, 0.08);
  backdrop-filter: blur(18px);
}

.umbreon-shell.theme-dark .bottom-nav {
  background: rgba(23, 27, 32, 0.95);
}

.bottom-nav-btn {
  border: none;
  background: transparent;
  color: var(--md-sys-color-on-surface-variant);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  padding: 8px 12px;
  border-radius: 14px;
  min-width: 64px;
  cursor: pointer;
  transition: background 0.2s ease, color 0.2s ease;
}

.bottom-nav-btn .material-icons {
  font-size: 22px;
}

.bottom-nav-btn.active {
  background: rgba(43, 99, 255, 0.14);
  color: var(--md-sys-color-primary);
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
  gap: 14px;
  width: 100%;
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  min-height: 0;
}

.explore-pane {
  display: flex;
  flex-direction: column;
  gap: 12px;
  flex: 1;
  min-height: 0;
}

.explore-subheader {
  display: flex;
  align-items: center;
  gap: 8px;
}

.explore-back {
  border: none;
  background: transparent;
  color: var(--md-sys-color-on-surface-variant);
  display: inline-flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  font-size: 14px;
}

.explore-card {
  background: var(--md-sys-color-surface);
  border: 1px solid var(--md-sys-color-outline-variant);
  border-radius: 18px;
  padding: 6px;
  box-shadow: 0 10px 24px rgba(0, 0, 0, 0.08);
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.explore-item {
  border: none;
  background: transparent;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 12px;
  border-radius: 16px;
  cursor: pointer;
  font-size: 16px;
  color: var(--md-sys-color-on-surface);
}

.explore-item:hover {
  background: var(--md-sys-color-surface-container);
}

.explore-icon {
  width: 44px;
  height: 44px;
  border-radius: 14px;
  background: var(--md-sys-color-surface-container);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--md-sys-color-on-surface-variant);
  font-size: 22px;
  line-height: 44px;
}

.explore-icon.material-icons {
  line-height: 44px;
}

.explore-label {
  font-weight: 600;
}

.explore-chevron {
  margin-left: auto;
  color: var(--md-sys-color-on-surface-variant);
}

.timeline-pane > * {
  flex-shrink: 0;
}

.feed-card {
  position: relative;
  display: flex;
  align-items: flex-start;
  gap: 14px;
  padding: 18px 16px;
  border-radius: 18px;
  border: 1px solid var(--md-sys-color-outline-variant);
  background-color: var(--md-sys-color-surface);
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  width: 100%;
  max-height: 25vh;
  overflow: hidden;
  box-sizing: border-box;
  box-shadow: 0 12px 28px rgba(0, 0, 0, 0.08);
}

.feed-card .post-body {
  max-height: calc(25vh - 36px);
  overflow: hidden;
}

.feed-card .post-text {
  max-height: calc(25vh - 140px);
  overflow: hidden;
}

.feed-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 16px 32px rgba(0, 0, 0, 0.12);
}

.feed-card--marked {
  border-color: #f4a6c5;
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

.post-avatar-fallback {
  font-size: 18px;
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

.feed-lang-badge {
  margin-left: auto;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 999px;
  background: rgba(158, 195, 255, 0.18);
  color: var(--md-sys-color-primary);
  font-size: 12px;
  font-weight: 700;
}

.feed-lang-icon {
  width: 14px;
  height: 14px;
}

.feed-lang-label {
  text-transform: uppercase;
  letter-spacing: 0.3px;
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
  display: none;
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
  overflow-x: hidden;
  white-space: pre-wrap;
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

.post-action--marked {
  background: #f7c5d9;
  color: #a4004c;
}

.post-action--marked:hover {
  background: #f4b6cf;
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
    if let Ok(date) = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        let dt = date.and_hms_opt(0, 0, 0)?;
        return Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc).timestamp());
    }
    if let Some(replaced) = value.strip_suffix(" UTC") {
        let patched = format!("{} +0000", replaced);
        if let Ok(dt) = DateTime::parse_from_str(&patched, "%Y-%m-%d %H:%M:%S %z") {
            return Some(dt.timestamp());
        }
    }
    if let Ok(dt) = DateTime::parse_from_str(value, "%a, %d %b %Y %H:%M:%S %Z") {
        return Some(dt.timestamp());
    }
    if let Ok(dt) = DateTime::parse_from_str(value, "%a, %d %b %Y %H:%M %Z") {
        return Some(dt.timestamp());
    }
    None
}

fn parse_timestamp_for_feed(feed_type: &FeedType, value: &str) -> Option<i64> {
    match feed_type {
        FeedType::Atom => parse_timestamp_atom(value),
        FeedType::RSS1 => parse_timestamp_rss(value),
        FeedType::RSS2 => parse_timestamp_rss(value),
        _ => parse_timestamp_atom(value),
    }
}

fn parse_feed_with_fallback(feed_bytes: &[u8]) -> Result<feed_rs::model::Feed, String> {
    if let Ok(feed) = feed_rs::parser::parse(feed_bytes) {
        return Ok(feed);
    }

    let trimmed_bom = feed_bytes
        .strip_prefix(&[0xEF, 0xBB, 0xBF])
        .unwrap_or(feed_bytes);
    if let Ok(feed) = feed_rs::parser::parse(trimmed_bom) {
        return Ok(feed);
    }

    if let Some(xml_start) = trimmed_bom.iter().position(|byte| *byte == b'<') {
        let xml_body = &trimmed_bom[xml_start..];
        if let Ok(feed) = feed_rs::parser::parse(xml_body) {
            return Ok(feed);
        }

        let xml_text = String::from_utf8_lossy(xml_body);
        let normalized = xml_text
            .replacen(r#"version="1.0""#, r#"version="2.0""#, 1)
            .replacen("version='1.0'", "version='2.0'", 1);
        return feed_rs::parser::parse(normalized.as_bytes())
            .map_err(|err| format!("unable to parse feed: {err}"));
    }

    Err("unable to parse feed: no xml content".to_string())
}

async fn load_feeds_from_server(url: &str) -> Result<Vec<FeedItem>, String> {
    let feed_bytes = reqwest::get(url)
        .await
        .map_err(|err| format!("failed to load feed server: {err}"))?
        .bytes()
        .await
        .map_err(|err| format!("failed to read feed server: {err}"))?;
    let parsed = parse_feed_with_fallback(feed_bytes.as_ref())
        .map_err(|err| format!("failed to parse feed server: {err}"))?;

    let avatar_url = parsed
        .logo
        .clone()
        .or(parsed.icon.clone())
        .map(|image| image.uri);
    let feed_title = parsed
        .title
        .as_ref()
        .map(|value| value.content.clone())
        .unwrap_or_else(|| "Feed Server".to_string());
    let feed_type = parsed.feed_type;
    let source = match feed_type {
        FeedType::Atom => FeedSourceKind::Atom,
        _ => FeedSourceKind::Custom,
    };

    let mut items = Vec::new();

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
            .rm_tags([
                "img",
                "picture",
                "source",
                "figure",
                "figcaption",
                "video",
                "audio",
                "iframe",
                "svg",
            ])
            .clean(&summary)
            .to_string();
        let published_at: String = entry
            .published
            .as_ref()
            .map(ToString::to_string)
            .or_else(|| entry.updated.as_ref().map(ToString::to_string))
            .ok_or_else(|| {
                format!(
                    "missing published/updated date in feed entry {}",
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
                    "invalid date '{}' in feed entry {}",
                    published_at,
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
            format!("feed-{suffix}")
        } else {
            entry.id.clone()
        };

        let mut tags = Vec::new();
        for category in &entry.categories {
            let term = category.term.trim();
            if !term.is_empty() {
                tags.push(term.to_string());
            }
        }
        tags.sort();
        tags.dedup();

        let author = entry
            .authors
            .first()
            .map(|author| author.name.clone())
            .unwrap_or_else(|| feed_title.clone());

        items.push(FeedItem {
            id,
            title,
            summary,
            source,
            published_at,
            published_ts,
            link,
            author,
            avatar_url: avatar_url.clone(),
            tags,
        });
    }

    if items.is_empty() {
        return Err("no feed entries found".to_string());
    }

    items.sort_by(|a, b| b.published_ts.cmp(&a.published_ts));

    Ok(items)
}

fn trigger_feed_sync(
    url: String,
    mut feed_items: Signal<Vec<FeedItem>>,
    mut settings_status: Signal<Option<String>>,
) {
    if url.is_empty() {
        *settings_status.write() = Some("Please enter a Feed Server URL.".to_string());
        return;
    }
    *settings_status.write() = Some("Syncing feeds...".to_string());
    spawn(async move {
        match load_feeds_from_server(&url).await {
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
}

#[allow(non_snake_case)]
pub fn AppRoot() -> Element {
    let stored_settings = storage::load_settings();
    let initial_theme = stored_settings.theme.unwrap_or(ThemeMode::Light);
    let initial_feed_server_url = stored_settings
        .feed_server_url
        .unwrap_or_else(|| "https://feed-aggregator-worker.scnace.workers.dev".to_string());
    let initial_memory_server_url = stored_settings
        .memory_server_url
        .unwrap_or_else(|| "http://localhost:8787".to_string());
    let cached_feed_items = storage::load_feed_items();
    let should_auto_sync_stale_cache = !cached_feed_items.is_empty()
        && !cached_feed_items.iter().any(|item| {
            item.tags
                .iter()
                .any(|tag| tag.trim_start().starts_with("StackLang:"))
        });
    let initial_feed_items = if cached_feed_items.is_empty() {
        state::mock_feed_items()
    } else {
        cached_feed_items
    };

    let nav = use_signal(|| NavSection::Explore);
    let theme = use_signal(|| initial_theme);
    let feed_items = use_signal(|| initial_feed_items);
    let live_streams = use_signal(state::mock_live_streams);
    let now_playing = use_signal(state::mock_initial_session);
    let memory_panel = use_signal(state::mock_memory_panel);
    let feed_server_url = use_signal(|| initial_feed_server_url);
    let memory_server_url = use_signal(|| initial_memory_server_url);
    let settings_status = use_signal(|| None::<String>);

    let ctx_seed = AppContext {
        nav,
        theme,
        feed_items,
        live_streams,
        now_playing,
        memory_panel,
        feed_server_url,
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
    let header_title = match active_nav {
        NavSection::Dialogue => "对话",
        NavSection::Explore => "探索",
        NavSection::Memory => "记忆",
        NavSection::Settings => "设置",
    };

    let mut feed_server_url = ctx.feed_server_url;
    let mut memory_server_url = ctx.memory_server_url;
    let settings_status = ctx.settings_status;
    let mut theme = ctx.theme;
    let mode = *theme.read();
    let feed_items = ctx.feed_items;
    let mut auto_sync_once = use_signal(|| false);

    use_effect(move || {
        if *auto_sync_once.read() || !should_auto_sync_stale_cache {
            return;
        }
        *auto_sync_once.write() = true;
        let url = feed_server_url.read().trim().to_string();
        trigger_feed_sync(url, feed_items.clone(), settings_status.clone());
    });

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
                    NavSection::Settings => rsx!(
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
                                        trigger_feed_sync(url, feed_items.clone(), settings_status.clone());
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
                    ),
                }
            }
            NavigationBar {}
        }
    }
}
