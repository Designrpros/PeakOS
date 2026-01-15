// Message handling logic

use super::{AppState, Message, PeakNative};
use crate::apps::library::LibraryMessage;
use crate::components::{
    app_switcher::SwitcherMessage, dock, menubar::MenubarMessage, omnibar::OmnibarMessage,
};
use crate::pages::Page;
use crate::registry::ShellMode;
use iced::Task;

impl PeakNative {
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
                                iced::Size::new(size.width as f32, size.height as f32);
                        }
                        iced::window::Event::Moved(point) => {
                            self.window_position = *point;
                        }
                        _ => {}
                    }
                }

                // Mouse Events for Dragging & Focus
                match event {
                    iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                        iced::mouse::Button::Left,
                    )) => {
                        let mut focused_app: Option<crate::registry::AppId> = None;
                        for &app_id in self.window_manager.z_order.iter().rev() {
                            if let Some(state) = self.window_manager.window_states.get(&app_id) {
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
                                state.x = position.x - offset.x;
                                state.y = position.y - offset.y;

                                // Constraint: Don't drag above menubar
                                if state.y < 32.0 {
                                    state.y = 32.0;
                                }
                            }
                        }
                    }
                    iced::Event::Mouse(iced::mouse::Event::ButtonReleased(
                        iced::mouse::Button::Left,
                    )) => {
                        self.window_manager.dragging = None;
                        self.dragging_app = None;
                        self.desktop
                            .update(crate::components::desktop::DesktopMessage::EndSelection);
                    }
                    iced::Event::Keyboard(iced::keyboard::Event::ModifiersChanged(modifiers)) => {
                        self.tracked_modifiers = modifiers;
                    }
                    _ => {}
                }

                // Keyboard Events (using reference to avoid move)
                if let iced::Event::Keyboard(kb_event) = &event {
                    match kb_event {
                        iced::keyboard::Event::KeyPressed { key, modifiers, .. } => {
                            // Omnibar Navigation (when open) - Handle first
                            if self.show_omnibar {
                                match key {
                                    iced::keyboard::Key::Named(
                                        iced::keyboard::key::Named::ArrowUp,
                                    ) => {
                                        return Task::done(Message::Omnibar(
                                            crate::components::omnibar::OmnibarMessage::NavigateUp,
                                        ));
                                    }
                                    iced::keyboard::Key::Named(
                                        iced::keyboard::key::Named::ArrowDown,
                                    ) => {
                                        return Task::done(Message::Omnibar(
                                            crate::components::omnibar::OmnibarMessage::NavigateDown,
                                        ));
                                    }
                                    iced::keyboard::Key::Named(
                                        iced::keyboard::key::Named::Enter,
                                    ) => {
                                        return Task::done(Message::Omnibar(
                                            crate::components::omnibar::OmnibarMessage::Submit,
                                        ));
                                    }
                                    iced::keyboard::Key::Named(
                                        iced::keyboard::key::Named::Escape,
                                    ) => {
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
                            if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) =
                                key
                            {
                                if modifiers.command() || modifiers.control() || modifiers.alt() {
                                    return Task::done(Message::ToggleOmnibar);
                                }
                            }

                            // Quick Look (Space)
                            if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) =
                                key
                            {
                                if !modifiers.command() && !modifiers.control() && !modifiers.alt()
                                {
                                    if let Some(path) = self.desktop.selected.first() {
                                        self.quick_look_path = Some(path.clone());
                                    }
                                }
                            }

                            // Switcher (Cmd/Alt + Tab)
                            if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) = key
                            {
                                if modifiers.command() || modifiers.alt() {
                                    if !self.show_switcher {
                                        // Populate switcher with running apps from z_order (reverse for MRU)
                                        let running_apps: Vec<crate::registry::AppInfo> = self
                                            .window_manager
                                            .z_order
                                            .iter()
                                            .rev()
                                            .map(|&id| crate::registry::AppInfo::get_info(id))
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
                                    } else {
                                        if modifiers.shift() {
                                            self.switcher.prev();
                                        } else {
                                            self.switcher.next();
                                        }
                                    }
                                }
                            }

                            // Wizard Navigation (Enter/Space on Welcome screen)
                            if matches!(self.state, AppState::Setup(_)) {
                                match key {
                                    iced::keyboard::Key::Named(
                                        iced::keyboard::key::Named::Enter,
                                    )
                                    | iced::keyboard::Key::Named(
                                        iced::keyboard::key::Named::Space,
                                    ) => {
                                        if let AppState::Setup(ref wizard_state) = self.state {
                                            if wizard_state.current_step
                                                == crate::apps::wizard::WizardStep::Welcome
                                            {
                                                return Task::done(Message::Wizard(
                                                    crate::apps::wizard::WizardMessage::NextStep,
                                                ));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
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
                // Poll window position if we think we are at 0,0 (likely startup)
                // We now rely on iced::window::Event::Moved for passive sync
                // to avoid freezing the main thread with osascript.

                // Sync Browser Window Logic
                if let Some(child) = &mut self.browser_process {
                    if let Some(stdin) = &mut child.stdin {
                        // Find if browser is open and where it is
                        if let Some(state) = self
                            .window_manager
                            .window_states
                            .get(&crate::registry::AppId::Browser)
                        {
                            // Only sync if it's in the z_order (visible/active)
                            if self
                                .window_manager
                                .z_order
                                .contains(&crate::registry::AppId::Browser)
                            {
                                use std::io::Write;
                                // Offset by main window position + titlebar (approx 32px for chrome padding + iced::widget::text(
                                let titlebar_height = 32.0;
                                let cmd = crate::apps::browser::BrowserCommand::Layout {
                                    x: (self.window_position.x + state.x) as f64,
                                    y: (self.window_position.y + state.y + titlebar_height) as f64,
                                    width: state.width as f64,
                                    height: (state.height - titlebar_height).max(1.0) as f64,
                                };
                                if let Ok(json) = serde_json::to_string(&cmd) {
                                    let _ = writeln!(stdin, "{}", json);
                                }
                            }
                        }
                    }
                }

                self.vector_bg.clear_cache();
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
                // Check if browser is already running
                if self.browser_process.is_none() {
                    let current_exe =
                        std::env::current_exe().unwrap_or_else(|_| "peak-native".into());
                    let child = std::process::Command::new(current_exe)
                        .arg("--browser")
                        .arg(&url)
                        .stdin(std::process::Stdio::piped())
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .spawn()
                        .expect("Failed to spawn browser process");

                    self.browser_process = Some(child);
                } else {
                    // Send navigate command if already running
                    if let Some(child) = &mut self.browser_process {
                        if let Some(stdin) = &mut child.stdin {
                            use std::io::Write;
                            let cmd =
                                crate::apps::browser::BrowserCommand::Navigate { url: url.clone() };
                            if let Ok(json) = serde_json::to_string(&cmd) {
                                let _ = writeln!(stdin, "{}", json);
                            }
                        }
                    }
                }

                // Ensure window is open in WM
                if let Some(state) = self
                    .window_manager
                    .window_states
                    .get(&crate::registry::AppId::Browser)
                {
                    if state.reality != self.mode || state.desktop_idx != self.current_desktop {
                        // Switch to the app's workspace
                        self.mode = state.reality;
                        self.current_desktop = state.desktop_idx;
                        self.window_manager
                            .bring_to_front(crate::registry::AppId::Browser);
                    }
                } else {
                    self.ensure_window_state(crate::registry::AppId::Browser, 1024.0, 768.0);
                }
                Task::none()
            }
            Message::CloseBrowser => {
                self.close_window(crate::registry::AppId::Browser);
                // Optionally kill the process or keep it warm?
                // For now, let's keep the process running but hide it (via IPC usually, or just move it offscreen)
                // Sending a 0x0 size layout might be safest to "hide" it visually if the process stays alive.
                if let Some(child) = &mut self.browser_process {
                    if let Some(stdin) = &mut child.stdin {
                        use std::io::Write;
                        let cmd = crate::apps::browser::BrowserCommand::Layout {
                            x: -10000.0,
                            y: -10000.0,
                            width: 0.0,
                            height: 0.0,
                        };
                        if let Ok(json) = serde_json::to_string(&cmd) {
                            let _ = writeln!(stdin, "{}", json);
                        }
                    }
                }
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
                if cmd.starts_with("steam://") {
                    std::process::Command::new("open").arg(&cmd).spawn().ok();
                } else {
                    std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&cmd)
                        .spawn()
                        .ok();
                }
                Task::none()
            }
            Message::ToggleTheme => {
                self.settings.theme_mode = match self.settings.theme_mode {
                    crate::apps::settings::ThemeMode::Light => {
                        crate::apps::settings::ThemeMode::Dark
                    }
                    crate::apps::settings::ThemeMode::Dark => {
                        crate::apps::settings::ThemeMode::Light
                    }
                };
                self.theme = match self.settings.theme_mode {
                    crate::apps::settings::ThemeMode::Light => crate::theme::Theme::Light,
                    crate::apps::settings::ThemeMode::Dark => crate::theme::Theme::Dark,
                };
                Task::none()
            }
            Message::Navigate(page) => {
                self.current_page = page;
                Task::none()
            }
            Message::DockInteraction(dock_msg) => {
                match dock_msg {
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
                            crate::registry::AppId::Terminal => {
                                return Task::done(Message::ToggleTerminal);
                            }
                            crate::registry::AppId::Browser => {
                                return Task::done(Message::LaunchBrowser(
                                    "https://duckduckgo.com".into(),
                                ));
                            }
                            crate::registry::AppId::Library => {
                                return Task::done(Message::ToggleArcade);
                            }
                            crate::registry::AppId::Cortex => {
                                self.current_page = Page::Cortex;
                            }
                            crate::registry::AppId::Settings => {
                                return Task::done(Message::ToggleSettings);
                            }
                            crate::registry::AppId::Turntable => {
                                return Task::done(Message::ToggleJukebox);
                            }
                            crate::registry::AppId::FileManager => {
                                return Task::done(Message::ToggleExplorer);
                            }
                            crate::registry::AppId::Store => {
                                return Task::done(Message::ToggleStore);
                            }
                            crate::registry::AppId::AppGrid => {
                                return Task::done(Message::ToggleAppGrid);
                            }
                            crate::registry::AppId::Antigravity => {
                                if cfg!(target_os = "macos") {
                                    std::process::Command::new("open")
                                        .arg("-a")
                                        .arg("Antigravity")
                                        .spawn()
                                        .ok();
                                } else {
                                    std::process::Command::new("antigravity").spawn().ok();
                                }
                            }
                            crate::registry::AppId::Spotify => {
                                if cfg!(target_os = "macos") {
                                    std::process::Command::new("open")
                                        .arg("/Applications/Spotify.app")
                                        .spawn()
                                        .ok();
                                } else {
                                    std::process::Command::new("spotify").spawn().ok();
                                }
                            }
                            crate::registry::AppId::Editor => {
                                return Task::done(Message::ToggleEditor);
                            }
                            crate::registry::AppId::Desktop => {
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
                if let crate::components::inspector::InspectorMessage::OpenSettings = msg {
                    // Open settings and ensure the AI section is potentially selected (future)
                    // For now just toggle settings visibility
                    return Task::done(Message::ToggleSettings);
                }
                self.inspector.update(msg).map(Message::Inspector)
            }
            Message::ToggleInspector => {
                self.inspector.is_visible = !self.inspector.is_visible;
                Task::none()
            }
            Message::Settings(msg) => match msg {
                crate::apps::settings::SettingsMessage::ThemeChanged(mode) => {
                    self.settings.theme_mode = mode;
                    self.theme = match mode {
                        crate::apps::settings::ThemeMode::Light => crate::theme::Theme::Light,
                        crate::apps::settings::ThemeMode::Dark => crate::theme::Theme::Dark,
                    };
                    Task::none()
                }
                _ => self.settings.update(msg).map(Message::Settings),
            },
            Message::SwitchMode(mode) => {
                self.mode = mode;
                self.show_spaces_selector = false;
                match self.mode {
                    ShellMode::Peak => self.current_page = Page::Home,
                    ShellMode::Poolside => self.current_page = Page::Library,
                }
                Task::none()
            }
            Message::ToggleMode => {
                self.mode = match self.mode {
                    ShellMode::Peak => ShellMode::Poolside,
                    ShellMode::Poolside => ShellMode::Peak,
                };
                match self.mode {
                    ShellMode::Peak => self.current_page = Page::Home,
                    ShellMode::Poolside => self.current_page = Page::Library,
                }
                Task::none()
            }

            Message::Library(msg) => match msg {
                LibraryMessage::LaunchItem(cmd) => {
                    return Task::done(Message::LaunchGame(cmd));
                }
                LibraryMessage::ImageLoaded(url, handle) => {
                    if let Some(game) = self.games.iter_mut().find(|g| g.cover_image == url) {
                        game.image_handle = Some(handle);
                    }
                    Task::none()
                }
                LibraryMessage::ImageLoadFailed(_url) => Task::none(),
            },
            Message::Explorer(msg) => {
                self.explorer.update(msg);
                Task::none()
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
                        self.networks = crate::integrations::network::get_available_networks();
                    }

                    Task::none()
                }
                MenubarMessage::ToggleInspector => Task::done(Message::ToggleInspector),
            },
            Message::Wizard(msg) => {
                let mut should_complete = false;
                let mut new_profile_opt = None;
                let mut theme_pref = String::new();

                if let AppState::Setup(ref mut wizard_state) = self.state {
                    match msg {
                        crate::apps::wizard::WizardMessage::CompleteSetup => {
                            should_complete = true;
                            theme_pref = self.settings.theme_mode.to_string();
                            new_profile_opt = Some(crate::apps::auth::UserProfile {
                                username: wizard_state.username_input.clone(),
                                full_name: wizard_state.full_name_input.clone(),
                                theme_preference: theme_pref.clone(),
                                avatar_icon: wizard_state.selected_avatar.clone(),
                                password_hash: wizard_state.password_input.clone(), // Saving password plain for now
                                ..Default::default()
                            });
                        }
                        _ => {
                            let _ = crate::apps::wizard::update(wizard_state, msg);
                        }
                    }
                }

                if should_complete {
                    if let Some(profile) = new_profile_opt {
                        if crate::apps::auth::save_user(&profile) {
                            self.user = Some(profile);
                            self.state = AppState::Desktop;
                            if theme_pref == "Riviera" {
                                self.mode = ShellMode::Poolside;
                            }
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
                    crate::apps::jukebox::JukeboxMessage::PlayTrack(item) => {
                        let cmd = item.launch_command.replace("play ", "").replace("\"", "");
                        crate::audio::play_track(cmd);
                    }
                    crate::apps::jukebox::JukeboxMessage::TogglePlayback => {
                        crate::audio::toggle_playback();
                    }
                    _ => {}
                }
                self.jukebox.update(msg.clone());
                Task::none()
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
                    return Task::done(Message::SwitchDesktop(idx));
                }
            },
            Message::Desktop(msg) => {
                match &msg {
                    crate::components::desktop::DesktopMessage::Open(path) => {
                        self.is_desktop_revealed = false;
                        if path.is_dir() {
                            self.explorer.current_path = path.clone();
                            return Task::done(Message::ToggleExplorer);
                        } else {
                            self.editor = crate::apps::editor::EditorApp::open(path.clone());
                            self.show_editor = true;
                            return Task::done(Message::ToggleEditor);
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
            Message::Editor(msg) => {
                self.editor.update(msg);
                Task::none()
            }
            Message::ToggleEditor => {
                self.is_desktop_revealed = false;
                self.show_editor = !self.show_editor;
                if self.show_editor {
                    self.window_manager.ensure_window_state(
                        crate::registry::AppId::Editor,
                        800.0,
                        600.0,
                        self.mode,
                        self.current_desktop,
                    );
                } else {
                    self.close_window(crate::registry::AppId::Editor);
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
                    OmnibarMessage::SelectApk(_pkg_name) => {
                        // For now we just close or keep open?
                        // User might want to stay in search.
                        // self.show_omnibar = false;
                        Task::none()
                    }
                    _ => Task::none(),
                };

                return Task::batch(vec![task.map(Message::Omnibar), side_effect]);
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
                    .get(&crate::registry::AppId::Library)
                {
                    if state.reality != self.mode || state.desktop_idx != self.current_desktop {
                        self.mode = state.reality;
                        self.current_desktop = state.desktop_idx;
                        self.window_manager
                            .bring_to_front(crate::registry::AppId::Library);
                        self.library.is_open = true;
                    } else {
                        self.library.is_open = false;
                        self.close_window(crate::registry::AppId::Library);
                    }
                } else {
                    self.library.is_open = true;
                    self.ensure_window_state(crate::registry::AppId::Library, 800.0, 600.0);
                }
                Task::none()
            }
            Message::ToggleJukebox => {
                self.is_desktop_revealed = false;
                if let Some(state) = self
                    .window_manager
                    .window_states
                    .get(&crate::registry::AppId::Turntable)
                {
                    if state.reality != self.mode || state.desktop_idx != self.current_desktop {
                        self.mode = state.reality;
                        self.current_desktop = state.desktop_idx;
                        self.window_manager
                            .bring_to_front(crate::registry::AppId::Turntable);
                        self.jukebox.is_open = true;
                    } else {
                        self.jukebox.is_open = false;
                        self.close_window(crate::registry::AppId::Turntable);
                    }
                } else {
                    self.jukebox.is_open = true;
                    self.ensure_window_state(crate::registry::AppId::Turntable, 700.0, 600.0);
                }
                Task::none()
            }
            Message::ToggleSettings => {
                self.is_desktop_revealed = false;
                if let Some(state) = self
                    .window_manager
                    .window_states
                    .get(&crate::registry::AppId::Settings)
                {
                    if state.reality != self.mode || state.desktop_idx != self.current_desktop {
                        self.mode = state.reality;
                        self.current_desktop = state.desktop_idx;
                        self.window_manager
                            .bring_to_front(crate::registry::AppId::Settings);
                        self.show_settings = true;
                    } else {
                        self.show_settings = false;
                        self.close_window(crate::registry::AppId::Settings);
                    }
                } else {
                    self.show_settings = true;
                    self.ensure_window_state(crate::registry::AppId::Settings, 600.0, 400.0);
                }
                Task::none()
            }
            Message::ToggleSystemMenu => {
                self.show_system_menu = !self.show_system_menu;
                Task::none()
            }
            Message::ToggleExplorer => {
                self.is_desktop_revealed = false;
                if let Some(state) = self
                    .window_manager
                    .window_states
                    .get(&crate::registry::AppId::FileManager)
                {
                    if state.reality != self.mode || state.desktop_idx != self.current_desktop {
                        self.mode = state.reality;
                        self.current_desktop = state.desktop_idx;
                        self.window_manager
                            .bring_to_front(crate::registry::AppId::FileManager);
                    } else {
                        self.close_window(crate::registry::AppId::FileManager);
                    }
                } else {
                    self.ensure_window_state(crate::registry::AppId::FileManager, 600.0, 450.0);
                }
                Task::none()
            }
            Message::ToggleStore => {
                if let Some(state) = self
                    .window_manager
                    .window_states
                    .get(&crate::registry::AppId::Store)
                {
                    if state.reality != self.mode || state.desktop_idx != self.current_desktop {
                        self.mode = state.reality;
                        self.current_desktop = state.desktop_idx;
                        self.window_manager
                            .bring_to_front(crate::registry::AppId::Store);
                    } else {
                        self.close_window(crate::registry::AppId::Store);
                    }
                } else {
                    self.ensure_window_state(crate::registry::AppId::Store, 800.0, 600.0);
                }
                Task::none()
            }
            Message::Store(msg) => self.store.update(msg),
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
                let config_path = crate::apps::auth::get_config_dir().join("user.json");
                if config_path.exists() {
                    let _ = std::fs::remove_file(config_path);
                }
                // Reset state
                self.user = None;
                self.state = AppState::Setup(crate::apps::wizard::WizardState::default());
                self.show_system_menu = false;
                self.settings.theme_mode = crate::apps::settings::ThemeMode::Light; // Default to light
                self.theme = crate::theme::Theme::Light;
                Task::none()
            }
            Message::ToggleTerminal => {
                self.is_desktop_revealed = false;
                if let Some(state) = self
                    .window_manager
                    .window_states
                    .get(&crate::registry::AppId::Terminal)
                {
                    if state.reality != self.mode || state.desktop_idx != self.current_desktop {
                        // Switch to the app's workspace
                        self.mode = state.reality;
                        self.current_desktop = state.desktop_idx;
                        self.window_manager
                            .bring_to_front(crate::registry::AppId::Terminal);
                    } else {
                        // Already on current workspace, toggle (close)
                        self.close_window(crate::registry::AppId::Terminal);
                        self.terminal.is_open = false;
                    }
                } else {
                    self.ensure_window_state(crate::registry::AppId::Terminal, 640.0, 480.0);
                    self.terminal.is_open = true;
                }
                Task::none()
            }
            Message::Terminal(msg) => {
                self.terminal.update(msg);
                Task::none()
            }
            Message::ToggleAppGrid => {
                self.show_app_grid = !self.show_app_grid;
                Task::none()
            }
        }
    }
}
