use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn toggle(&self) -> ThemeMode {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavSection {
    Timeline,
    Live,
    Vod,
    Memory,
    Settings,
}

impl NavSection {
    pub const ALL: [NavSection; 5] = [
        NavSection::Timeline,
        NavSection::Live,
        NavSection::Vod,
        NavSection::Memory,
        NavSection::Settings,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            NavSection::Timeline => "圈子",
            NavSection::Live => "直播",
            NavSection::Vod => "追番",
            NavSection::Memory => "吾魂",
            NavSection::Settings => "设置",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            NavSection::Timeline => "article",
            NavSection::Live => "live_tv",
            NavSection::Vod => "movie",
            NavSection::Memory => "memory",
            NavSection::Settings => "settings",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedSourceKind {
    Atom,
    RssHub,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeedItem {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub source: FeedSourceKind,
    pub published_at: String,
    pub published_ts: i64,
    pub link: String,
    pub author: String,
    pub avatar_url: Option<String>,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaKind {
    Live,
    Vod,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveStream {
    pub id: String,
    pub title: String,
    pub stream_url: String,
    pub danmaku_endpoint: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MediaSession {
    pub title: String,
    pub source: String,
    pub kind: MediaKind,
    pub stream_url: String,
    pub danmaku_endpoint: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryPanelState {
    pub synced: bool,
    pub last_synced: Option<String>,
    pub highlights: Vec<String>,
}

impl Default for MemoryPanelState {
    fn default() -> Self {
        Self {
            synced: false,
            last_synced: None,
            highlights: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct AppContext {
    pub nav: Signal<NavSection>,
    pub theme: Signal<ThemeMode>,
    pub sidebar_collapsed: Signal<bool>,
    pub feed_items: Signal<Vec<FeedItem>>,
    pub live_streams: Signal<Vec<LiveStream>>,
    pub now_playing: Signal<Option<MediaSession>>,
    pub memory_panel: Signal<MemoryPanelState>,
    pub gist_url: Signal<String>,
    pub memory_server_url: Signal<String>,
    pub settings_status: Signal<Option<String>>,
}

pub fn use_app_context() -> AppContext {
    use_context::<AppContext>()
}

pub(crate) fn mock_feed_items() -> Vec<FeedItem> {
    vec![
        FeedItem {
            id: "atom-001".into(),
            title: "Rust workspace planning for Umbreon".into(),
            summary: "Progress log about feed aggregation, danmaku playback, and memory bridge."
                .into(),
            source: FeedSourceKind::Atom,
            published_at: "2026-02-05T11:30:00Z".into(),
            published_ts: 1770271800,
            link: "https://example.com/blog/umbreon-planning".into(),
            author: "Umbreon Blog".into(),
            avatar_url: Some("https://example.com/assets/umbreon-icon.png".into()),
        },
        FeedItem {
            id: "rsshub-fox-001".into(),
            title: "Bilibili LIVE schedule highlights".into(),
            summary: "Auto-subscribed via RSSHub, includes danmaku endpoints.".into(),
            source: FeedSourceKind::RssHub,
            published_at: "2026-02-05T08:25:00Z".into(),
            published_ts: 1770260700,
            link: "https://rsshub.app/bilibili/live".into(),
            author: "RSSHub".into(),
            avatar_url: Some("https://rsshub.app/logo.png".into()),
        },
        FeedItem {
            id: "custom-yt-042".into(),
            title: "Custom crawler fetched new VOD playlist".into(),
            summary: "CloudWorker parsed playlist.m3u + metadata injection.".into(),
            source: FeedSourceKind::Custom,
            published_at: "2026-02-04T21:03:00Z".into(),
            published_ts: 1770200580,
            link: "https://gist.github.com/umbreon/vod".into(),
            author: "Umbreon Crawler".into(),
            avatar_url: None,
        },
    ]
}

pub(crate) fn mock_live_streams() -> Vec<LiveStream> {
    vec![
        LiveStream {
            id: "live-001".into(),
            title: "伴生体策略例会".into(),
            stream_url: "https://live.cdn.example.com/channel/umbreon/index.m3u8".into(),
            danmaku_endpoint: Some("wss://danmaku.example.com/umbreon".into()),
        },
        LiveStream {
            id: "live-002".into(),
            title: "夜间低延迟测试".into(),
            stream_url: "https://edge.example.net/night.m3u8".into(),
            danmaku_endpoint: None,
        },
    ]
}

pub(crate) fn mock_initial_session() -> Option<MediaSession> {
    Some(MediaSession {
        title: "Umbreon intro stream".into(),
        source: "Internal LIVE".into(),
        kind: MediaKind::Live,
        stream_url: "https://live.cdn.example.com/umbreon/intro.m3u8".into(),
        danmaku_endpoint: Some("wss://danmaku.example.com/umbreon".into()),
    })
}

pub(crate) fn mock_memory_panel() -> MemoryPanelState {
    MemoryPanelState {
        synced: true,
        last_synced: Some("2026-02-06T08:15:00Z".into()),
        highlights: vec![
            "Feed preference: prioritize RSSHub anime tech".into(),
            "Reminder: test custom danmaku overlay".into(),
            "Memory: Live latency target < 2s".into(),
        ],
    }
}
