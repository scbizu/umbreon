use crate::state::{use_app_context, LiveStream, MediaKind, MediaSession};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NowPlayingPane() -> Element {
    let ctx = use_app_context();
    let now_playing = ctx.now_playing.read().clone();
    let live_streams = ctx.live_streams.read().clone();

    rsx! {
        section { class: "player-pane",
            header { h2 { "Media" } }
            if let Some(session) = now_playing {
                NowPlayingCard { session }
            } else {
                p { class: "empty-state", "Select a stream or VOD to get started." }
            }
            LiveStreamList { streams: live_streams }
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
