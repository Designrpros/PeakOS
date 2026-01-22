// Initialization logic for PeakNative

use super::{AppState, Message, PeakNative};
#[cfg(not(target_arch = "wasm32"))]
use crate::audio;
use crate::components::inspector::Inspector;
use crate::components::omnibar::Omnibar;
use crate::pages::Page;
use iced::Task;
use peak_apps::library::LibraryMessage;
use peak_core::models::MediaItem;
use peak_core::registry::ShellMode;
use peak_shell::app_switcher::AppSwitcher;
#[cfg(not(target_arch = "wasm32"))]
use sysinfo::System;

impl PeakNative {
    pub fn new(flags: crate::app::PeakNativeFlags) -> (Self, Task<Message>) {
        let mode_str = flags.mode;
        let launch_mode = flags.launch_mode;

        let theme = peak_core::theme::Theme::Light;

        // Initialize apps (moved down)

        // Initialize Audio (One-time)
        // ONLY valid for Desktop mode to avoid fighting for audio device
        #[cfg(not(target_arch = "wasm32"))]
        let stream = if launch_mode == crate::app::LaunchMode::Desktop {
            audio::init()
        } else {
            None
        };

        #[cfg(not(target_arch = "wasm32"))]
        if stream.is_some() {
            audio::set_volume(0.5);
        }

        // Initialize apps (Skip for Bar/Dock to save RAM/Time and avoid DB locks)
        let final_games = if launch_mode == crate::app::LaunchMode::Desktop {
            MediaItem::scan_system()
        } else {
            Vec::new()
        };

        let mut current_page = Page::Home;
        if mode_str == "poolside" {
            current_page = Page::Poolside;
        }

        let load_images_task = if launch_mode == crate::app::LaunchMode::Desktop {
            Task::batch(final_games.iter().map(|g| {
                let url = g.cover_image.clone();
                Task::perform(
                    peak_core::systems::image_loader::ImageLoader::load(url.clone()),
                    move |handle| match handle {
                        Some(h) => Message::Library(LibraryMessage::ImageLoaded(url.clone(), h)),
                        None => Message::Library(LibraryMessage::ImageLoadFailed(url.clone())),
                    },
                )
            }))
        } else {
            Task::none()
        };

        // Check for existing user (Skip for Bar/Dock, they don't need auth state)
        let user_profile = if launch_mode == crate::app::LaunchMode::Desktop {
            peak_apps::auth::load_user()
        } else {
            None
        };

        // If we are Bar/Dock, we are always "Logged In" effectively (or at least running)
        // But to avoid rendering the Login screen, we must set state to Desktop
        let initial_state = if launch_mode == crate::app::LaunchMode::Desktop {
            if user_profile.is_some() {
                AppState::Login(String::new())
            } else {
                AppState::Setup(peak_apps::wizard::WizardState::default())
            }
        } else {
            // Bar/Dock always start in Desktop state to render their views immediately
            AppState::Desktop
        };

        let mut shell = Self {
            state: initial_state,
            user: user_profile,
            // vector_bg removed
            theme,
            current_page,
            games: final_games.clone(), // Clone for self.games
            cortex_state: crate::pages::cortex::State::new(),
            mode: match mode_str.as_str() {
                "mobile" => ShellMode::Mobile,
                "auto" => ShellMode::Auto,
                "console" => ShellMode::Console,
                "fireplace" => ShellMode::Fireplace,
                "kiosk" => ShellMode::Kiosk,
                "robot" => ShellMode::Robot,
                "server" => ShellMode::Server,
                "smarthome" => ShellMode::SmartHome,
                "tv" => ShellMode::TV,
                _ => ShellMode::Desktop,
            },
            shell_style: if let Some(ref s) = flags.style {
                use peak_core::registry::ShellStyle;
                match s.to_lowercase().as_str() {
                    "cupertino" => ShellStyle::Cupertino,
                    "redmond" => ShellStyle::Redmond,
                    "ai" => ShellStyle::AI,
                    "console" => ShellStyle::Console,
                    "tv" => ShellStyle::TV,
                    _ => ShellStyle::Cupertino,
                }
            } else {
                peak_core::registry::ShellStyle::default() // Cupertino (macOS) by default
            },
            launch_mode,
            custom_wallpaper: None,
            inspector: Inspector::new(),
            current_desktop: 0,
            omnibar: Omnibar::new(),
            show_omnibar: false,
            #[cfg(not(target_arch = "wasm32"))]
            system: System::new_all(),
            last_monitor_update: instant::Instant::now(),
            switcher: AppSwitcher::new(),
            show_switcher: false,
            #[cfg(not(target_arch = "wasm32"))]
            _stream: stream,
            show_settings: false,
            show_spaces_selector: false,
            // Initialize Registry & Window Management
            registry: crate::systems::registry::AppRegistry::new(),
            window_manager: crate::systems::window_manager::WindowManager::new(),
            _show_app_library: false,
            cursor_position: iced::Point::ORIGIN,
            dock_visible: true,
            show_system_menu: false,
            show_reality_menu: false,
            show_wifi_menu: false,
            show_app_grid: false,
            networks: Vec::new(),
            // browser_process removed - using external Firefox
            window_position: iced::Point::ORIGIN,
            is_polling_window: false,
            polling_attempts: 0,
            alert: None,

            // Initialize Advanced Dock State
            pinned_apps: vec![
                peak_core::registry::AppId::Terminal,
                peak_core::registry::AppId::PeakUI,
                peak_core::registry::AppId::Browser,
                peak_core::registry::AppId::Turntable,
                peak_core::registry::AppId::Library,
                peak_core::registry::AppId::FileManager,
                peak_core::registry::AppId::Store,
                peak_core::registry::AppId::Settings,
                peak_core::registry::AppId::AppGrid,
            ],
            running_apps: Vec::new(),
            dragging_app: None,
            context_menu_app: None,
            is_desktop_revealed: false,
            quick_look_path: None,
            tracked_modifiers: iced::keyboard::Modifiers::default(),
            is_mouse_button_pressed: false,
            scanned_apps: {
                let mut apps = peak_core::registry::AppInfo::all_as_media();
                apps.extend(final_games.clone());
                apps
            },

            // Initialize Desktop
            desktop: crate::components::desktop::Desktop::new(),
            show_editor: false,
            tokens: peak_theme::ThemeTokens::get(
                match mode_str.as_str() {
                    "mobile" => ShellMode::Mobile,
                    "auto" => ShellMode::Auto,
                    "console" => ShellMode::Console,
                    "fireplace" => ShellMode::Fireplace,
                    "kiosk" => ShellMode::Kiosk,
                    "robot" => ShellMode::Robot,
                    "server" => ShellMode::Server,
                    "smarthome" => ShellMode::SmartHome,
                    "tv" => ShellMode::TV,
                    _ => ShellMode::Desktop,
                },
                peak_theme::ThemeTone::Light,
            ),
            active_downloads: std::collections::HashSet::new(),
            assistant: None,
            active_model_id: None,
            pending_chat: None,
            ai_input_text: String::new(),
        };

        // --- Register Modular Apps ---

        // NOTE: Browser removed - using Firefox via `opener::open` instead

        // Terminal (with Desktop Wrapper)
        #[cfg(not(target_arch = "wasm32"))]
        {
            let terminal = peak_apps::terminal::DesktopTerminalApp::new();
            shell.registry.register(
                peak_core::registry::AppId::Terminal,
                Box::new(crate::systems::registry::AppWrapper {
                    app: terminal,
                    map_msg: Message::Terminal,
                    try_unmap: |msg| match msg {
                        Message::Terminal(t) => Some(t),
                        _ => None,
                    },
                }),
            );
        }

        // PeakUI Reference App
        let peak_ui = peak_apps::peak_ui::PeakUIApp::new();
        shell.registry.register(
            peak_core::registry::AppId::PeakUI,
            Box::new(crate::systems::registry::AppWrapper {
                app: peak_ui,
                map_msg: Message::PeakUI,
                try_unmap: |msg| match msg {
                    Message::PeakUI(m) => Some(m),
                    _ => None,
                },
            }),
        );

        // Settings (with Desktop Wrapper)
        let settings = peak_apps::settings::DesktopSettingsApp::new();
        shell.registry.register(
            peak_core::registry::AppId::Settings,
            Box::new(crate::systems::registry::AppWrapper {
                app: settings,
                map_msg: Message::Settings,
                try_unmap: |msg| match msg {
                    Message::Settings(s) => Some(s),
                    _ => None,
                },
            }),
        );

        // Library
        let library = peak_apps::library::LibraryApp::new(final_games.clone());
        shell.registry.register(
            peak_core::registry::AppId::Library,
            Box::new(crate::systems::registry::AppWrapper {
                app: library,
                map_msg: Message::Library,
                try_unmap: |msg| match msg {
                    Message::Library(m) => Some(m),
                    _ => None,
                },
            }),
        );

        // Jukebox (Turntable)
        let jukebox = peak_apps::jukebox::JukeboxApp::new(final_games.clone());
        shell.registry.register(
            peak_core::registry::AppId::Turntable,
            Box::new(crate::systems::registry::AppWrapper {
                app: jukebox,
                map_msg: Message::Jukebox,
                try_unmap: |msg| match msg {
                    Message::Jukebox(m) => Some(m),
                    _ => None,
                },
            }),
        );

        // Store
        let store = peak_apps::store::StoreApp::new();
        shell.registry.register(
            peak_core::registry::AppId::Store,
            Box::new(crate::systems::registry::AppWrapper {
                app: store,
                map_msg: Message::Store,
                try_unmap: |msg| match msg {
                    Message::Store(m) => Some(m),
                    _ => None,
                },
            }),
        );

        // Explorer (FileManager)
        let explorer = peak_apps::explorer::ExplorerApp::new();
        shell.registry.register(
            peak_core::registry::AppId::FileManager,
            Box::new(crate::systems::registry::AppWrapper {
                app: explorer,
                map_msg: Message::Explorer,
                try_unmap: |msg| match msg {
                    Message::Explorer(m) => Some(m),
                    _ => None,
                },
            }),
        );

        // Editor
        let editor = peak_apps::editor::EditorApp::new();
        shell.registry.register(
            peak_core::registry::AppId::Editor,
            Box::new(crate::systems::registry::AppWrapper {
                app: editor,
                map_msg: Message::Editor,
                try_unmap: |msg| match msg {
                    Message::Editor(m) => Some(m),
                    _ => None,
                },
            }),
        );

        shell.update_style_from_mode();
        (shell, load_images_task)
    }
}
