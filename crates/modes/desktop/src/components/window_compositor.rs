use crate::app::Message;
use iced::widget::{container, stack, text};
use iced::{padding, Element};

use peak_ui::window_chrome;

pub struct WindowCompositor<'a> {
    window_manager: &'a crate::systems::window_manager::WindowManager,
    registry: &'a crate::systems::registry::AppRegistry,
    theme: &'a peak_core::theme::Theme,
    mode: peak_core::registry::ShellMode,
    current_desktop: usize,
    is_desktop_revealed: bool,
    screen_size: iced::Size,
    is_light: bool,
}

impl<'a> WindowCompositor<'a> {
    pub fn new(
        window_manager: &'a crate::systems::window_manager::WindowManager,
        registry: &'a crate::systems::registry::AppRegistry,
        theme: &'a peak_core::theme::Theme,
        mode: peak_core::registry::ShellMode,
        current_desktop: usize,
        is_desktop_revealed: bool,
    ) -> Self {
        let screen_size = window_manager.screen_size;
        let is_light = matches!(theme, peak_core::theme::Theme::Light);
        Self {
            window_manager,
            registry,
            theme,
            mode,
            current_desktop,
            is_desktop_revealed,
            screen_size,
            is_light,
        }
    }

    pub fn view(self, base_layer: Element<'a, Message>) -> Element<'a, Message> {
        let mut stack = stack![base_layer];

        for &app_id in &self.window_manager.z_order {
            if let Some(state) = self.window_manager.window_states.get(&app_id) {
                // Workspace Filtering
                if !state.is_sticky
                    && (state.reality != self.mode || state.desktop_idx != self.current_desktop)
                {
                    continue;
                }

                let content: Element<'_, Message> =
                    if let Some(app) = self.registry.running_apps.get(&app_id) {
                        app.view(&self.theme)
                    } else {
                        container(text("UNSUPPORTED")).into()
                    };

                let title = if let Some(all_app) = self.registry.running_apps.get(&app_id) {
                    all_app.title()
                } else {
                    Self::fallback_title(app_id)
                };

                let on_close = Self::close_message(app_id);
                // CloseBrowser special handling logic is in view_desktop, maybe move here?
                let on_close = match app_id {
                    peak_core::registry::AppId::Browser => Message::CloseBrowser,
                    _ => on_close,
                };

                let mut win_x = state.x;
                let mut win_y = state.y.max(40.0); // Safe guard for menubar

                if self.is_desktop_revealed {
                    let screen_center_x = self.screen_size.width / 2.0;
                    win_y = -state.height + 60.0;
                    win_x = screen_center_x - (state.width / 2.0);

                    if let Some(pos) = self
                        .window_manager
                        .z_order
                        .iter()
                        .position(|&id| id == app_id)
                    {
                        let offset = pos as f32 * 4.0;
                        win_x += offset;
                        win_y += offset;
                    }
                }

                stack = stack.push(
                    container(
                        container(window_chrome::view(
                            title,
                            content,
                            on_close,
                            Some(Message::Maximize(app_id)),
                            None,
                            self.is_light,
                        ))
                        .width(state.width)
                        .height(state.height),
                    )
                    .padding(padding::Padding {
                        top: win_y,
                        left: win_x,
                        ..Default::default()
                    }),
                );
            }
        }

        stack.into()
    }

    fn fallback_title(app_id: peak_core::registry::AppId) -> String {
        use peak_core::registry::AppId;
        match app_id {
            AppId::Terminal => "System Console".to_string(),
            AppId::Library => "The Arcade".to_string(),
            AppId::Turntable => "The Jukebox".to_string(),
            AppId::Settings => "Core Sync".to_string(),
            AppId::FileManager => "File System".to_string(),
            AppId::Store => "App Store".to_string(),
            AppId::Editor => "Simple Text".to_string(),
            AppId::Browser => "Netscape Navigator".to_string(),
            _ => "Application".to_string(),
        }
    }

    fn close_message(app_id: peak_core::registry::AppId) -> Message {
        use peak_core::registry::AppId;
        match app_id {
            AppId::Terminal => Message::ToggleTerminal,
            AppId::Library => Message::ToggleArcade,
            AppId::Turntable => Message::ToggleJukebox,
            AppId::Settings => Message::ToggleSettings,
            AppId::FileManager => Message::ToggleExplorer,
            AppId::Store => Message::ToggleStore,
            AppId::Editor => Message::ToggleEditor,
            AppId::Browser => Message::LaunchBrowser("about:blank".into()),
            _ => Message::Exit,
        }
    }
}
