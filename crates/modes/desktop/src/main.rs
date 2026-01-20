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

    // NOTE: Standalone browser mode removed - using Firefox instead

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

    let _is_game_mode = args.contains(&"--game".to_string());

    iced::application(PeakNative::title, PeakNative::update, PeakNative::view)
        .theme(PeakNative::theme)
        .subscription(PeakNative::subscription)
        .window(iced::window::Settings {
            decorations: false,
            transparent: true,
            size: match launch_mode {
                app::LaunchMode::Bar => iced::Size::new(1920.0, 48.0),
                app::LaunchMode::Dock => iced::Size::new(800.0, 100.0),
                _ => iced::Size::new(1280.0, 720.0),
            },
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
