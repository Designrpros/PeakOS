mod app;
#[cfg(not(target_arch = "wasm32"))]
mod audio;
mod components;
#[cfg(target_os = "linux")]
mod layer_app;
mod pages;
#[cfg(not(target_arch = "wasm32"))]
mod recorder;

#[cfg(not(target_arch = "wasm32"))]
mod systems;
#[cfg(target_arch = "wasm32")]
mod systems {
    pub mod registry;
    pub mod window_manager;
}

use app::PeakNative;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple manual parsing to avoid heavy dependencies
    let args: Vec<String> = std::env::args().collect();

    let mode_arg = args
        .iter()
        .position(|r| r == "--mode")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "game".to_string());

    let mut launch_mode = app::LaunchMode::Desktop;
    if args.iter().any(|r| r == "--bar") {
        launch_mode = app::LaunchMode::Bar;
    } else if args.iter().any(|r| r == "--dock") {
        launch_mode = app::LaunchMode::Dock;
    }

    let style_arg = args
        .iter()
        .position(|r| r == "--style")
        .and_then(|i| args.get(i + 1))
        .cloned();

    let flags = app::PeakNativeFlags {
        mode: mode_arg.clone(),
        launch_mode,
        style: style_arg.clone(),
    };

    // Process spawning (Only for Desktop mode/Launcher)
    #[cfg(target_os = "linux")]
    if launch_mode == app::LaunchMode::Desktop && !args.iter().any(|r| r == "--layer") {
        // Spawn bar and dock
        std::thread::spawn(|| {
            // In a real scenario, use std::process::Command
            // Using current executable
            if let Ok(exe) = std::env::current_exe() {
                let _ = std::process::Command::new(&exe)
                    .arg("--layer")
                    .arg("--bar")
                    .spawn();

                let _ = std::process::Command::new(&exe)
                    .arg("--layer")
                    .arg("--dock")
                    .spawn();
            }
        });
    }

    #[cfg(target_os = "linux")]
    if args.iter().any(|r| r == "--layer") {
        use iced_layershell::Application;

        let layer_settings = match launch_mode {
            app::LaunchMode::Bar => layer_app::get_menubar_settings(),
            app::LaunchMode::Dock => layer_app::get_dock_settings(),
            app::LaunchMode::Desktop => layer_app::get_desktop_settings(), // Integrated Desktop (Wallpaper)
        };

        return layer_app::PeakLayerShell::run(iced_layershell::settings::Settings {
            flags,
            layer_settings,
            ..Default::default()
        })
        .map_err(|e| e.into());
    }

    use peak_ui::core::App;
    PeakNative::run(flags).map_err(|e| e.into())
}

#[cfg(target_arch = "wasm32")]
pub fn main() {}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Console log failed");

    log::info!("PeakOS WASM entry point started");

    let flags = app::PeakNativeFlags {
        mode: "game".to_string(),
        launch_mode: app::LaunchMode::Desktop,
        style: None,
    };

    log::info!("Launching iced application...");
    let _ = iced::application(PeakNative::title, PeakNative::update, PeakNative::view)
        .theme(PeakNative::theme)
        .subscription(PeakNative::subscription)
        .run_with(move || PeakNative::new(flags));
}
