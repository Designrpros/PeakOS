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
    pub shell_style: peak_core::registry::ShellStyle, // Visual layout (Cupertino/Redmond/AI)
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
    // browser_process removed - using external Firefox
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
    pub assistant: Option<peak_intelligence::brain::Assistant>,
    pub active_model_id: Option<String>,
    pub pending_chat: Option<String>,
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

        // 1. Boot Assistant if needed (Active Model set, but no assistant)
        // Note: This relies on something triggering the boot state.
        // For simplicity, we can't just "boot if none" because user might not want it running.
        // So we need a "booting" state. But for now, let's assume if we have an active model and pending chat, we ensure boot?
        // Or if we just activated a model.
        // Let's rely on explicit boot via Task in update for now, or use subscription if we want auto-recovery.

        // 2. Chat Reply Subscription
        if let Some(prompt) = &self.pending_chat {
            if let Some(assistant) = &self.assistant {
                subs.push(reply_subscription(assistant.clone(), prompt.clone()));
            } else if let Some(model_id) = &self.active_model_id {
                subs.push(boot_subscription(model_id.clone()));
            }
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
            let file = files.values().flatten().find(|f| {
                f.name.contains("Q4_K_M")
                    || f.name.contains("Q4_0")
                    || f.name.contains("block_medium")
            });

            let file = match file {
                Some(f) => f.clone(),
                None => {
                    // Fallback to first file
                    if let Some(first) = files.values().flatten().next() {
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

            // Find all related shards if it's a sharded model
            let mut shards_to_download = Vec::new();
            let mut total_shards_str = None;
            if file.name.contains("-00001-of-") {
                let base_name = file.name.split("-00001-of-").next().unwrap();
                let suffix = file.name.split("-of-").last().unwrap();
                let total_shards_str_val = suffix.split('.').next().unwrap().to_string();
                if let Ok(total_count) = total_shards_str_val.parse::<u32>() {
                    // Try to find all shards in the original list
                    let all_files: Vec<_> = files.values().flatten().collect();
                    for i in 1..=total_count {
                        let shard_pattern = format!("{:05}-of-{}", i, suffix);
                        if let Some(shard_file) = all_files.iter().find(|f| {
                            f.name.contains(&shard_pattern) && f.name.starts_with(base_name)
                        }) {
                            shards_to_download.push((*shard_file).clone());
                        }
                    }
                }
                total_shards_str = Some(total_shards_str_val);
            }

            let total_shards_label = total_shards_str.as_deref().unwrap_or("1");

            if shards_to_download.is_empty() {
                shards_to_download.push(file);
            }

            let mut total_downloaded: u64 = 0;
            let mut total_size: u64 = shards_to_download
                .iter()
                .filter_map(|s| s.size.as_ref().map(|sz| sz.0))
                .sum();
            if total_size == 0 {
                total_size = 1;
            } // Avoid div by zero

            for (idx, shard) in shards_to_download.into_iter().enumerate() {
                let mut stream = Box::pin(shard.download(&directory));

                output
                    .send(Message::Settings(
                        peak_apps::settings::SettingsMessage::ModelDownloadProgress(
                            id.clone(),
                            total_downloaded as f32 / total_size as f32,
                        ),
                    ))
                    .await
                    .ok();

                while let Some(progress) = stream.next().await {
                    let current_total = total_downloaded + progress.downloaded;
                    output
                        .send(Message::Settings(
                            peak_apps::settings::SettingsMessage::ModelDownloadProgress(
                                id.clone(),
                                current_total as f32 / total_size as f32,
                            ),
                        ))
                        .await
                        .ok();
                }

                if let Ok(path) = stream.await {
                    if let Ok(meta) = std::fs::metadata(path) {
                        total_downloaded += meta.len();
                    }
                } else {
                    output
                        .send(Message::Settings(
                            peak_apps::settings::SettingsMessage::ModelDownloadFailed(
                                id.clone(),
                                format!(
                                    "Failed to download shard {}/{}",
                                    idx + 1,
                                    total_shards_label
                                ),
                            ),
                        ))
                        .await
                        .ok();
                    return;
                }
            }

            output
                .send(Message::Settings(
                    peak_apps::settings::SettingsMessage::ModelDownloadComplete(id.clone()),
                ))
                .await
                .ok();
        }),
    )
}

fn reply_subscription(
    assistant: peak_intelligence::brain::Assistant,
    prompt: String,
) -> iced::Subscription<Message> {
    use iced::futures::StreamExt;

    iced::Subscription::run_with_id(
        format!("chat-{}", prompt),
        iced::stream::channel(100, move |mut output| async move {
            let messages = vec![];
            let mut stream = Box::pin(assistant.reply(prompt, messages, vec![]));

            while let Some((reply, token)) = stream.next().await {
                output
                    .send(Message::AssistantReply(reply, token))
                    .await
                    .ok();
            }

            // Check final result
            if (stream.await).is_err() {
                // Error handling if needed, though we stop anyway
            }

            output.send(Message::AssistantFinished).await.ok();
        }),
    )
}

fn boot_subscription(model_id: String) -> iced::Subscription<Message> {
    use iced::futures::StreamExt;
    use peak_intelligence::brain::model::{File, Model};

    iced::Subscription::run_with_id(
        format!("boot-{}", model_id),
        iced::stream::channel(100, move |mut output| async move {
            // 1. Resolve Model/File (Simplified version of download resolution)
            let Ok(models) = Model::search(model_id.clone()).await else {
                output
                    .send(Message::AssistantBooted(Err(
                        peak_intelligence::brain::Error::DockerFailed("Model Search Failed"),
                    )))
                    .await
                    .ok();
                return;
            };
            let Some(model) = models.first() else {
                output
                    .send(Message::AssistantBooted(Err(
                        peak_intelligence::brain::Error::DockerFailed("Model Not Found"),
                    )))
                    .await
                    .ok();
                return;
            };
            let Ok(files) = File::list(model.id.clone()).await else {
                output
                    .send(Message::AssistantBooted(Err(
                        peak_intelligence::brain::Error::DockerFailed("File List Failed"),
                    )))
                    .await
                    .ok();
                return;
            };

            // Pick best file (same logic as download)
            let file = files.values().flatten().find(|f| {
                f.name.contains("Q4_K_M")
                    || f.name.contains("Q4_0")
                    || f.name.contains("block_medium")
            });
            let file = match file {
                Some(f) => f.clone(),
                None => {
                    if let Some(first) = files.values().flatten().next() {
                        first.clone()
                    } else {
                        output
                            .send(Message::AssistantBooted(Err(
                                peak_intelligence::brain::Error::DockerFailed("No File Found"),
                            )))
                            .await
                            .ok();
                        return;
                    }
                }
            };

            let directory = peak_intelligence::brain::model::Directory::default();

            // Blindly use CPU for safety on all platforms for now unless we know better
            let backend = peak_intelligence::brain::assistant::Backend::Cpu;

            let mut straw = Box::pin(peak_intelligence::brain::Assistant::boot(
                directory, file, backend,
            ));

            while let Some(event) = straw.next().await {
                match event {
                    peak_intelligence::brain::assistant::BootEvent::Progressed {
                        stage: _,
                        percent: _,
                    } => {
                        // Ignore progress for now
                    }
                    peak_intelligence::brain::assistant::BootEvent::Logged(_) => {}
                }
            }

            // Result should be the Assistant
            match straw.await {
                Ok(assistant) => {
                    output
                        .send(Message::AssistantBooted(Ok(assistant)))
                        .await
                        .ok();
                }
                Err(e) => {
                    output.send(Message::AssistantBooted(Err(e))).await.ok();
                }
            }
        }),
    )
}
