// PeakNative application module
// Refactored from monolithic app.rs into focused sub-modules

use crate::apps::explorer::ExplorerApp;
use crate::apps::library::LibraryApp;
use crate::apps::settings::SettingsApp;
use crate::apps::terminal::TerminalApp;
use crate::components::app_switcher::AppSwitcher;
use crate::components::inspector::Inspector;
use crate::components::omnibar::Omnibar;
use crate::pages::Page;
use iced::Theme as IcedTheme;
use peak_core::models::MediaItem;
use peak_core::registry::ShellMode;
use sysinfo::System;

// Sub-modules
mod init;
pub mod state;
mod update;
mod view;
mod view_desktop;
mod window_handling;

// Re-export public types
pub use state::{AppState, Message};

// Main application struct
pub struct PeakNative {
    pub state: AppState,
    pub user: Option<crate::apps::auth::UserProfile>,

    pub theme: peak_core::theme::Theme,
    pub current_page: Page,
    pub games: Vec<MediaItem>,
    pub library: LibraryApp,
    pub cortex_state: crate::pages::cortex::State,
    pub mode: ShellMode,
    pub terminal: TerminalApp,
    pub settings: SettingsApp,
    pub jukebox: crate::apps::jukebox::JukeboxApp,
    pub explorer: ExplorerApp,
    pub store: crate::apps::store::StoreApp,
    pub inspector: Inspector,
    pub show_settings: bool,
    // Spotlight / Omnibar
    pub omnibar: Omnibar,
    pub show_omnibar: bool,
    // Neural Link (System Monitoring)
    pub system: System,
    pub last_monitor_update: std::time::Instant,
    pub show_spaces_selector: bool,
    pub current_desktop: usize,
    pub switcher: AppSwitcher,
    pub show_switcher: bool,
    pub(crate) _stream: Option<rodio::OutputStream>, // Must keep alive (if audio available)

    // Window Management
    pub window_manager: crate::systems::window_manager::WindowManager,
    pub _show_app_library: bool,
    pub cursor_position: iced::Point,
    pub dock_visible: bool,
    pub show_system_menu: bool,
    pub show_reality_menu: bool,
    pub show_wifi_menu: bool,
    pub networks: Vec<String>,
    pub vector_bg: crate::components::vector_background::VectorBackground,
    pub show_app_grid: bool,
    pub browser_process: Option<std::process::Child>,
    pub window_position: iced::Point,
    pub is_polling_window: bool,
    pub polling_attempts: usize,
    pub alert: Option<(String, String)>,

    // Advanced Dock State
    pub pinned_apps: Vec<peak_core::registry::AppId>,
    pub running_apps: Vec<peak_core::registry::AppId>,
    pub dragging_app: Option<(peak_core::registry::AppId, usize)>, // (AppId, original_index)
    pub context_menu_app: Option<peak_core::registry::AppId>,

    // Desktop & Editor
    pub desktop: crate::components::desktop::Desktop,
    pub editor: crate::apps::editor::EditorApp,
    pub show_editor: bool,
    pub is_desktop_revealed: bool,
    pub quick_look_path: Option<std::path::PathBuf>,
    pub tracked_modifiers: iced::keyboard::Modifiers,
    pub is_mouse_button_pressed: bool, // Track left mouse button state for reliable drag release
}

impl PeakNative {
    pub fn title(&self) -> String {
        String::from("Peak")
    }

    pub fn theme(&self) -> IcedTheme {
        self.theme.as_iced()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        use iced::time;

        let subs = iced::Subscription::batch(vec![
            iced::event::listen().map(Message::GlobalEvent),
            time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick),
        ]);

        if self.terminal.is_open {
            iced::Subscription::batch(vec![
                subs,
                self.terminal.subscription().map(Message::Terminal),
            ])
        } else {
            subs
        }
    }
}
