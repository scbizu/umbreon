use crate::state::{use_app_context, LiveStream, MediaKind, MediaSession};
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn NowPlayingPane(mode: MediaKind) -> Element {
    let ctx = use_app_context();
    let now_playing = ctx.now_playing.read().clone();
    let live_streams = ctx.live_streams.read().clone();
    let title = match mode {
        MediaKind::Live => "Live",
        MediaKind::Vod => "VOD",
    };

    rsx! {
        section { class: "player-pane",
            header { h2 { "{title}" } }
            if let Some(session) = now_playing {
                NowPlayingCard { session }
            } else {
                p { class: "empty-state", "Select a stream or VOD to get started." }
            }
            if mode == MediaKind::Live {
                LiveStreamList { streams: live_streams }
            } else {
                p { class: "empty-state", "VOD library is empty. Add sources in your config." }
            }
        }
    }
}

#[component]
fn NowPlayingCard(session: MediaSession) -> Element {
    let kind_label = match session.kind {
        MediaKind::Live => "LIVE",
        MediaKind::Vod => "VOD",
    };

    rsx! {
        div { class: "now-playing",
            span { class: "badge", "{kind_label}" }
            h3 { "{session.title}" }
            p { class: "source", "Source: {session.source}" }
            p { class: "stream", "Stream: {session.stream_url}" }
            if let Some(endpoint) = session.danmaku_endpoint {
                p { class: "danmaku", "Danmaku: {endpoint}" }
            }
        }
    }
}

#[component]
fn LiveStreamList(streams: Vec<LiveStream>) -> Element {
    if streams.is_empty() {
        return rsx! { p { class: "empty-state", "No live entries" } };
    }

    rsx! {
        div { class: "live-stream-list",
            h3 { "Live entries" }
            ul {
                for stream in streams.into_iter() {
                    li { key: "{stream.id}",
                        span { class: "title", "{stream.title}" }
                        small { class: "url", "{stream.stream_url}" }
                        if let Some(danmaku) = stream.danmaku_endpoint {
                            small { class: "danmaku", "Danmaku: {danmaku}" }
                        }
                    }
                }
            }
        }
    }
}
