// Message handling logic

use super::{AppState, Message, PeakNative};
use crate::components::omnibar::OmnibarMessage;
use crate::pages::Page;
use iced::Task;
use peak_apps::library::LibraryMessage;
#[allow(unused_imports)]
use peak_core::app_traits::PeakApp;
use peak_core::registry::{AppId, ShellMode};
use peak_shell::{app_switcher::SwitcherMessage, dock, menubar::MenubarMessage};

impl PeakNative {
    /// Helper to toggle an app window - handles workspace switching and window lifecycle
    fn toggle_app(
        &mut self,
        app_id: AppId,
        default_width: f32,
        default_height: f32,
    ) -> Task<Message> {
        self.is_desktop_revealed = false;

        if let Some(state) = self.window_manager.window_states.get(&app_id) {
            // If window exists but is on different workspace, switch to it
            if state.reality != self.mode || state.desktop_idx != self.current_desktop {
                self.mode = state.reality;
                self.current_desktop = state.desktop_idx;
                self.window_manager.bring_to_front(app_id);
            } else {
                // Otherwise close it
                self.close_window(app_id);
            }
        } else {
            // Create new window
            self.ensure_window_state(app_id, default_width, default_height);
        }

        Task::none()
    }

    // Helper for registry dispatch
    fn forward_to_app(&mut self, app_id: AppId, message: Message) -> Task<Message> {
        if let Some(app) = self.registry.running_apps.get_mut(&app_id) {
            let context = crate::systems::registry::DesktopShellContext::new(
                app_id,
                (self.window_position.x, self.window_position.y),
            );
            app.update(message, &context)
        } else {
            Task::none()
        }
    }

    pub fn update_style_from_mode(&mut self) {
        match self.mode {
            ShellMode::Console => {
                self.shell_style = peak_core::registry::ShellStyle::Console;
                self.theme = peak_core::theme::Theme::Dark;
            }
            ShellMode::TV => {
                self.shell_style = peak_core::registry::ShellStyle::TV;
                self.theme = peak_core::theme::Theme::Dark;
            }
            ShellMode::Desktop => {
                self.shell_style = peak_core::registry::ShellStyle::Cupertino;
                self.theme = peak_core::theme::Theme::Light;
            }
            ShellMode::Mobile => {
                self.shell_style = peak_core::registry::ShellStyle::AI;
                self.theme = peak_core::theme::Theme::Light;
            }
            _ => {
                // Default fallback
                self.shell_style = peak_core::registry::ShellStyle::Cupertino;
                self.theme = peak_core::theme::Theme::Light;
            }
        }
        self.update_tokens();
    }

    pub fn update_tokens(&mut self) {
        let tone = match self.theme {
            peak_core::theme::Theme::Light => peak_theme::ThemeTone::Light,
            peak_core::theme::Theme::Dark => peak_theme::ThemeTone::Dark,
            _ => peak_theme::ThemeTone::Light,
        };
        self.tokens = peak_theme::ThemeTokens::get(self.mode, tone);
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowPositionFound(pos_opt) => {
                self.is_polling_window = false;
                if let Some(pos) = pos_opt {
                    self.window_position = pos;
                } else {
                    self.polling_attempts += 1;
                    if self.polling_attempts >= 5 && self.alert.is_none() {
                        self.alert = Some((
                            "Window Sync Failed".to_string(), 
                            "Could not detect window position. Please grant 'Automation' permission to your terminal in System Settings -> Privacy & Security -> Automation.".to_string()
                        ));
                    }
                }
                Task::none()
            }
            Message::CloseAlert => {
                self.alert = None;
                Task::none()
            }
            Message::GlobalEvent(event) => {
                // Window Resize & Move
                if let iced::Event::Window(window_event) = &event {
                    match window_event {
                        iced::window::Event::Resized(size) => {
                            self.window_manager.screen_size =
                                iced::Size::new(size.width, size.height);
                            // Browser sync removed - using Firefox instead
                        }
                        iced::window::Event::Moved(point) => {
                            self.window_position = *point;
                            // Browser sync removed - using Firefox instead
                        }
                        _ => {}
                    }
                }

                // Track mouse button state for internal logic if needed
                if let iced::Event::Mouse(mouse_event) = &event {
                    match mouse_event {
                        iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left) => {
                            self.is_mouse_button_pressed = true;
                        }
                        iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left) => {
                            self.is_mouse_button_pressed = false;
                        }
                        _ => {}
                    }
                }

                // Definitive release handler - this MUST clear dragging
                if matches!(
                    event,
                    iced::Event::Mouse(iced::mouse::Event::ButtonReleased(
                        iced::mouse::Button::Left
                    ))
                ) {
                    self.window_manager.dragging = None;
                    self.dragging_app = None;
                    self.desktop
                        .update(crate::components::desktop::DesktopMessage::EndSelection);
                }

                // Mouse Events for Dragging & Focus
                match event {
                    iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                        iced::mouse::Button::Left,
                    )) => {
                        let mut focused_app: Option<peak_core::registry::AppId> = None;

                        // Hit-test system overlays first
                        let inspector_visible = self.inspector.is_visible;
                        let is_over_inspector = inspector_visible
                            && self.cursor_position.x
                                > self.window_manager.screen_size.width - 360.0;
                        let is_over_menubar = self.cursor_position.y < 32.0;

                        if is_over_inspector || is_over_menubar {
                            self.window_manager.dragging = None;
                            self.dragging_app = None;
                        } else {
                            for &app_id in self.window_manager.z_order.iter().rev() {
                                if let Some(state) = self.window_manager.window_states.get(&app_id)
                                {
                                    let rect = iced::Rectangle {
                                        x: state.x,
                                        y: state.y,
                                        width: state.width,
                                        height: state.height,
                                    };

                                    if rect.contains(self.cursor_position) {
                                        focused_app = Some(app_id);
                                        let title_bar_rect = iced::Rectangle {
                                            x: state.x,
                                            y: state.y,
                                            width: state.width,
                                            height: 40.0,
                                        };
                                        if title_bar_rect.contains(self.cursor_position) {
                                            self.window_manager.dragging = Some((
                                                app_id,
                                                iced::Point::new(
                                                    self.cursor_position.x - state.x,
                                                    self.cursor_position.y - state.y,
                                                ),
                                            ));
                                        }
                                        break;
                                    }
                                }
                            }
                        }

                        if let Some(app_id) = focused_app {
                            self.is_desktop_revealed = false;
                            if let Some(pos) = self
                                .window_manager
                                .z_order
                                .iter()
                                .position(|&id| id == app_id)
                            {
                                self.window_manager.z_order.remove(pos);
                                self.window_manager.z_order.push(app_id);
                            }
                        } else {
                            // Clicked background or icons
                            let menubar_h = 32.0;
                            let dock_h = 100.0;
                            let screen_h = self.window_manager.screen_size.height;

                            let is_on_menubar = self.cursor_position.y < menubar_h;
                            let is_on_dock = self.cursor_position.y > screen_h - dock_h;

                            if !is_on_menubar && !is_on_dock {
                                // If we're NOT over an icon, start marquee selection
                                if !self.desktop.is_over_icon(self.cursor_position) {
                                    self.is_desktop_revealed = true;
                                    self.desktop.update(
                                        crate::components::desktop::DesktopMessage::StartSelection(
                                            self.cursor_position,
                                            self.tracked_modifiers,
                                        ),
                                    );
                                } else {
                                    // Clicked an icon - reveal mode should probably still trigger or stay
                                    // but selection is handled by the icon component itself.
                                }
                            } else {
                                self.is_desktop_revealed = false;
                            }
                        }
                    }
                    iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                        self.cursor_position = position;

                        // Selection update
                        if self.desktop.drag_start.is_some() {
                            self.desktop.update(
                                crate::components::desktop::DesktopMessage::UpdateSelection(
                                    position,
                                ),
                            );
                        }

                        // Dock Auto-Hide Logic
                        let threshold_bottom = 80.0;
                        let threshold_hide = 150.0;
                        if position.y > self.window_manager.screen_size.height - threshold_bottom {
                            self.dock_visible = true;
                        } else if position.y
                            < self.window_manager.screen_size.height - threshold_hide
                        {
                            self.dock_visible = false;
                        }

                        if let Some((app_id, offset)) = self.window_manager.dragging {
                            if let Some(state) = self.window_manager.window_states.get_mut(&app_id)
                            {
                                // Note: position is already in workspace coordinates (which is what we want)
                                // No adjustment needed because windows are drawn in the workspace
                                state.x = position.x - offset.x;
                                state.y = position.y - offset.y;

                                // Constraint: Don't drag above menubar
                                if state.y < 32.0 {
                                    state.y = 32.0;
                                }

                                // Browser sync removed - using Firefox instead
                            }
                        } else {
                            // If we were dragging and it stopped, we can log it here if needed
                        }
                    }
                    iced::Event::Keyboard(iced::keyboard::Event::ModifiersChanged(modifiers)) => {
                        self.tracked_modifiers = modifiers;
                    }
                    _ => {}
                }

                // Keyboard Events (using reference to avoid move)
                if let iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key,
                    modifiers,
                    ..
                }) = &event
                {
                    // Omnibar Navigation (when open) - Handle first
                    if self.show_omnibar {
                        match key {
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                                return Task::done(Message::Omnibar(
                                    crate::components::omnibar::OmnibarMessage::NavigateUp,
                                ));
                            }
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                                return Task::done(Message::Omnibar(
                                    crate::components::omnibar::OmnibarMessage::NavigateDown,
                                ));
                            }
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => {
                                return Task::done(Message::Omnibar(
                                    crate::components::omnibar::OmnibarMessage::Submit,
                                ));
                            }
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) => {
                                self.window_manager.dragging = None; // Reset dragging on Escape
                                if self.quick_look_path.is_some() {
                                    self.quick_look_path = None;
                                    return Task::none();
                                }
                                return Task::done(Message::Omnibar(
                                    crate::components::omnibar::OmnibarMessage::Cancel,
                                ));
                            }
                            _ => {}
                        }
                    }

                    // Magnet Snapping (Option + Arrows)
                    if modifiers.alt() {
                        let top_app = self.window_manager.z_order.last().cloned();
                        if let Some(app_id) = top_app {
                            self.handle_snapping(app_id, key.clone());
                        }
                    }

                    // Omnibar (Cmd/Alt/Ctrl + Space)
                    if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) = key {
                        if modifiers.command() || modifiers.control() || modifiers.alt() {
                            return Task::done(Message::ToggleOmnibar);
                        }
                    }

                    // Quick Look (Space)
                    if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) = key {
                        if !modifiers.command() && !modifiers.control() && !modifiers.alt() {
                            if let Some(path) = self.desktop.selected.first() {
                                self.quick_look_path = Some(path.clone());
                            }
                        }
                    }

                    // Switcher (Cmd/Alt + Tab)
                    if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) = key {
                        if modifiers.command() || modifiers.alt() {
                            if !self.show_switcher {
                                // Populate switcher with running apps from z_order (reverse for MRU)
                                let running_apps: Vec<peak_core::registry::AppInfo> = self
                                    .window_manager
                                    .z_order
                                    .iter()
                                    .rev()
                                    .map(|&id| peak_core::registry::AppInfo::get_info(id))
                                    .collect();

                                if !running_apps.is_empty() {
                                    self.switcher.apps = running_apps;
                                    self.switcher.selected_index = 0;
                                    self.show_switcher = true;
                                    // macOS Cmd+Tab starts at the SECOND item (last used)
                                    if self.switcher.apps.len() > 1 {
                                        self.switcher.next();
                                    }
                                }
                            } else if modifiers.shift() {
                                self.switcher.prev();
                            } else {
                                self.switcher.next();
                            }
                        }
                    }

                    // Wizard Navigation (Enter/Space on Welcome screen)
                    if matches!(self.state, AppState::Setup(_)) {
                        match key {
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter)
                            | iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
                                if let AppState::Setup(ref wizard_state) = self.state {
                                    if wizard_state.current_step
                                        == peak_apps::wizard::WizardStep::Welcome
                                    {
                                        return Task::done(Message::Wizard(
                                            peak_apps::wizard::WizardMessage::NextStep,
                                        ));
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                // Close Switcher on Modifiers Released
                if let iced::Event::Keyboard(iced::keyboard::Event::KeyReleased { key, .. }) = event
                {
                    if self.show_switcher {
                        match key {
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Super)
                            | iced::keyboard::Key::Named(iced::keyboard::key::Named::Alt)
                            | iced::keyboard::Key::Named(iced::keyboard::key::Named::Control) => {
                                // Finalize selection
                                let app_to_activate = self
                                    .switcher
                                    .apps
                                    .get(self.switcher.selected_index)
                                    .map(|a| a.id);

                                if let Some(app_id) = app_to_activate {
                                    self.window_manager.bring_to_front(app_id);
                                }
                                self.show_switcher = false;
                            }
                            _ => {}
                        }
                    }
                }
                Task::none()
            }
            Message::Tick => {
                // Check for button state transition on every tick
                // This catches releases that never generated ButtonReleased events (consumed by Inspector)
                if self.is_mouse_button_pressed {
                    // Button was pressed last frame, check if still pressed by looking for recent events
                    // If we haven't seen a matching release event but dragging is active,
                    // we rely on the GlobalEvent handler. But if the event was consumed,
                    // we need to poll the actual state.

                    // For now, we just rely on the event-based tracking in GlobalEvent
                    // But we could add platform-specific polling here if needed
                }

                // Poll window position if we think we are at 0,0 (likely startup)
                // We now rely on iced::window::Event::Moved for passive sync
                // to avoid freezing the main thread with osascript.

                if self.last_monitor_update.elapsed() > std::time::Duration::from_millis(500) {
                    // Diagnostic: Disable sysinfo refresh to see if it fixes the freeze
                    // self.system.refresh_cpu();
                    // self.system.refresh_memory();

                    let cpu = self.system.global_cpu_info().cpu_usage();
                    let mem_used =
                        self.system.used_memory() as f32 / self.system.total_memory() as f32;

                    self.cortex_state.push_data(cpu, mem_used);
                    self.last_monitor_update = std::time::Instant::now();
                }
                Task::none()
            }
            Message::Exit => Task::none(),
            Message::LaunchBrowser(url) => {
                // Use system browser (Firefox on PeakOS ISO)
                let _ = opener::open(&url);
                Task::none()
            }
            Message::CloseBrowser => {
                // No-op since we now use external Firefox
                Task::none()
            }
            Message::Maximize(app_id) => {
                if let Some(state) = self.window_manager.window_states.get_mut(&app_id) {
                    let menubar_h = 40.0;
                    // We leave space for the dock? User said "adjusted for menubar".
                    // Fullscreen usually ignores dock or keeps it.
                    // Let's maximize to available space minus menubar.
                    let dock_h = if self.dock_visible { 100.0 } else { 0.0 };

                    state.x = 0.0;
                    state.y = menubar_h;
                    state.width = self.window_manager.screen_size.width;
                    state.height = self.window_manager.screen_size.height - menubar_h - dock_h;
                }
                Task::none()
            }
            Message::LaunchGame(cmd) => {
                #[cfg(target_os = "macos")]
                {
                    if cmd.starts_with("steam://") {
                        std::process::Command::new("open").arg(&cmd).spawn().ok();
                    } else {
                        std::process::Command::new("sh")
                            .arg("-c")
                            .arg(&cmd)
                            .spawn()
                            .ok();
                    }
                }
                #[cfg(not(target_os = "macos"))]
                {
                    let mut process_cmd = if cmd.starts_with("steam://") {
                        let mut c = std::process::Command::new("xdg-open");
                        c.arg(&cmd);
                        c
                    } else {
                        let mut c = std::process::Command::new("sh");
                        c.arg("-c").arg(&cmd);
                        c
                    };
                    // Propagate Wayland environment
                    if let Ok(display) = std::env::var("WAYLAND_DISPLAY") {
                        process_cmd.env("WAYLAND_DISPLAY", display);
                    }
                    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
                        process_cmd.env("XDG_RUNTIME_DIR", runtime_dir);
                    }
                    process_cmd.env("GDK_BACKEND", "wayland");
                    process_cmd.spawn().ok();
                }
                Task::none()
            }
            Message::ToggleTheme => {
                self.theme = match self.theme {
                    peak_core::theme::Theme::Light => peak_core::theme::Theme::Dark,
                    peak_core::theme::Theme::Dark => peak_core::theme::Theme::Light,
                    _ => peak_core::theme::Theme::Light,
                };
                self.update_tokens();
                Task::none()
            }
            Message::Navigate(page) => {
                self.current_page = page;
                Task::none()
            }
            Message::DockInteraction(dock_msg) => {
                match dock_msg {
                    dock::DockMessage::LaunchMedia(item) => {
                        self.show_app_grid = false;
                        self.is_desktop_revealed = false;

                        let command = item.launch_command.clone();
                        #[cfg(target_os = "macos")]
                        {
                            if command.contains("/Applications/") || command.contains("open ") {
                                std::process::Command::new("sh")
                                    .arg("-c")
                                    .arg(&command)
                                    .spawn()
                                    .ok();
                            } else {
                                std::process::Command::new(command).spawn().ok();
                            }
                        }
                        #[cfg(not(target_os = "macos"))]
                        {
                            let mut cmd = std::process::Command::new(&command);
                            // Propagate Wayland environment
                            if let Ok(display) = std::env::var("WAYLAND_DISPLAY") {
                                cmd.env("WAYLAND_DISPLAY", display);
                            }
                            if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
                                cmd.env("XDG_RUNTIME_DIR", runtime_dir);
                            }
                            cmd.env("GDK_BACKEND", "wayland");
                            cmd.spawn().ok();
                        }
                    }
                    dock::DockMessage::Launch(app_id) => {
                        self.show_app_grid = false;
                        self.is_desktop_revealed = false;

                        // Track as running if not pinned
                        if !self.pinned_apps.contains(&app_id)
                            && !self.running_apps.contains(&app_id)
                        {
                            self.running_apps.push(app_id);
                        }

                        match app_id {
                            peak_core::registry::AppId::Terminal => {
                                return Task::done(Message::ToggleTerminal);
                            }
                            peak_core::registry::AppId::Browser => {
                                return Task::done(Message::LaunchBrowser(
                                    "https://duckduckgo.com".into(),
                                ));
                            }
                            peak_core::registry::AppId::Library => {
                                return Task::done(Message::ToggleArcade);
                            }
                            peak_core::registry::AppId::Cortex => {
                                self.current_page = Page::Cortex;
                            }
                            peak_core::registry::AppId::Settings => {
                                return Task::done(Message::ToggleSettings);
                            }
                            peak_core::registry::AppId::Turntable => {
                                return Task::done(Message::ToggleJukebox);
                            }
                            peak_core::registry::AppId::FileManager => {
                                return Task::done(Message::ToggleExplorer);
                            }
                            peak_core::registry::AppId::Store => {
                                return Task::done(Message::ToggleStore);
                            }
                            peak_core::registry::AppId::AppGrid => {
                                return Task::done(Message::ToggleAppGrid);
                            }
                            peak_core::registry::AppId::Spotify => {
                                if cfg!(target_os = "macos") {
                                    std::process::Command::new("open")
                                        .arg("/Applications/Spotify.app")
                                        .spawn()
                                        .ok();
                                } else {
                                    std::process::Command::new("spotify").spawn().ok();
                                }
                            }
                            peak_core::registry::AppId::Editor => {
                                return Task::done(Message::ToggleEditor);
                            }
                            peak_core::registry::AppId::Desktop => {
                                // Desktop is always visible
                            }
                        }
                    }
                    dock::DockMessage::StartDrag(index) => {
                        self.context_menu_app = None; // Close menu on drag
                        if index < self.pinned_apps.len() {
                            self.dragging_app = Some((self.pinned_apps[index], index));
                        }
                    }

                    dock::DockMessage::UpdateDrag(hover_index) => {
                        if let Some((app_id, original_index)) = self.dragging_app {
                            if hover_index < self.pinned_apps.len() && hover_index != original_index
                            {
                                self.pinned_apps.remove(original_index);
                                self.pinned_apps.insert(hover_index, app_id);
                                self.dragging_app = Some((app_id, hover_index));
                            }
                        }
                    }
                    dock::DockMessage::EndDrag => {
                        self.dragging_app = None;
                    }
                    dock::DockMessage::RightClick(app_id) => {
                        self.context_menu_app = Some(app_id);
                    }
                    dock::DockMessage::Pin(app_id) => {
                        if !self.pinned_apps.contains(&app_id) {
                            self.pinned_apps.push(app_id);
                            self.running_apps.retain(|&id| id != app_id);
                        }
                        self.context_menu_app = None;
                    }
                    dock::DockMessage::Unpin(app_id) => {
                        self.pinned_apps.retain(|&id| id != app_id);
                        // Move to running if it was active (we assume it is if it was in dock)
                        if !self.running_apps.contains(&app_id) {
                            self.running_apps.push(app_id);
                        }
                        self.context_menu_app = None;
                    }
                    dock::DockMessage::Quit(app_id) => {
                        self.close_window(app_id);
                        self.running_apps.retain(|&id| id != app_id);
                        self.context_menu_app = None;
                    }
                    dock::DockMessage::CloseContextMenu => {
                        self.context_menu_app = None;
                    }
                }
                Task::none()
            }
            Message::Inspector(msg) => {
                // Always clear dragging when Inspector receives any event
                self.window_manager.dragging = None;
                self.dragging_app = None;

                // Special handling for MouseReleased to ensure it's processed
                if matches!(
                    msg,
                    crate::components::inspector::InspectorMessage::MouseReleased
                ) {
                    return Task::none();
                }

                // Intercept SubmitMessage to start chat
                if let crate::components::inspector::InspectorMessage::SubmitMessage = msg {
                    if !self.inspector.input_content.trim().is_empty() {
                        let prompt = self.inspector.input_content.clone();
                        self.pending_chat = Some(prompt);
                        // Note: Inspector::update clears input_content, so we grabbed it first.
                        // Inspector also adds the user message local history.
                        // But we want to add a *placeholder* for the assistant immediately?
                        // Or just let the first token create it.
                        // Inspector::update currently adds a fake reply. We should remove that.
                    }
                }

                self.inspector.update(msg).map(Message::Inspector)
            }
            Message::ToggleInspector => {
                self.inspector.is_visible = !self.inspector.is_visible;

                // Sync available models when opening Inspector
                if self.inspector.is_visible {
                    let models_dir = peak_intelligence::brain::model::Directory::default();
                    let mut available_models = Vec::new();

                    if let Ok(entries) = std::fs::read_dir(models_dir.path()) {
                        for entry in entries.flatten() {
                            if let Ok(file_name) = entry.file_name().into_string() {
                                if file_name.ends_with(".gguf") {
                                    // Extract model ID from filename (remove .gguf extension)
                                    let model_id = file_name.trim_end_matches(".gguf").to_string();
                                    available_models.push(model_id);
                                }
                            }
                        }
                    }

                    return Task::done(Message::Inspector(
                        crate::components::inspector::InspectorMessage::SyncAvailableModels(
                            available_models,
                        ),
                    ));
                }

                Task::none()
            }
            Message::Settings(settings_msg) => {
                // Intercept global settings actions if needed
                match &settings_msg {
                    peak_apps::settings::SettingsMessage::ThemeChanged(mode) => {
                        self.theme = match mode {
                            peak_apps::settings::ThemeMode::Light => peak_core::theme::Theme::Light,
                            peak_apps::settings::ThemeMode::Dark => peak_core::theme::Theme::Dark,
                        };
                        self.update_tokens();
                    }
                    peak_apps::settings::SettingsMessage::ModelDownload(id) => {
                        // Start tracking this download
                        println!("Requesting download for model: {}", id);
                        self.active_downloads.insert(id.clone());
                    }
                    peak_apps::settings::SettingsMessage::ModelDownloadCancel(id) => {
                        // Stop tracking limits download, causing subscription to drop
                        self.active_downloads.remove(id);
                    }
                    peak_apps::settings::SettingsMessage::ModelDownloadProgress(id, progress) => {
                        // Remove from active downloads if complete
                        if *progress >= 1.0 {
                            self.active_downloads.remove(id);
                        }
                    }
                    peak_apps::settings::SettingsMessage::ModelDownloadFailed(id, _) => {
                        self.active_downloads.remove(id);
                    }
                    peak_apps::settings::SettingsMessage::ModelActivate(id) => {
                        // Set active model in main state
                        self.active_model_id = Some(id.clone());
                        // Notify Inspector of active model change
                        return Task::batch(vec![
                            self.forward_to_app(
                                AppId::Settings,
                                Message::Settings(settings_msg.clone()),
                            ),
                            Task::done(Message::Inspector(
                                crate::components::inspector::InspectorMessage::SetActiveModel(
                                    id.clone(),
                                ),
                            )),
                        ]);
                    }
                    #[cfg(feature = "voice")]
                    peak_apps::settings::SettingsMessage::ToggleVoice(enabled) => {
                        return Task::batch(vec![
                            self.forward_to_app(
                                AppId::Settings,
                                Message::Settings(settings_msg.clone()),
                            ),
                            Task::done(Message::Inspector(
                                crate::components::inspector::InspectorMessage::SetVoiceEnabled(
                                    *enabled,
                                ),
                            )),
                        ]);
                    }
                    peak_apps::settings::SettingsMessage::WallpaperChanged(path) => {
                        self.custom_wallpaper = Some(path.clone());
                    }
                    peak_apps::settings::SettingsMessage::ModeChanged(mode) => {
                        self.mode = *mode;
                        self.update_style_from_mode();
                    }
                    peak_apps::settings::SettingsMessage::ShellStyleChanged(style) => {
                        self.shell_style = *style;
                    }
                    _ => {}
                }

                self.forward_to_app(AppId::Settings, Message::Settings(settings_msg))
            }
            Message::SwitchMode(mode) => {
                self.mode = mode;
                self.show_spaces_selector = false;
                match self.mode {
                    ShellMode::Desktop => self.current_page = Page::Home,
                    _ => self.current_page = Page::Home,
                }
                self.update_style_from_mode();
                Task::none()
            }
            Message::SwitchShellStyle(style) => {
                self.shell_style = style;
                Task::none()
            }
            Message::AiInputChange(text) => {
                self.ai_input_text = text;
                Task::none()
            }
            Message::AiSubmit => {
                let prompt = self.ai_input_text.trim().to_string();
                if !prompt.is_empty() {
                    self.pending_chat = Some(prompt);
                    self.ai_input_text.clear();
                }
                Task::none()
            }
            Message::ToggleMode => {
                self.mode = match self.mode {
                    ShellMode::Desktop => ShellMode::Mobile,
                    _ => ShellMode::Desktop,
                };
                self.current_page = Page::Home;
                self.update_style_from_mode();
                Task::none()
            }

            Message::Library(msg) => {
                match &msg {
                    LibraryMessage::LaunchItem(cmd) => {
                        let cmd = cmd.clone();
                        return Task::done(Message::LaunchGame(cmd));
                    }
                    LibraryMessage::ImageLoaded(url, handle) => {
                        if let Some(game) = self.games.iter_mut().find(|g| &g.cover_image == url) {
                            game.image_handle = Some(handle.clone());
                        }
                    }
                    _ => {}
                }
                self.forward_to_app(AppId::Library, Message::Library(msg))
            }
            Message::Explorer(msg) => {
                self.forward_to_app(AppId::FileManager, Message::Explorer(msg))
            }
            Message::MenubarAction(action) => match action {
                MenubarMessage::ToggleSettings => {
                    self.show_settings = !self.show_settings;
                    Task::none()
                }
                MenubarMessage::ToggleOmnibar => {
                    self.show_omnibar = !self.show_omnibar;
                    Task::none()
                }
                MenubarMessage::ToggleSpaces => {
                    self.show_spaces_selector = !self.show_spaces_selector;
                    Task::none()
                }
                MenubarMessage::ToggleSystemMenu => {
                    self.show_system_menu = !self.show_system_menu;
                    self.show_reality_menu = false;
                    self.show_wifi_menu = false;
                    Task::none()
                }
                MenubarMessage::ToggleRealityMenu => {
                    self.show_reality_menu = !self.show_reality_menu;
                    self.show_system_menu = false;
                    self.show_wifi_menu = false;
                    Task::none()
                }
                MenubarMessage::ToggleWifiMenu => {
                    self.show_wifi_menu = !self.show_wifi_menu;
                    self.show_system_menu = false;
                    self.show_reality_menu = false;

                    if self.show_wifi_menu {
                        self.networks = peak_core::integrations::network::get_available_networks();
                    }

                    Task::none()
                }
                MenubarMessage::ToggleInspector => Task::done(Message::ToggleInspector),
            },
            Message::Wizard(msg) => {
                let mut should_complete = false;
                let mut new_profile_opt = None;
                let theme_pref;
                let mut selected_app_mode = ShellMode::Desktop;
                let mut selected_shell_style = peak_core::registry::ShellStyle::Cupertino;

                if let AppState::Setup(ref mut wizard_state) = self.state {
                    match msg {
                        peak_apps::wizard::WizardMessage::CompleteSetup => {
                            should_complete = true;
                            theme_pref = match self.theme {
                                peak_core::theme::Theme::Light => "Light".to_string(),
                                peak_core::theme::Theme::Dark => "Dark".to_string(),
                                _ => "Light".to_string(),
                            };

                            // Map string mode to ShellMode
                            if let Some(mode_str) = &wizard_state.selected_mode {
                                selected_app_mode = match mode_str.as_str() {
                                    "desktop" => ShellMode::Desktop,
                                    "mobile" => ShellMode::Mobile,
                                    "tv" => ShellMode::TV,
                                    "console" => ShellMode::Console,
                                    _ => ShellMode::Desktop,
                                };
                            }

                            // Map string theme to ShellStyle
                            if let Some(theme_str) = &wizard_state.selected_theme {
                                selected_shell_style = match theme_str.as_str() {
                                    "cupertino" => peak_core::registry::ShellStyle::Cupertino,
                                    "redmond" => peak_core::registry::ShellStyle::Redmond,
                                    "ai" => peak_core::registry::ShellStyle::AI,
                                    _ => peak_core::registry::ShellStyle::Cupertino,
                                };
                            }

                            // Use selected avatar or default to "peak"
                            let avatar = wizard_state
                                .selected_avatar
                                .clone()
                                .or(Some("peak".to_string()));

                            new_profile_opt = Some(peak_apps::auth::UserProfile {
                                username: wizard_state.username_input.clone(),
                                full_name: wizard_state.full_name_input.clone(),
                                theme_preference: theme_pref.clone(),
                                avatar_icon: avatar,
                                password_hash: wizard_state.password_input.clone(),
                                ..Default::default()
                            });
                        }
                        _ => {
                            let _ = peak_apps::wizard::update(wizard_state, msg);
                        }
                    }
                }

                if should_complete {
                    if let Some(profile) = new_profile_opt {
                        if peak_apps::auth::save_user(&profile) {
                            self.user = Some(profile);
                            self.state = AppState::Desktop;
                            self.mode = selected_app_mode;
                            self.shell_style = selected_shell_style;
                            self.update_style_from_mode(); // Refresh styles for new mode
                        }
                    }
                }

                Task::none()
            }
            Message::UpdateLoginPassword(s) => {
                if let AppState::Login(_) = self.state {
                    self.state = AppState::Login(s);
                }
                Task::none()
            }
            Message::SubmitLogin => {
                if let AppState::Login(ref pwd) = self.state {
                    let mut authorized = false;
                    // Check against user profile
                    if let Some(user) = &self.user {
                        if user.password_hash == *pwd {
                            authorized = true;
                        } else if user.password_hash.is_empty() {
                            // Recoverable state: if no password set, allow
                            authorized = true;
                        }
                    } else {
                        // CRITICAL: No user loaded? Deny login.
                        println!("Login Error: No user profile loaded.");
                        authorized = false;
                    }

                    if authorized {
                        self.state = AppState::Desktop;
                    } else {
                        println!("Login Failed: Incorrect Password or No User");
                        // TODO: Shake animation or catch error
                    }
                }
                Task::none()
            }
            Message::Jukebox(msg) => {
                match &msg {
                    peak_apps::jukebox::JukeboxMessage::PlayTrack(item) => {
                        let cmd = item.launch_command.replace("play ", "").replace("\"", "");
                        crate::audio::play_track(cmd);
                    }
                    peak_apps::jukebox::JukeboxMessage::TogglePlayback => {
                        crate::audio::toggle_playback();
                    }
                    _ => {}
                }
                self.forward_to_app(AppId::Turntable, Message::Jukebox(msg))
            }
            Message::ToggleOmnibar => {
                self.show_omnibar = !self.show_omnibar;
                Task::none()
            }
            Message::ToggleSpaces => {
                self.show_spaces_selector = !self.show_spaces_selector;
                Task::none()
            }
            Message::SwitchSpace(msg) => match msg {
                crate::components::spaces_strip::SpacesMessage::SwitchDesktop(idx) => {
                    Task::done(Message::SwitchDesktop(idx))
                }
            },
            Message::Desktop(msg) => {
                match &msg {
                    crate::components::desktop::DesktopMessage::Open(path) => {
                        self.is_desktop_revealed = false;
                        if path.is_dir() {
                            return Task::batch(vec![
                                self.forward_to_app(
                                    AppId::FileManager,
                                    Message::Explorer(
                                        peak_apps::explorer::ExplorerMessage::Navigate(
                                            path.clone(),
                                        ),
                                    ),
                                ),
                                Task::done(Message::ToggleExplorer),
                            ]);
                        } else {
                            self.show_editor = true;
                            // Send Open(path) message to Editor
                            return Task::batch(vec![
                                self.forward_to_app(
                                    AppId::Editor,
                                    Message::Editor(peak_apps::editor::EditorMessage::Open(
                                        path.clone(),
                                    )),
                                ),
                                Task::done(Message::ToggleEditor),
                            ]);
                        }
                    }
                    crate::components::desktop::DesktopMessage::Select(path, _) => {
                        let is_multi =
                            self.tracked_modifiers.shift() || self.tracked_modifiers.command();
                        if let Some(follow_up) =
                            self.desktop
                                .update(crate::components::desktop::DesktopMessage::Select(
                                    path.clone(),
                                    is_multi,
                                ))
                        {
                            return Task::done(Message::Desktop(follow_up));
                        }
                    }
                    _ => {
                        if let Some(follow_up) = self.desktop.update(msg) {
                            return Task::done(Message::Desktop(follow_up));
                        }
                    }
                }
                Task::none()
            }
            Message::Editor(msg) => self.forward_to_app(AppId::Editor, Message::Editor(msg)),
            Message::ToggleEditor => {
                self.is_desktop_revealed = false;
                self.show_editor = !self.show_editor;
                if self.show_editor {
                    self.window_manager.ensure_window_state(
                        peak_core::registry::AppId::Editor,
                        800.0,
                        600.0,
                        self.mode,
                        self.current_desktop,
                    );
                } else {
                    self.close_window(peak_core::registry::AppId::Editor);
                }
                Task::none()
            }
            Message::SwitchDesktop(idx) => {
                self.current_desktop = idx;
                self.show_spaces_selector = false;
                Task::none()
            }
            Message::Omnibar(msg) => {
                let task = self.omnibar.update(msg.clone());

                let side_effect = match msg {
                    OmnibarMessage::Cancel => {
                        self.show_omnibar = false;
                        Task::none()
                    }
                    OmnibarMessage::Submit => {
                        // Handle submission based on mode
                        if let Some(app_id) = self.omnibar.get_selected_app() {
                            // Search mode - launch app
                            self.show_omnibar = false;
                            Task::done(Message::DockInteraction(dock::DockMessage::Launch(app_id)))
                        } else if let Some(apk_name) = self.omnibar.get_selected_apk() {
                            // Install mode - select APK
                            Task::done(Message::Omnibar(
                                crate::components::omnibar::OmnibarMessage::SelectApk(apk_name),
                            ))
                        } else if let Some(menu_item) = self.omnibar.get_selected_menu_item() {
                            // Menu mode - select menu item
                            Task::done(Message::Omnibar(
                                crate::components::omnibar::OmnibarMessage::SelectMenuItem(
                                    menu_item,
                                ),
                            ))
                        } else {
                            Task::none()
                        }
                    }
                    OmnibarMessage::SelectApp(app_id) => {
                        self.show_omnibar = false;
                        Task::done(Message::DockInteraction(dock::DockMessage::Launch(app_id)))
                    }
                    OmnibarMessage::SelectMenuItem(_item) => {
                        // Menu items handled in omnibar.update()
                        Task::none()
                    }
                    OmnibarMessage::SelectApk(pkg_name) => {
                        self.show_omnibar = false;
                        let cmd = format!("sudo apk add {}", pkg_name);
                        Task::batch(vec![
                            Task::done(Message::DockInteraction(dock::DockMessage::Launch(
                                AppId::Terminal,
                            ))),
                            self.forward_to_app(
                                AppId::Terminal,
                                Message::Terminal(
                                    peak_core::apps::terminal::TerminalMessage::RunCommand(cmd),
                                ),
                            ),
                        ])
                    }
                    _ => Task::none(),
                };

                Task::batch(vec![task.map(Message::Omnibar), side_effect])
            }
            Message::Switcher(msg) => {
                match msg {
                    SwitcherMessage::Next => self.switcher.next(),
                    SwitcherMessage::Prev => self.switcher.prev(),
                    SwitcherMessage::Select(app_id) => {
                        self.show_switcher = false;
                        self.is_desktop_revealed = false;
                        return Task::done(Message::DockInteraction(dock::DockMessage::Launch(
                            app_id,
                        )));
                    }
                }
                Task::none()
            }
            Message::ToggleSwitcher => {
                self.show_switcher = !self.show_switcher;
                Task::none()
            }

            Message::ToggleArcade => {
                self.is_desktop_revealed = false;
                if let Some(state) = self
                    .window_manager
                    .window_states
                    .get(&peak_core::registry::AppId::Library)
                {
                    if state.reality != self.mode || state.desktop_idx != self.current_desktop {
                        self.mode = state.reality;
                        self.current_desktop = state.desktop_idx;
                        self.window_manager
                            .bring_to_front(peak_core::registry::AppId::Library);
                    } else {
                        self.close_window(peak_core::registry::AppId::Library);
                    }
                } else {
                    self.ensure_window_state(peak_core::registry::AppId::Library, 800.0, 600.0);
                }
                Task::none()
            }
            Message::ToggleJukebox => {
                self.is_desktop_revealed = false;
                if let Some(state) = self
                    .window_manager
                    .window_states
                    .get(&peak_core::registry::AppId::Turntable)
                {
                    if state.reality != self.mode || state.desktop_idx != self.current_desktop {
                        self.mode = state.reality;
                        self.current_desktop = state.desktop_idx;
                        self.window_manager
                            .bring_to_front(peak_core::registry::AppId::Turntable);
                    } else {
                        self.close_window(peak_core::registry::AppId::Turntable);
                    }
                } else {
                    self.ensure_window_state(peak_core::registry::AppId::Turntable, 700.0, 600.0);
                }
                Task::none()
            }

            Message::ToggleSettings => {
                self.window_manager.dragging = None;
                self.dragging_app = None;
                self.show_settings = !self.show_settings;
                let task = self.toggle_app(peak_core::registry::AppId::Settings, 1000.0, 800.0);
                if self.show_settings {
                    return Task::batch(vec![
                        task,
                        self.forward_to_app(
                            AppId::Settings,
                            Message::Settings(peak_apps::settings::SettingsMessage::ModeChanged(
                                self.mode,
                            )),
                        ),
                    ]);
                }
                task
            }

            Message::ToggleSystemMenu => {
                self.show_system_menu = !self.show_system_menu;
                Task::none()
            }
            Message::ToggleExplorer => self.toggle_app(AppId::FileManager, 600.0, 450.0),
            Message::ToggleStore => self.toggle_app(AppId::Store, 800.0, 600.0),
            Message::Store(msg) => {
                match &msg {
                    peak_apps::store::StoreMessage::LaunchUrl(url) => {
                        return Task::done(Message::LaunchBrowser(url.clone()));
                    }
                    peak_apps::store::StoreMessage::InstallApp(pkg_name) => {
                        let cmd = format!("sudo apk add {}", pkg_name.to_lowercase());
                        // Forward to Store so it updates UI state (even if its internal install fails)
                        // And launch Terminal for the actual sudo work
                        return Task::batch(vec![
                            self.forward_to_app(AppId::Store, Message::Store(msg.clone())),
                            Task::done(Message::DockInteraction(dock::DockMessage::Launch(
                                AppId::Terminal,
                            ))),
                            self.forward_to_app(
                                AppId::Terminal,
                                Message::Terminal(
                                    peak_core::apps::terminal::TerminalMessage::RunCommand(cmd),
                                ),
                            ),
                        ]);
                    }
                    _ => {}
                }
                self.forward_to_app(AppId::Store, Message::Store(msg))
            }
            Message::Restart => {
                std::process::exit(0);
            }
            Message::LogOut => {
                // Keep the user profile loaded so we can show their name/avatar
                // self.user = None;
                self.state = AppState::Login(String::new());
                self.show_system_menu = false;
                // Reset other states
                self.show_settings = false;
                self.show_switcher = false;
                self.show_omnibar = false;
                Task::none()
            }
            Message::FactoryReset => {
                // Delete user config
                let config_path = peak_apps::auth::get_config_dir().join("user.json");
                if config_path.exists() {
                    let _ = std::fs::remove_file(config_path);
                }
                // Reset state
                self.user = None;
                self.state = AppState::Setup(peak_apps::wizard::WizardState::default());
                self.show_system_menu = false;
                self.theme = peak_core::theme::Theme::Light;
                Task::none()
            }
            Message::ToggleTerminal => self.toggle_app(AppId::Terminal, 640.0, 480.0),
            Message::Terminal(msg) => self.forward_to_app(AppId::Terminal, Message::Terminal(msg)),
            Message::ToggleAppGrid => {
                self.show_app_grid = !self.show_app_grid;
                Task::none()
            }
            // Message::Browser removed - using external Firefox
            Message::AssistantBooted(result) => {
                match result {
                    Ok(assistant) => {
                        self.assistant = Some(assistant);
                        self.alert = Some((
                            "Assistant Ready".into(),
                            "Peak Intelligence is now active.".into(),
                        ));
                    }
                    Err(e) => {
                        self.alert = Some((
                            "AI Boot Failed".into(),
                            format!("Could not start AI: {}", e),
                        ));
                        self.pending_chat = None;
                    }
                }
                Task::none()
            }
            Message::AssistantReply(_reply, token) => {
                // Check if last message is assistant, if so append, else push
                // Inspector stores history as Vec<(Role, Content)>

                let content_chunk = match token {
                    peak_intelligence::brain::assistant::Token::Talking(s) => s,
                    peak_intelligence::brain::assistant::Token::Reasoning(s) => s, // Treat reasoning as text for now
                };

                if let Some((role, content)) = self.inspector.chat_history.last_mut() {
                    if role == "assistant" {
                        content.push_str(&content_chunk);
                    } else {
                        // New assistant message
                        self.inspector
                            .chat_history
                            .push(("assistant".to_string(), content_chunk));
                    }
                } else {
                    self.inspector
                        .chat_history
                        .push(("assistant".to_string(), content_chunk));
                }

                // Auto-scroll to bottom
                iced::widget::scrollable::snap_to(
                    iced::widget::scrollable::Id::new("chat_scroll"),
                    iced::widget::scrollable::RelativeOffset::END,
                )
            }
            Message::AssistantFinished => {
                self.pending_chat = None;
                Task::none()
            }
            Message::ConsoleCategory(msg) => {
                // Handle Console category selection
                let peak_shell::console::category_bar::CategoryBarMessage::SelectCategory(_cat) =
                    msg;
                // Update state or trigger filter
                Task::none()
            }
            Message::ConsoleGame(msg) => {
                // Handle Console game selection/launch
                if let peak_shell::console::game_rail::GameRailMessage::SelectGame(cmd) = msg {
                    opener::open(cmd).ok();
                }
                Task::none()
            }
            Message::TVApp(msg) => {
                // Handle TV app selection
                match msg {
                    peak_shell::tv::app_rail::AppRailMessage::SelectApp(id) => {
                        self.toggle_app(id, 800.0, 600.0)
                    }
                    peak_shell::tv::app_rail::AppRailMessage::LaunchApp(id) => {
                        self.toggle_app(id, 800.0, 600.0)
                    }
                }
            }
            Message::RedmondTaskbar(msg) => {
                // Handle Redmond taskbar interaction
                match msg {
                    peak_shell::redmond::taskbar::TaskbarMessage::OpenStart => {
                        self.show_app_grid = !self.show_app_grid;
                        Task::none()
                    }
                    peak_shell::redmond::taskbar::TaskbarMessage::LaunchApp(id) => {
                        self.toggle_app(id, 800.0, 600.0)
                    }
                    _ => Task::none(),
                }
            }
        }
    }
}
