use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavSection {
    Timeline,
    Live,
    Vod,
    Memory,
}

impl NavSection {
    pub const ALL: [NavSection; 4] = [
        NavSection::Timeline,
        NavSection::Live,
        NavSection::Vod,
        NavSection::Memory,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            NavSection::Timeline => "Timeline",
            NavSection::Live => "Live",
            NavSection::Vod => "VOD",
            NavSection::Memory => "Memory",
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
    pub link: String,
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
    pub feed_items: Signal<Vec<FeedItem>>,
    pub live_streams: Signal<Vec<LiveStream>>,
    pub now_playing: Signal<Option<MediaSession>>,
    pub memory_panel: Signal<MemoryPanelState>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            nav: use_signal(|| NavSection::Timeline),
            feed_items: use_signal(mock_feed_items),
            live_streams: use_signal(mock_live_streams),
            now_playing: use_signal(mock_initial_session),
            memory_panel: use_signal(mock_memory_panel),
        }
    }
}

pub fn use_app_context() -> AppContext {
    use_context::<AppContext>()
}

fn mock_feed_items() -> Vec<FeedItem> {
    vec![
        FeedItem {
            id: "atom-001".into(),
            title: "Rust workspace planning for Umbreon".into(),
            summary: "Progress log about feed aggregation, danmaku playback, and memory bridge.".into(),
            source: FeedSourceKind::Atom,
            published_at: "2026-02-05T11:30:00Z".into(),
            link: "https://example.com/blog/umbreon-planning".into(),
        },
        FeedItem {
            id: "rsshub-fox-001".into(),
            title: "Bilibili LIVE schedule highlights".into(),
            summary: "Auto-subscribed via RSSHub, includes danmaku endpoints.".into(),
            source: FeedSourceKind::RssHub,
            published_at: "2026-02-05T08:25:00Z".into(),
            link: "https://rsshub.app/bilibili/live".into(),
        },
        FeedItem {
            id: "custom-yt-042".into(),
            title: "Custom crawler fetched new VOD playlist".into(),
            summary: "CloudWorker parsed playlist.m3u + metadata injection.".into(),
            source: FeedSourceKind::Custom,
            published_at: "2026-02-04T21:03:00Z".into(),
            link: "https://gist.github.com/umbreon/vod".into(),
        },
    ]
}

fn mock_live_streams() -> Vec<LiveStream> {
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

fn mock_initial_session() -> Option<MediaSession> {
    Some(MediaSession {
        title: "Umbreon intro stream".into(),
        source: "Internal LIVE".into(),
        kind: MediaKind::Live,
        stream_url: "https://live.cdn.example.com/umbreon/intro.m3u8".into(),
        danmaku_endpoint: Some("wss://danmaku.example.com/umbreon".into()),
    })
}

fn mock_memory_panel() -> MemoryPanelState {
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
