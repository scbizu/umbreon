mod app;
mod components;
mod helper;
mod memory_client;
mod settings;
mod state;
mod storage;
mod style;
mod timeline;

use app::AppRoot;
use dioxus_mobile::{Config, launch::launch};
use std::any::Any;
use tracing::info;

fn main() {
    // Initialize logging early for debugging lifecycle hooks when running locally or via Android shell.
    tracing_subscriber::fmt::init();

    info!("launching Umbreon mobile shell with Dioxus");

    let platform_contexts: Vec<Box<dyn Fn() -> Box<dyn Any>>> = Vec::new();

    launch(AppRoot, platform_contexts, Config::default());
}
