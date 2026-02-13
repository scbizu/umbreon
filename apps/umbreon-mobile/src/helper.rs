use chrono::{DateTime, NaiveDateTime, Utc};
use feed_rs::model::FeedType;

pub fn parse_timestamp_atom(value: &str) -> Option<i64> {
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

pub fn parse_timestamp_rss(value: &str) -> Option<i64> {
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

pub fn parse_timestamp_for_feed(feed_type: &FeedType, value: &str) -> Option<i64> {
    match feed_type {
        FeedType::Atom => parse_timestamp_atom(value),
        FeedType::RSS1 => parse_timestamp_rss(value),
        FeedType::RSS2 => parse_timestamp_rss(value),
        _ => parse_timestamp_atom(value),
    }
}
