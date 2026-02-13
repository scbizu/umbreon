use crate::general_ai_client;
use crate::helper;
use crate::state::{self, FeedItem, FeedSourceKind};
use crate::storage;
use chrono::{FixedOffset, TimeZone};
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

fn plain_text_from_html(input: &str) -> String {
    let mut output = String::new();
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ => {
                if !in_tag {
                    output.push(ch);
                }
            }
        }
    }
    output
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

fn fallback_summary(text: &str) -> String {
    let max_len = 140usize;
    let mut trimmed = text.chars().take(max_len).collect::<String>();
    if text.chars().count() > max_len {
        trimmed.push('…');
    }
    trimmed
}

fn format_date_utc8(ts: i64) -> String {
    let offset = FixedOffset::east_opt(8 * 3600)
        .unwrap_or_else(|| FixedOffset::east_opt(0).expect("valid UTC offset"));
    offset
        .timestamp_opt(ts, 0)
        .single()
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "1970-01-01".to_string())
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
        let published_at = format_date_utc8(published_ts);
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
            summary: summary.clone(),
            full_content: summary,
            summarized: false,
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
    mut feed_syncing: Signal<bool>,
    llm_endpoint: Signal<String>,
    llm_api_key: Signal<String>,
    llm_model: Signal<String>,
) {
    if url.is_empty() {
        *settings_status.write() = Some("Please enter a Feed Server URL.".to_string());
        return;
    }
    *feed_syncing.write() = true;
    *settings_status.write() = Some("Syncing feeds...".to_string());
    spawn(async move {
        match load_feeds_from_server(&url).await {
            Ok(items) => {
                let endpoint = llm_endpoint.read().trim().to_string();
                let api_key = llm_api_key.read().trim().to_string();
                let model = llm_model.read().trim().to_string();
                let mut items = items;
                if !endpoint.is_empty() && !api_key.is_empty() && !model.is_empty() {
                    let total = items.len();
                    let mut done = 0usize;
                    let mut pending = Vec::new();
                    for (index, item) in items.iter_mut().enumerate() {
                        if item.summarized {
                            continue;
                        }
                        let body_text = plain_text_from_html(&item.full_content);
                        let fallback = if body_text.is_empty() {
                            fallback_summary(&item.summary)
                        } else {
                            fallback_summary(&body_text)
                        };
                        pending.push((index, body_text, fallback));
                    }

                    let mut offset = 0usize;
                    while offset < pending.len() {
                        let batch_end = (offset + 20).min(pending.len());
                        let chunk = &pending[offset..batch_end];
                        let mut chunk_offset = 0usize;
                        while chunk_offset < chunk.len() {
                            let group_end = (chunk_offset + 5).min(chunk.len());
                            let group = &chunk[chunk_offset..group_end];

                            let mut futures = Vec::with_capacity(group.len());
                            for (index, body_text, fallback) in group.iter() {
                                done += 1;
                                *settings_status.write() =
                                    Some(format!("正在生成摘要 {}/{}...", done, total));
                                let title = items[*index].title.clone();
                                let content = if body_text.is_empty() {
                                    items[*index].summary.clone()
                                } else {
                                    body_text.clone()
                                };
                                let fallback = fallback.clone();
                                let endpoint = endpoint.clone();
                                let api_key = api_key.clone();
                                let model = model.clone();
                                futures.push(async move {
                                    let result = general_ai_client::summarize_text(
                                        &endpoint, &api_key, &model, &title, &content,
                                    )
                                    .await;
                                    (result, fallback)
                                });
                            }

                            let results = futures::future::join_all(futures).await;
                            for ((index, _body_text, _fallback), (result, fallback)) in
                                group.iter().zip(results.into_iter())
                            {
                                let item = &mut items[*index];
                                match result {
                                    Ok(summary) => {
                                        let cleaned = ammonia::Builder::default()
                                            .add_tags(["p", "br"])
                                            .clean(&summary)
                                            .to_string();
                                        if cleaned.trim().is_empty() {
                                            item.summary = fallback;
                                            item.summarized = false;
                                        } else {
                                            item.summary = cleaned;
                                            item.summarized = true;
                                        }
                                    }
                                    Err(_) => {
                                        item.summary = fallback;
                                        item.summarized = false;
                                    }
                                }
                            }

                            chunk_offset = group_end;
                        }
                        offset = batch_end;
                    }
                }
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
        *feed_syncing.write() = false;
    });
}
