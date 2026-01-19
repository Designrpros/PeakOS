mod app;
mod audio;
mod components;
#[cfg(target_os = "linux")]
mod layer_app;
mod pages;
mod recorder;
mod systems;

use app::PeakNative;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple manual parsing to avoid heavy dependencies
    let args: Vec<String> = std::env::args().collect();

    // Check for standalone browser mode (Multi-process support for macOS)
    if let Some(pos) = args.iter().position(|r| r == "--browser") {
        if let Some(url) = args.get(pos + 1) {
            peak_apps::browser::BrowserApp::run(url, None);
            return Ok(());
        }
    }

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

    let flags = app::PeakNativeFlags {
        mode: mode_arg.clone(),
        launch_mode,
    };

    #[cfg(target_os = "linux")]
    if args.iter().any(|r| r == "--layer") {
        use iced_layershell::Application;

        let layer_settings = match launch_mode {
            app::LaunchMode::Bar => layer_app::get_menubar_settings(),
            app::LaunchMode::Dock => layer_app::get_dock_settings(),
            _ => layer_app::get_menubar_settings(), // Fallback or Desktop (but desktop usually isn't layer)
        };

        return layer_app::PeakLayerShell::run(iced_layershell::settings::Settings {
            flags,
            layer_settings,
            ..Default::default()
        })
        .map_err(|e| e.into());
    }

    // Process spawning (Only for Desktop mode/Launcher)
    #[cfg(target_os = "linux")]
    if launch_mode == app::LaunchMode::Desktop {
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

    let _is_game_mode = args.contains(&"--game".to_string());

    iced::application(PeakNative::title, PeakNative::update, PeakNative::view)
        .theme(PeakNative::theme)
        .subscription(PeakNative::subscription)
        .window(iced::window::Settings {
            decorations: false,
            transparent: true,
            size: iced::Size::new(1280.0, 720.0), // Smaller default, will maximize on Linux
            resizable: true,
            #[cfg(target_os = "linux")]
            platform_specific: iced::window::settings::PlatformSpecific {
                application_id: "peak-desktop".to_string(),
                ..Default::default()
            },
            ..Default::default()
        })
        .run_with(move || PeakNative::new(flags))
        .map_err(|e| e.into())
}
