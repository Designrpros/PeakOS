use crate::app_traits::{PeakApp, ShellContext};
use crate::theme::Theme;
use iced::{Element, Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeMode::Light => write!(f, "Light"),
            ThemeMode::Dark => write!(f, "Dark"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    General,
    WiFi,
    Bluetooth,
    Battery,
    Appearance,
    Display,
    Sound,
    Focus,
    Privacy,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ThemeChanged(ThemeMode),
    VolumeChanged(f32),
    TabChanged(SettingsTab),
    SearchChanged(String),
    ToggleWiFi(bool),
    ToggleBluetooth(bool),
    WallpaperChanged(String),
}

pub struct SettingsApp {
    pub theme_mode: ThemeMode,
    pub current_tab: SettingsTab,
    pub volume: f32,
    pub search_query: String,
    pub wifi_enabled: bool,
    pub bluetooth_enabled: bool,
    pub wallpapers: Vec<String>,
    pub current_wallpaper: String,
}

impl SettingsApp {
    pub fn new() -> Self {
        Self {
            theme_mode: ThemeMode::Light,
            current_tab: SettingsTab::General,
            volume: 0.8,
            search_query: String::new(),
            wifi_enabled: true,
            bluetooth_enabled: true,
            wallpapers: Vec::new(),
            current_wallpaper: String::from("Peak.png"),
        }
    }
}

impl PeakApp for SettingsApp {
    type Message = SettingsMessage;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        match message {
            SettingsMessage::ThemeChanged(mode) => {
                self.theme_mode = mode;
            }
            SettingsMessage::VolumeChanged(v) => {
                self.volume = v;
            }
            SettingsMessage::TabChanged(tab) => {
                self.current_tab = tab;
            }
            SettingsMessage::SearchChanged(q) => {
                self.search_query = q;
            }
            SettingsMessage::ToggleWiFi(enabled) => {
                self.wifi_enabled = enabled;
            }
            SettingsMessage::ToggleBluetooth(enabled) => {
                self.bluetooth_enabled = enabled;
            }
            SettingsMessage::WallpaperChanged(path) => {
                self.current_wallpaper = path;
            }
        }
        Task::none()
    }

    fn view(&self, _theme: &Theme) -> Element<'_, Self::Message> {
        iced::widget::text("Settings View (Stub)").into()
    }
}
