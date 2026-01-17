mod app;
mod audio;
mod components;
#[cfg(target_os = "linux")]
mod layer_app;
mod pages;
mod systems;

use app::PeakNative;

pub fn main() -> iced::Result {
    // Simple manual parsing to avoid heavy dependencies
    let args: Vec<String> = std::env::args().collect();

    // Check for standalone browser mode (Multi-process support for macOS)
    if let Some(pos) = args.iter().position(|r| r == "--browser") {
        if let Some(url) = args.get(pos + 1) {
            peak_apps::browser::BrowserApp::run(url, None);
            return Ok(());
        }
    }

    #[cfg(target_os = "linux")]
    if args.iter().any(|r| r == "--layer") {
        use iced_layershell::Application;
        let mode_arg = args
            .iter()
            .position(|r| r == "--mode")
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| "peak".to_string());

        let layer_settings = layer_app::get_menubar_settings();
        return layer_app::PeakLayerShell::run(iced_layershell::Settings {
            flags: mode_arg,
            layer_settings,
            ..Default::default()
        });
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
