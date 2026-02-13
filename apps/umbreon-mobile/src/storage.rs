use crate::state::{FeedItem, FeedSourceKind, ThemeMode};

#[derive(Default)]
pub struct StoredSettings {
    pub feed_server_url: Option<String>,
    pub memory_server_url: Option<String>,
    pub llm_endpoint: Option<String>,
    pub llm_api_key: Option<String>,
    pub llm_model: Option<String>,
    pub llm_models: Option<Vec<String>>,
    pub theme: Option<ThemeMode>,
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use super::{FeedItem, FeedSourceKind, StoredSettings, ThemeMode};
    use chrono::{FixedOffset, TimeZone};
    use rusqlite::{Connection, params};
    use std::path::PathBuf;

    const SETTINGS_FEED_SERVER_URL: &str = "feed_server_url";
    const SETTINGS_GIST_URL_LEGACY: &str = "gist_url";
    const SETTINGS_MEMORY_SERVER_URL: &str = "memory_server_url";
    const SETTINGS_LLM_ENDPOINT: &str = "llm_endpoint";
    const SETTINGS_LLM_API_KEY: &str = "llm_api_key";
    const SETTINGS_LLM_MODEL: &str = "llm_model";
    const SETTINGS_LLM_MODELS: &str = "llm_models";
    const SETTINGS_THEME: &str = "theme";

    fn db_path() -> PathBuf {
        let mut base = dirs::data_dir()
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(std::env::temp_dir);
        base.push("umbreon");
        let _ = std::fs::create_dir_all(&base);
        base.push("umbreon.db");
        base
    }

    fn open_db() -> Result<Connection, rusqlite::Error> {
        let conn = Connection::open(db_path())?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS settings (\n                key TEXT PRIMARY KEY,\n                value TEXT NOT NULL\n            );\n            CREATE TABLE IF NOT EXISTS feeds (\n                id TEXT PRIMARY KEY,\n                title TEXT NOT NULL,\n                summary TEXT NOT NULL,\n                full_content TEXT NOT NULL,\n                summarized INTEGER NOT NULL DEFAULT 0,\n                source TEXT NOT NULL,\n                published_at TEXT NOT NULL,\n                published_ts INTEGER NOT NULL,\n                link TEXT NOT NULL,\n                author TEXT NOT NULL,\n                avatar_url TEXT,\n                tags TEXT\n            );",
        )?;
        let _ = conn.execute("ALTER TABLE feeds ADD COLUMN tags TEXT", []);
        let _ = conn.execute("ALTER TABLE feeds ADD COLUMN full_content TEXT", []);
        let _ = conn.execute("ALTER TABLE feeds ADD COLUMN summarized INTEGER", []);
        Ok(conn)
    }

    fn upsert_setting(conn: &Connection, key: &str, value: &str) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)\n            ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    fn theme_to_value(theme: ThemeMode) -> &'static str {
        match theme {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }

    fn theme_from_value(value: &str) -> Option<ThemeMode> {
        match value {
            "light" => Some(ThemeMode::Light),
            "dark" => Some(ThemeMode::Dark),
            _ => None,
        }
    }

    fn models_to_value(models: &[String]) -> Option<String> {
        serde_json::to_string(models).ok()
    }

    fn models_from_value(value: &str) -> Option<Vec<String>> {
        serde_json::from_str::<Vec<String>>(value).ok()
    }

    fn source_to_value(source: &FeedSourceKind) -> &'static str {
        match source {
            FeedSourceKind::Atom => "atom",
            FeedSourceKind::RssHub => "rsshub",
            FeedSourceKind::Custom => "custom",
        }
    }

    fn source_from_value(value: &str) -> FeedSourceKind {
        match value {
            "rsshub" => FeedSourceKind::RssHub,
            "custom" => FeedSourceKind::Custom,
            _ => FeedSourceKind::Atom,
        }
    }

    fn format_date_utc8(ts: i64) -> Option<String> {
        let offset = FixedOffset::east_opt(8 * 3600)?;
        offset
            .timestamp_opt(ts, 0)
            .single()
            .map(|dt| dt.format("%Y-%m-%d").to_string())
    }

    pub fn load_settings() -> StoredSettings {
        let mut settings = StoredSettings::default();
        let Ok(conn) = open_db() else {
            return settings;
        };
        let Ok(mut stmt) = conn.prepare("SELECT key, value FROM settings") else {
            return settings;
        };
        let Ok(rows) = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }) else {
            return settings;
        };
        for row in rows.flatten() {
            match row.0.as_str() {
                SETTINGS_FEED_SERVER_URL => settings.feed_server_url = Some(row.1),
                SETTINGS_GIST_URL_LEGACY => {
                    if settings.feed_server_url.is_none() {
                        settings.feed_server_url = Some(row.1);
                    }
                }
                SETTINGS_MEMORY_SERVER_URL => settings.memory_server_url = Some(row.1),
                SETTINGS_LLM_ENDPOINT => settings.llm_endpoint = Some(row.1),
                SETTINGS_LLM_API_KEY => settings.llm_api_key = Some(row.1),
                SETTINGS_LLM_MODEL => settings.llm_model = Some(row.1),
                SETTINGS_LLM_MODELS => settings.llm_models = models_from_value(&row.1),
                SETTINGS_THEME => settings.theme = theme_from_value(&row.1),
                _ => {}
            }
        }
        settings
    }

    pub fn store_feed_server_url(url: &str) {
        let Ok(conn) = open_db() else {
            return;
        };
        let _ = upsert_setting(&conn, SETTINGS_FEED_SERVER_URL, url);
    }

    pub fn store_memory_server_url(url: &str) {
        let Ok(conn) = open_db() else {
            return;
        };
        let _ = upsert_setting(&conn, SETTINGS_MEMORY_SERVER_URL, url);
    }

    pub fn store_llm_endpoint(endpoint: &str) {
        let Ok(conn) = open_db() else {
            return;
        };
        let _ = upsert_setting(&conn, SETTINGS_LLM_ENDPOINT, endpoint);
    }

    pub fn store_llm_api_key(api_key: &str) {
        let Ok(conn) = open_db() else {
            return;
        };
        let _ = upsert_setting(&conn, SETTINGS_LLM_API_KEY, api_key);
    }

    pub fn store_llm_model(model: &str) {
        let Ok(conn) = open_db() else {
            return;
        };
        let _ = upsert_setting(&conn, SETTINGS_LLM_MODEL, model);
    }

    pub fn store_llm_models(models: &[String]) {
        let Some(value) = models_to_value(models) else {
            return;
        };
        let Ok(conn) = open_db() else {
            return;
        };
        let _ = upsert_setting(&conn, SETTINGS_LLM_MODELS, &value);
    }

    pub fn store_theme(theme: ThemeMode) {
        let Ok(conn) = open_db() else {
            return;
        };
        let _ = upsert_setting(&conn, SETTINGS_THEME, theme_to_value(theme));
    }

    pub fn load_feed_items() -> Vec<FeedItem> {
        let Ok(conn) = open_db() else {
            return Vec::new();
        };
        let Ok(mut stmt) = conn.prepare(
            "SELECT id, title, summary, full_content, summarized, source, published_at, published_ts, link, author, avatar_url, tags\n            FROM feeds\n            ORDER BY published_ts DESC",
        ) else {
            return Vec::new();
        };
        let Ok(rows) = stmt.query_map([], |row| {
            let source: String = row.get(5)?;
            let tags_raw: Option<String> = row.get(11)?;
            let tags = tags_raw
                .unwrap_or_default()
                .split(',')
                .map(|tag| tag.trim().to_string())
                .filter(|tag| !tag.is_empty())
                .collect::<Vec<_>>();
            let summary: String = row.get(2)?;
            let full_content: Option<String> = row.get(3)?;
            let summarized: i64 = row.get(4)?;
            let published_ts: i64 = row.get(7)?;
            let published_at_raw: String = row.get(6)?;
            Ok(FeedItem {
                id: row.get(0)?,
                title: row.get(1)?,
                summary: summary.clone(),
                full_content: full_content.unwrap_or(summary),
                summarized: summarized != 0,
                source: source_from_value(&source),
                published_at: format_date_utc8(published_ts).unwrap_or(published_at_raw),
                published_ts,
                link: row.get(8)?,
                author: row.get(9)?,
                avatar_url: row.get(10)?,
                tags,
            })
        }) else {
            return Vec::new();
        };
        rows.flatten().collect()
    }

    pub fn store_feed_items(items: &[FeedItem]) -> Result<(), String> {
        let mut conn = open_db().map_err(|err| format!("open db failed: {err}"))?;
        let tx = conn
            .transaction()
            .map_err(|err| format!("start transaction failed: {err}"))?;
        tx.execute("DELETE FROM feeds", [])
            .map_err(|err| format!("clear feeds failed: {err}"))?;
        {
            let mut stmt = tx
                .prepare(
                    "INSERT INTO feeds (id, title, summary, full_content, summarized, source, published_at, published_ts, link, author, avatar_url, tags)\n                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                )
                .map_err(|err| format!("prepare insert failed: {err}"))?;
            for item in items {
                let tags = item.tags.join(",");
                stmt.execute(params![
                    item.id,
                    item.title,
                    item.summary,
                    item.full_content,
                    if item.summarized { 1 } else { 0 },
                    source_to_value(&item.source),
                    item.published_at,
                    item.published_ts,
                    item.link,
                    item.author,
                    item.avatar_url,
                    tags,
                ])
                .map_err(|err| format!("insert feed failed: {err}"))?;
            }
        }
        tx.commit()
            .map_err(|err| format!("commit feeds failed: {err}"))?;
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
mod imp {
    use super::{FeedItem, StoredSettings, ThemeMode};

    const FEED_SERVER_STORAGE_KEY: &str = "umbreon.feed_server_url";
    const GIST_STORAGE_KEY_LEGACY: &str = "umbreon.gist_url";
    const MEMORY_SERVER_STORAGE_KEY: &str = "umbreon.memory_server_url";
    const LLM_ENDPOINT_STORAGE_KEY: &str = "umbreon.llm_endpoint";
    const LLM_API_KEY_STORAGE_KEY: &str = "umbreon.llm_api_key";
    const LLM_MODEL_STORAGE_KEY: &str = "umbreon.llm_model";
    const LLM_MODELS_STORAGE_KEY: &str = "umbreon.llm_models";
    const THEME_STORAGE_KEY: &str = "umbreon.theme";

    fn theme_from_value(value: &str) -> Option<ThemeMode> {
        match value {
            "light" => Some(ThemeMode::Light),
            "dark" => Some(ThemeMode::Dark),
            _ => None,
        }
    }

    fn theme_to_value(theme: ThemeMode) -> &'static str {
        match theme {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }

    fn models_to_value(models: &[String]) -> Option<String> {
        serde_json::to_string(models).ok()
    }

    fn models_from_value(value: &str) -> Option<Vec<String>> {
        serde_json::from_str::<Vec<String>>(value).ok()
    }

    pub fn load_settings() -> StoredSettings {
        let mut settings = StoredSettings::default();
        let Some(window) = web_sys::window() else {
            return settings;
        };
        let Ok(Some(storage)) = window.local_storage() else {
            return settings;
        };
        if let Ok(Some(value)) = storage.get_item(FEED_SERVER_STORAGE_KEY) {
            settings.feed_server_url = Some(value);
        } else if let Ok(Some(value)) = storage.get_item(GIST_STORAGE_KEY_LEGACY) {
            settings.feed_server_url = Some(value);
        }
        if let Ok(Some(value)) = storage.get_item(MEMORY_SERVER_STORAGE_KEY) {
            settings.memory_server_url = Some(value);
        }
        if let Ok(Some(value)) = storage.get_item(LLM_ENDPOINT_STORAGE_KEY) {
            settings.llm_endpoint = Some(value);
        }
        if let Ok(Some(value)) = storage.get_item(LLM_API_KEY_STORAGE_KEY) {
            settings.llm_api_key = Some(value);
        }
        if let Ok(Some(value)) = storage.get_item(LLM_MODEL_STORAGE_KEY) {
            settings.llm_model = Some(value);
        }
        if let Ok(Some(value)) = storage.get_item(LLM_MODELS_STORAGE_KEY) {
            settings.llm_models = models_from_value(&value);
        }
        if let Ok(Some(value)) = storage.get_item(THEME_STORAGE_KEY) {
            settings.theme = theme_from_value(&value);
        }
        settings
    }

    pub fn store_feed_server_url(url: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(FEED_SERVER_STORAGE_KEY, url);
            }
        }
    }

    pub fn store_memory_server_url(url: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(MEMORY_SERVER_STORAGE_KEY, url);
            }
        }
    }

    pub fn store_llm_endpoint(endpoint: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(LLM_ENDPOINT_STORAGE_KEY, endpoint);
            }
        }
    }

    pub fn store_llm_api_key(api_key: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(LLM_API_KEY_STORAGE_KEY, api_key);
            }
        }
    }

    pub fn store_llm_model(model: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(LLM_MODEL_STORAGE_KEY, model);
            }
        }
    }

    pub fn store_llm_models(models: &[String]) {
        let Some(value) = models_to_value(models) else {
            return;
        };
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(LLM_MODELS_STORAGE_KEY, &value);
            }
        }
    }

    pub fn store_theme(theme: ThemeMode) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(THEME_STORAGE_KEY, theme_to_value(theme));
            }
        }
    }

    pub fn load_feed_items() -> Vec<FeedItem> {
        Vec::new()
    }

    pub fn store_feed_items(_items: &[FeedItem]) -> Result<(), String> {
        Ok(())
    }
}

pub use imp::{
    load_feed_items, load_settings, store_feed_items, store_feed_server_url, store_llm_api_key,
    store_llm_endpoint, store_llm_model, store_llm_models, store_memory_server_url, store_theme,
};
