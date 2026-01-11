mod app;
mod apps;
mod audio;
mod components;
mod icons;
mod integrations;
mod models;
mod pages;
mod registry;
pub mod styles;
mod systems;
pub mod theme;
mod utils;

use app::PeakNative;

pub fn main() -> iced::Result {
    // Simple manual parsing to avoid heavy dependencies
    let args: Vec<String> = std::env::args().collect();

    // Check for standalone browser mode (Multi-process support for macOS)
    if let Some(pos) = args.iter().position(|r| r == "--browser") {
        if let Some(url) = args.get(pos + 1) {
            crate::apps::browser::BrowserApp::run(url);
            return Ok(());
        }
    }

    let mode_arg = args
        .iter()
        .position(|r| r == "--mode")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "game".to_string());

    let is_game_mode = mode_arg == "game";

    iced::application(PeakNative::title, PeakNative::update, PeakNative::view)
        .theme(PeakNative::theme)
        .subscription(PeakNative::subscription)
        .window(iced::window::Settings {
            decorations: !is_game_mode,
            transparent: is_game_mode,
            resizable: !is_game_mode,
            ..Default::default()
        })
        .run_with(move || PeakNative::new(mode_arg.clone()))
}
