// Initialization logic for PeakNative

use super::{AppState, Message, PeakNative};
use crate::apps::explorer::ExplorerApp;
use crate::apps::library::{LibraryApp, LibraryMessage};
use crate::apps::settings::SettingsApp;
use crate::apps::terminal::TerminalApp;
use crate::audio;
use crate::components::app_switcher::AppSwitcher;
use crate::components::inspector::Inspector;
use crate::components::omnibar::Omnibar;
use crate::pages::Page;
use iced::Task;
use peak_core::models::MediaItem;
use peak_core::registry::ShellMode;
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
        let user_profile = crate::apps::auth::load_user();

        let initial_state = if user_profile.is_some() {
            AppState::Login(String::new())
        } else {
            AppState::Setup(crate::apps::wizard::WizardState::default())
        };

        (
            Self {
                state: initial_state,
                user: user_profile,
                vector_bg: crate::components::vector_background::VectorBackground::new(),
                theme,
                current_page,
                games: final_games,
                library: LibraryApp::new(),
                cortex_state: crate::pages::cortex::State::new(),
                mode: if mode_str == "poolside" {
                    ShellMode::Poolside
                } else {
                    ShellMode::Peak
                },
                terminal: TerminalApp::new(),
                settings: SettingsApp::new(),
                inspector: Inspector::new(),
                jukebox: crate::apps::jukebox::JukeboxApp::new(),
                explorer: ExplorerApp::new(),
                store: crate::apps::store::StoreApp::new(),
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

                // Initialize Desktop & Editor
                desktop: crate::components::desktop::Desktop::new(),
                editor: crate::apps::editor::EditorApp::new(),
                show_editor: false,
            },
            load_images_task,
        )
    }
}
