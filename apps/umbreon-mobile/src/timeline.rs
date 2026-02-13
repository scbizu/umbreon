use crate::helper;
use crate::state::{self, FeedItem, FeedSourceKind};
use crate::storage;
use dioxus::prelude::*;
use feed_rs::model::FeedType;

pub struct FeedBootstrap {
    pub feed_items: Vec<FeedItem>,
    pub feed_server_url: String,
    pub should_auto_sync_stale_cache: bool,
}

pub fn init_feed_bootstrap(stored_settings: &storage::StoredSettings) -> FeedBootstrap {
    let initial_feed_server_url = stored_settings
        .feed_server_url
        .clone()
        .unwrap_or_else(|| "https://feed-aggregator-worker.scnace.workers.dev".to_string());
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

    FeedBootstrap {
        feed_items: initial_feed_items,
        feed_server_url: initial_feed_server_url,
        should_auto_sync_stale_cache,
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
            .replacen(r#"version=\"1.0\""#, r#"version=\"2.0\""#, 1)
            .replacen("version='1.0'", "version='2.0'", 1);
        return feed_rs::parser::parse(normalized.as_bytes())
            .map_err(|err| format!("unable to parse feed: {err}"));
    }

    Err("unable to parse feed: no xml content".to_string())
}

pub async fn load_feeds_from_server(url: &str) -> Result<Vec<FeedItem>, String> {
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
            helper::parse_timestamp_for_feed(&feed_type, &published_at).ok_or_else(|| {
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

pub fn trigger_feed_sync(
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
