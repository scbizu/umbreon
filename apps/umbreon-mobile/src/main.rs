mod app;
mod components;
mod state;
mod storage;

use app::AppRoot;
use dioxus_mobile::{launch::launch, Config};
use std::any::Any;
use tracing::info;

fn main() {
    // Initialize logging early for debugging lifecycle hooks when running locally or via Android shell.
    tracing_subscriber::fmt::init();

    info!("launching Umbreon mobile shell with Dioxus");

    let platform_contexts: Vec<Box<dyn Fn() -> Box<dyn Any>>> = Vec::new();

    launch(AppRoot, platform_contexts, Config::default());
}
