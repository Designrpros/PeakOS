// use iced::widget::svg; // Handle not needed here if we store path strings.

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
    Antigravity,
    Spotify,

    WebOS, // The Third Space (Legacy Bridge)

    // Media Apps
    Stremio,
    Vlc,
    Turntable,
}

impl std::fmt::Display for AppId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct AppInfo {
    pub id: AppId,
    pub name: &'static str,
    pub icon_path: &'static str,
}

impl AppInfo {
    pub fn productivity() -> Vec<Self> {
        vec![
            AppInfo {
                id: AppId::Terminal,
                name: "System Console",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/terminal.svg").into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::Browser,
                name: "Netscape",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/browser.svg").into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::Settings,
                name: "Core Sync",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/settings.svg").into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::FileManager,
                name: "File Explorer",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/folder.svg").into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::Store,
                name: "App Store",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/store.svg").into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::AppGrid,
                name: "App Library",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/library.svg").into_boxed_str(),
                ),
            },
        ]
    }

    pub fn library() -> Vec<Self> {
        vec![
            AppInfo {
                id: AppId::Terminal, // Added Terminal to Poolside for retro feel
                name: "Command Line",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/riviera/terminal.png")
                        .into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::Browser,
                name: "Netscape",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/riviera/browser.png")
                        .into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::Settings, // Added Settings to Poolside
                name: "Control Panel",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/riviera/settings.png")
                        .into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::Library,
                name: "The Arcade",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/gamepad.svg").into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::Stremio,
                name: "Stremio",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/riviera/stremio.png")
                        .into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::Vlc,
                name: "VLC Player",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/riviera/vlc.png").into_boxed_str(),
                ),
            },
            AppInfo {
                id: AppId::Turntable,
                name: "The Jukebox",
                icon_path: Box::leak(
                    crate::utils::assets::get_asset_path("icons/riviera/music.png")
                        .into_boxed_str(),
                ),
            },
        ]
    }

    pub fn all() -> Vec<Self> {
        let mut apps = Self::productivity();

        // Add apps not in productivity but available in system
        apps.push(AppInfo {
            id: AppId::Cortex,
            name: "Neural Link",
            icon_path: Box::leak(
                crate::utils::assets::get_asset_path("icons/cpu.svg").into_boxed_str(),
            ),
        });
        apps.push(AppInfo {
            id: AppId::Antigravity,
            name: "Antigravity",
            icon_path: Box::leak(
                crate::utils::assets::get_asset_path("icons/antigravity.svg").into_boxed_str(),
            ),
        });
        apps.push(AppInfo {
            id: AppId::Spotify,
            name: "Spotify",
            icon_path: Box::leak(
                crate::utils::assets::get_asset_path("icons/media.svg").into_boxed_str(),
            ),
        });
        apps.push(AppInfo {
            id: AppId::Library,
            name: "Game Library",
            icon_path: Box::leak(
                crate::utils::assets::get_asset_path("icons/gamepad.svg").into_boxed_str(),
            ),
        });
        apps.push(AppInfo {
            id: AppId::WebOS,
            name: "Web Uplink",
            icon_path: Box::leak(
                crate::utils::assets::get_asset_path("icons/browser.svg").into_boxed_str(),
            ),
        });
        apps.push(AppInfo {
            id: AppId::Stremio,
            name: "Stremio",
            icon_path: Box::leak(
                crate::utils::assets::get_asset_path("icons/riviera/stremio.png").into_boxed_str(),
            ),
        });
        apps.push(AppInfo {
            id: AppId::Vlc,
            name: "VLC Player",
            icon_path: Box::leak(
                crate::utils::assets::get_asset_path("icons/riviera/vlc.png").into_boxed_str(),
            ),
        });
        apps.push(AppInfo {
            id: AppId::Turntable,
            name: "Music / Turntable",
            icon_path: Box::leak(
                crate::utils::assets::get_asset_path("icons/riviera/music.png").into_boxed_str(),
            ),
        });

        apps
    }
}
