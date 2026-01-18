// PeakNative application module
// Refactored from monolithic app.rs into focused sub-modules

use crate::components::inspector::Inspector;
use crate::components::omnibar::Omnibar;
use crate::pages::Page;
use iced::futures::SinkExt;
use iced::Theme as IcedTheme;
use peak_shell::app_switcher::AppSwitcher;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LaunchMode {
    #[default]
    Desktop,
    Bar,
    Dock,
}

#[derive(Debug, Clone, Default)]
pub struct PeakNativeFlags {
    pub mode: String,
    pub launch_mode: LaunchMode,
}

// Main application struct
pub struct PeakNative {
    pub state: AppState,
    pub user: Option<peak_apps::auth::UserProfile>,

    pub theme: peak_core::theme::Theme,
    pub current_page: Page,
    pub games: Vec<MediaItem>, // Keep for now as source of truth? Or remove if hydration is enough. Keep for hydration.
    pub cortex_state: crate::pages::cortex::State,
    pub mode: ShellMode,
    pub launch_mode: LaunchMode,
    pub custom_wallpaper: Option<String>,

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

    // Registry & Window Management
    pub registry: crate::systems::registry::AppRegistry,
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

    // Desktop
    pub desktop: crate::components::desktop::Desktop,
    pub show_editor: bool,
    pub is_desktop_revealed: bool,
    pub quick_look_path: Option<std::path::PathBuf>,
    pub tracked_modifiers: iced::keyboard::Modifiers,
    pub is_mouse_button_pressed: bool, // Track left mouse button state for reliable drag release
    pub scanned_apps: Vec<MediaItem>,
    pub tokens: peak_theme::ThemeTokens,
    pub active_downloads: std::collections::HashSet<String>,
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

        let mut subs = vec![
            iced::event::listen().map(Message::GlobalEvent),
            time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick),
        ];

        // Add subscriptions from modular apps - only if visible (Resource Throttling)
        for (id, app) in &self.registry.running_apps {
            let is_visible = self
                .window_manager
                .window_states
                .get(id)
                .map(|ws| {
                    (ws.desktop_idx == self.current_desktop && ws.reality == self.mode)
                        || ws.is_sticky
                })
                .unwrap_or(false);

            if is_visible {
                subs.push(app.subscription());
            }
        }

        // Inspector subscription
        subs.push(self.inspector.subscription().map(Message::Inspector));

        // Check for active downloads
        for id in &self.active_downloads {
            subs.push(download_model_subscription(id.clone()));
        }

        iced::Subscription::batch(subs)
    }
}

fn download_model_subscription(id: String) -> iced::Subscription<Message> {
    use iced::futures::StreamExt;
    use peak_intelligence::brain::model::{File, Model};

    iced::Subscription::run_with_id(
        id.clone(),
        iced::stream::channel(100, move |mut output| async move {
            // Search for model
            let Ok(models) = Model::search(id.clone()).await else {
                output
                    .send(Message::Settings(
                        peak_apps::settings::SettingsMessage::ModelDownloadFailed(
                            id.clone(),
                            "Search failed".into(),
                        ),
                    ))
                    .await
                    .ok();
                return;
            };

            let Some(model) = models.first() else {
                output
                    .send(Message::Settings(
                        peak_apps::settings::SettingsMessage::ModelDownloadFailed(
                            id.clone(),
                            "Model not found".into(),
                        ),
                    ))
                    .await
                    .ok();
                return;
            };

            // List files
            let Ok(files) = File::list(model.id.clone()).await else {
                output
                    .send(Message::Settings(
                        peak_apps::settings::SettingsMessage::ModelDownloadFailed(
                            id.clone(),
                            "File list failed".into(),
                        ),
                    ))
                    .await
                    .ok();
                return;
            };

            // Pick best available quantization (Q4_K_M is a good default for Apple Silicon)
            let file = files.values().flat_map(|v| v).find(|f| {
                f.name.contains("Q4_K_M")
                    || f.name.contains("Q4_0")
                    || f.name.contains("block_medium")
            });

            let file = match file {
                Some(f) => f.clone(),
                None => {
                    // Fallback to first file
                    if let Some(first) = files.values().flat_map(|v| v).next() {
                        first.clone()
                    } else {
                        output
                            .send(Message::Settings(
                                peak_apps::settings::SettingsMessage::ModelDownloadFailed(
                                    id.clone(),
                                    "No GGUF file found".into(),
                                ),
                            ))
                            .await
                            .ok();
                        return;
                    }
                }
            };

            let directory = peak_intelligence::brain::model::Directory::default();
            let mut stream = Box::pin(file.download(&directory));

            output
                .send(Message::Settings(
                    peak_apps::settings::SettingsMessage::ModelDownloadProgress(id.clone(), 0.0),
                ))
                .await
                .ok();

            while let Some(progress) = stream.next().await {
                let total = progress.total.unwrap_or(progress.downloaded.max(1));
                let percent = progress.downloaded as f32 / total as f32;
                output
                    .send(Message::Settings(
                        peak_apps::settings::SettingsMessage::ModelDownloadProgress(
                            id.clone(),
                            percent,
                        ),
                    ))
                    .await
                    .ok();
            }

            match stream.await {
                Ok(_) => {
                    output
                        .send(Message::Settings(
                            peak_apps::settings::SettingsMessage::ModelDownloadComplete(id.clone()),
                        ))
                        .await
                        .ok();
                }
                Err(e) => {
                    output
                        .send(Message::Settings(
                            peak_apps::settings::SettingsMessage::ModelDownloadFailed(
                                id.clone(),
                                e.to_string(),
                            ),
                        ))
                        .await
                        .ok();
                }
            }
        }),
    )
}
