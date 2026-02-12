use crate::state::{FeedItem, FeedSourceKind, ThemeMode};

#[derive(Default)]
pub struct StoredSettings {
    pub gist_url: Option<String>,
    pub memory_server_url: Option<String>,
    pub theme: Option<ThemeMode>,
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use super::{FeedItem, FeedSourceKind, StoredSettings, ThemeMode};
    use rusqlite::{params, Connection};
    use std::path::PathBuf;

    const SETTINGS_GIST_URL: &str = "gist_url";
    const SETTINGS_MEMORY_SERVER_URL: &str = "memory_server_url";
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
            "CREATE TABLE IF NOT EXISTS settings (\n                key TEXT PRIMARY KEY,\n                value TEXT NOT NULL\n            );\n            CREATE TABLE IF NOT EXISTS feeds (\n                id TEXT PRIMARY KEY,\n                title TEXT NOT NULL,\n                summary TEXT NOT NULL,\n                source TEXT NOT NULL,\n                published_at TEXT NOT NULL,\n                published_ts INTEGER NOT NULL,\n                link TEXT NOT NULL,\n                author TEXT NOT NULL,\n                avatar_url TEXT,\n                tags TEXT\n            );",
        )?;
        let _ = conn.execute("ALTER TABLE feeds ADD COLUMN tags TEXT", []);
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

    pub fn load_settings() -> StoredSettings {
        let mut settings = StoredSettings::default();
        let Ok(conn) = open_db() else {
            return settings;
        };
        let Ok(mut stmt) = conn.prepare("SELECT key, value FROM settings") else {
            return settings;
        };
        let Ok(rows) = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))) else {
            return settings;
        };
        for row in rows.flatten() {
            match row.0.as_str() {
                SETTINGS_GIST_URL => settings.gist_url = Some(row.1),
                SETTINGS_MEMORY_SERVER_URL => settings.memory_server_url = Some(row.1),
                SETTINGS_THEME => settings.theme = theme_from_value(&row.1),
                _ => {}
            }
        }
        settings
    }

    pub fn store_gist_url(url: &str) {
        let Ok(conn) = open_db() else {
            return;
        };
        let _ = upsert_setting(&conn, SETTINGS_GIST_URL, url);
    }

    pub fn store_memory_server_url(url: &str) {
        let Ok(conn) = open_db() else {
            return;
        };
        let _ = upsert_setting(&conn, SETTINGS_MEMORY_SERVER_URL, url);
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
            "SELECT id, title, summary, source, published_at, published_ts, link, author, avatar_url, tags\n            FROM feeds\n            ORDER BY published_ts DESC",
        ) else {
            return Vec::new();
        };
        let Ok(rows) = stmt.query_map([], |row| {
            let source: String = row.get(3)?;
            let tags_raw: Option<String> = row.get(9)?;
            let tags = tags_raw
                .unwrap_or_default()
                .split(',')
                .map(|tag| tag.trim().to_string())
                .filter(|tag| !tag.is_empty())
                .collect::<Vec<_>>();
            Ok(FeedItem {
                id: row.get(0)?,
                title: row.get(1)?,
                summary: row.get(2)?,
                source: source_from_value(&source),
                published_at: row.get(4)?,
                published_ts: row.get(5)?,
                link: row.get(6)?,
                author: row.get(7)?,
                avatar_url: row.get(8)?,
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
                    "INSERT INTO feeds (id, title, summary, source, published_at, published_ts, link, author, avatar_url, tags)\n                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                )
                .map_err(|err| format!("prepare insert failed: {err}"))?;
            for item in items {
                let tags = item.tags.join(",");
                stmt.execute(params![
                    item.id,
                    item.title,
                    item.summary,
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

    const GIST_STORAGE_KEY: &str = "umbreon.gist_url";
    const MEMORY_SERVER_STORAGE_KEY: &str = "umbreon.memory_server_url";
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

    pub fn load_settings() -> StoredSettings {
        let mut settings = StoredSettings::default();
        let Some(window) = web_sys::window() else {
            return settings;
        };
        let Ok(Some(storage)) = window.local_storage() else {
            return settings;
        };
        if let Ok(Some(value)) = storage.get_item(GIST_STORAGE_KEY) {
            settings.gist_url = Some(value);
        }
        if let Ok(Some(value)) = storage.get_item(MEMORY_SERVER_STORAGE_KEY) {
            settings.memory_server_url = Some(value);
        }
        if let Ok(Some(value)) = storage.get_item(THEME_STORAGE_KEY) {
            settings.theme = theme_from_value(&value);
        }
        settings
    }

    pub fn store_gist_url(url: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(GIST_STORAGE_KEY, url);
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
    load_feed_items,
    load_settings,
    store_feed_items,
    store_gist_url,
    store_memory_server_url,
    store_theme,
};
