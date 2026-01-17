// Initialization logic for PeakNative

use super::{AppState, Message, PeakNative};
use crate::audio;
use crate::components::inspector::Inspector;
use crate::components::omnibar::Omnibar;
use crate::pages::Page;
use iced::Task;
use peak_apps::library::LibraryMessage;
use peak_core::models::MediaItem;
use peak_core::registry::ShellMode;
use peak_shell::app_switcher::AppSwitcher;
use sysinfo::System;

impl PeakNative {
    pub fn new(mode_str: String) -> (Self, Task<Message>) {
        let theme = peak_core::theme::Theme::Light;

        // Initialize apps
        let final_games = MediaItem::scan_system();

        // Initialize Audio (One-time, optional)
        let stream = audio::init();
        if stream.is_some() {
            audio::set_volume(0.5);
        }

        let mut current_page = Page::Home;
        if mode_str == "poolside" {
            current_page = Page::Poolside;
        }

        let load_images_task = Task::batch(final_games.iter().map(|g| {
            let url = g.cover_image.clone();
            Task::perform(
                peak_core::systems::image_loader::ImageLoader::load(url.clone()),
                move |handle| match handle {
                    Some(h) => Message::Library(LibraryMessage::ImageLoaded(url.clone(), h)),
                    None => Message::Library(LibraryMessage::ImageLoadFailed(url.clone())),
                },
            )
        }));

        // Check for existing user
        let user_profile = peak_apps::auth::load_user();

        let initial_state = if user_profile.is_some() {
            AppState::Login(String::new())
        } else {
            AppState::Setup(peak_apps::wizard::WizardState::default())
        };

        let mut shell = Self {
            state: initial_state,
            user: user_profile,
            vector_bg: crate::components::vector_background::VectorBackground::new(),
            theme,
            current_page,
            games: final_games.clone(), // Clone for self.games
            cortex_state: crate::pages::cortex::State::new(),
            mode: if mode_str == "poolside" {
                ShellMode::Poolside
            } else {
                ShellMode::Peak
            },
            custom_wallpaper: None,
            inspector: Inspector::new(),
            current_desktop: 0,
            omnibar: Omnibar::new(),
            show_omnibar: false,
            system: System::new_all(),
            last_monitor_update: std::time::Instant::now(),
            switcher: AppSwitcher::new(),
            show_switcher: false,
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
            browser_process: None,
            window_position: iced::Point::ORIGIN,
            is_polling_window: false,
            polling_attempts: 0,
            alert: None,

            // Initialize Advanced Dock State
            pinned_apps: vec![
                peak_core::registry::AppId::Terminal,
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
        };

        // --- Register Modular Apps ---

        // Browser
        let browser = peak_apps::browser_app::BrowserApp::new();
        shell.registry.register(
            peak_core::registry::AppId::Browser,
            Box::new(crate::systems::registry::AppWrapper {
                app: browser,
                map_msg: Message::Browser,
                try_unmap: |msg| match msg {
                    Message::Browser(b) => Some(b),
                    _ => None,
                },
            }),
        );

        // Terminal (with Desktop Wrapper)
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

        (shell, load_images_task)
    }
}
