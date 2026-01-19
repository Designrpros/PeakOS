// use iced::widget::svg; // Handle not needed here if we store path strings.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellMode {
    Desktop,
    Auto,
    Console,
    Fireplace,
    Kiosk,
    Mobile,
    Robot,
    Server,
    SmartHome,
    TV,
}

impl std::fmt::Display for ShellMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellMode::Desktop => write!(f, "Desktop"),
            ShellMode::Auto => write!(f, "Auto"),
            ShellMode::Console => write!(f, "Console"),
            ShellMode::Fireplace => write!(f, "Fireplace"),
            ShellMode::Kiosk => write!(f, "Kiosk"),
            ShellMode::Mobile => write!(f, "Mobile"),
            ShellMode::Robot => write!(f, "Robot"),
            ShellMode::Server => write!(f, "Server"),
            ShellMode::SmartHome => write!(f, "Home"),
            ShellMode::TV => write!(f, "TV"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppId {
    Terminal,
    Browser, // The Tauri Guest
    Library, // Internal View
    Cortex,  // Internal View
    Settings,
    FileManager,
    Store,
    AppGrid, // The Launchpad / App Library
    Editor,
    Desktop,
    #[allow(dead_code)]
    Spotify,

    // Media Apps
    Turntable,
}

impl AppId {
    pub fn is_repo(&self) -> bool {
        match self {
            AppId::Desktop | AppId::FileManager => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for AppId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for AppId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Terminal" => Ok(AppId::Terminal),
            "Browser" => Ok(AppId::Browser),
            "Library" => Ok(AppId::Library),
            "Cortex" => Ok(AppId::Cortex),
            "Settings" => Ok(AppId::Settings),
            "FileManager" => Ok(AppId::FileManager),
            "Store" => Ok(AppId::Store),
            "AppGrid" => Ok(AppId::AppGrid),
            "Editor" => Ok(AppId::Editor),
            "Desktop" => Ok(AppId::Desktop),
            "Spotify" => Ok(AppId::Spotify),
            "Turntable" => Ok(AppId::Turntable),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppInfo {
    pub id: AppId,
    pub name: &'static str,
}

impl AppInfo {
    pub fn get_info(id: AppId) -> Self {
        let name = match id {
            AppId::Terminal => "Terminal",
            AppId::Browser => "Netscape",
            AppId::Library => "Arcade",
            AppId::Cortex => "Neural Link",
            AppId::Settings => "Settings",
            AppId::FileManager => "Files",
            AppId::Store => "Store",
            AppId::AppGrid => "Launchpad",
            AppId::Editor => "Text Editor",
            AppId::Desktop => "Desktop",
            AppId::Spotify => "Spotify",
            AppId::Turntable => "Jukebox",
        };
        Self { id, name }
    }

    #[allow(dead_code)]
    pub fn dock() -> Vec<Self> {
        vec![
            AppId::Terminal,
            AppId::Browser,
            AppId::Turntable,
            AppId::Library,
            AppId::FileManager,
            AppId::Store,
            AppId::Settings,
            AppId::AppGrid,
        ]
        .into_iter()
        .map(Self::get_info)
        .collect()
    }

    pub fn all() -> Vec<Self> {
        vec![
            AppId::Terminal,
            AppId::Browser,
            AppId::Library,
            AppId::Cortex,
            AppId::Settings,
            AppId::FileManager,
            AppId::Store,
            AppId::AppGrid,
            AppId::Editor,
            AppId::Desktop,
            AppId::Turntable,
        ]
        .into_iter()
        .map(Self::get_info)
        .collect()
    }

    pub fn all_as_media() -> Vec<crate::models::MediaItem> {
        Self::all()
            .into_iter()
            .map(|info| crate::models::MediaItem {
                id: info.id.to_string(),
                title: info.name.to_string(),
                cover_image: format!("{}_icon.png", info.id.to_string().to_lowercase()),
                launch_command: info.id.to_string(),
                kind: crate::models::MediaKind::App,
                status: crate::models::MediaStatus::Ready,
                image_handle: None,
            })
            .collect()
    }
}
