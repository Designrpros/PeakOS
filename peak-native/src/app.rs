use crate::apps::browser::BrowserApp;
use crate::apps::explorer::{ExplorerApp, ExplorerMessage};
use crate::apps::library::{LibraryApp, LibraryMessage};
use crate::apps::settings::{SettingsApp, SettingsMessage};
use crate::apps::terminal::TerminalApp;
use crate::audio;
use crate::components::app_switcher::{AppSwitcher, SwitcherMessage};
use crate::components::menubar::{self, MenubarMessage};
use crate::components::omnibar::{Omnibar, OmnibarMessage};
use crate::components::{desktop_container, dock};
use crate::models::MediaItem;
use crate::pages::Page;
use iced::widget::{button, container, scrollable, svg, text, Column, Row, Stack};
use iced::{Color, Element, Length, Task, Theme as IcedTheme};
use std::collections::HashMap;
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct WindowState {
    #[allow(dead_code)]
    pub app_id: crate::registry::AppId,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub enum AppState {
    Setup(crate::apps::wizard::WizardState),
    Login(String), // Password Input
    Desktop,
}

pub struct PeakNative {
    pub state: AppState,
    pub user: Option<crate::apps::auth::UserProfile>,

    pub theme: crate::theme::Theme,
    pub current_page: Page,
    pub games: Vec<MediaItem>,
    pub library: LibraryApp,
    pub cortex_state: crate::pages::cortex::State,
    pub mode: ShellMode,
    pub terminal: TerminalApp,
    pub settings: SettingsApp,
    pub jukebox: crate::apps::turntable::JukeboxApp,
    pub explorer: ExplorerApp,
    pub store: crate::apps::store::StoreApp,
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
    _stream: Option<rodio::OutputStream>, // Must keep alive (if audio available)

    // Window Management
    pub window_states: HashMap<crate::registry::AppId, WindowState>,
    pub z_order: Vec<crate::registry::AppId>,
    pub dragging: Option<(crate::registry::AppId, iced::Point)>,
    pub screen_size: iced::Size,
    pub cursor_position: iced::Point,
    pub dock_visible: bool,
    pub show_system_menu: bool,
    pub show_reality_menu: bool,
    pub show_wifi_menu: bool,
    pub networks: Vec<String>,
    pub vector_bg: crate::components::vector_background::VectorBackground,
    pub show_app_grid: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    Library(LibraryMessage),
    Navigate(Page),
    ToggleTheme,
    LaunchGame(String),
    Tick,
    Exit,
    GlobalEvent(iced::Event),
    DockInteraction(dock::DockMessage),
    SwitchMode(ShellMode),
    ToggleMode,
    LogOut,
    MenubarAction(MenubarMessage),
    Settings(SettingsMessage),
    Jukebox(crate::apps::turntable::JukeboxMessage),
    ToggleSettings,
    Omnibar(OmnibarMessage),
    ToggleOmnibar,
    ToggleSpaces,
    SwitchSpace(crate::components::spaces_strip::SpacesMessage),
    SwitchDesktop(usize),
    Switcher(SwitcherMessage),
    ToggleSwitcher,
    ToggleTerminal,
    ToggleArcade,
    ToggleJukebox,
    Explorer(ExplorerMessage),
    ToggleExplorer,
    ToggleSystemMenu,
    // Applications
    ToggleStore,
    Store(crate::apps::store::StoreMessage),
    Terminal(crate::apps::terminal::TerminalMessage),
    Restart,
    // Setup & Login
    Wizard(crate::apps::wizard::WizardMessage),
    UpdateLoginPassword(String),
    SubmitLogin,
    FactoryReset,
    ToggleAppGrid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellMode {
    Peak,
    Poolside, // Riviera
}

impl PeakNative {
    pub fn new(mode_str: String) -> (Self, Task<Message>) {
        let theme = crate::theme::Theme::Light;

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
                crate::systems::image_loader::ImageLoader::load(url.clone()),
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
                jukebox: crate::apps::turntable::JukeboxApp::new(),
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
                window_states: HashMap::new(),
                z_order: Vec::new(),
                dragging: None,
                screen_size: iced::Size::new(1920.0, 1080.0), // Default fallback
                cursor_position: iced::Point::ORIGIN,
                dock_visible: true,
                show_system_menu: false,
                show_reality_menu: false,
                show_wifi_menu: false,
                show_app_grid: false,
                networks: Vec::new(),
            },
            load_images_task,
        )
    }

    pub fn title(&self) -> String {
        "PeakOS".into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GlobalEvent(event) => {
                // Window Resize
                if let iced::Event::Window(iced::window::Event::Resized(size)) = event {
                    self.screen_size = iced::Size::new(size.width as f32, size.height as f32);
                }

                // Mouse Events for Dragging & Focus
                match event {
                    iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                        iced::mouse::Button::Left,
                    )) => {
                        let mut focused_app: Option<crate::registry::AppId> = None;
                        for &app_id in self.z_order.iter().rev() {
                            if let Some(state) = self.window_states.get(&app_id) {
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
                                        self.dragging = Some((
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
                            if let Some(pos) = self.z_order.iter().position(|&id| id == app_id) {
                                self.z_order.remove(pos);
                                self.z_order.push(app_id);
                            }
                        }
                    }
                    iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                        self.cursor_position = position;

                        // Dock Auto-Hide Logic
                        let threshold_bottom = 80.0;
                        let threshold_hide = 150.0;
                        if position.y > self.screen_size.height - threshold_bottom {
                            self.dock_visible = true;
                        } else if position.y < self.screen_size.height - threshold_hide {
                            self.dock_visible = false;
                        }

                        if let Some((app_id, offset)) = self.dragging {
                            if let Some(state) = self.window_states.get_mut(&app_id) {
                                state.x = position.x - offset.x;
                                state.y = position.y - offset.y;
                            }
                        }
                    }
                    iced::Event::Mouse(iced::mouse::Event::ButtonReleased(
                        iced::mouse::Button::Left,
                    )) => {
                        self.dragging = None;
                    }
                    _ => {}
                }

                // Keyboard Events (using reference to avoid move)
                if let iced::Event::Keyboard(kb_event) = &event {
                    match kb_event {
                        iced::keyboard::Event::KeyPressed { key, modifiers, .. } => {
                            // Magnet Snapping (Option + Arrows)
                            if modifiers.alt() {
                                let top_app = self.z_order.last().cloned();
                                if let Some(app_id) = top_app {
                                    self.handle_snapping(app_id, key.clone());
                                }
                            }

                            // Omnibar (Cmd/Ctrl + Space)
                            if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) =
                                key
                            {
                                if modifiers.command() || modifiers.control() {
                                    return Task::done(Message::ToggleOmnibar);
                                }
                            }

                            // Switcher (Cmd/Alt + Tab)
                            if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) = key
                            {
                                if modifiers.command() || modifiers.alt() {
                                    if !self.show_switcher {
                                        self.show_switcher = true;
                                    } else {
                                        self.switcher.next();
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

                // Close Switcher on Modifiers Released is tricky in current Iced without tracking state
                // We'll use Escape for now or a generic release if available
                if let iced::Event::Keyboard(iced::keyboard::Event::KeyReleased {
                    key: _, ..
                }) = event
                {
                    // If no command/alt held, we close
                    // Note: modifiers might not be accurate in KeyReleased depending on platform
                    // self.show_switcher = false;
                }
                Task::none()
            }
            Message::Tick => {
                self.vector_bg.clear_cache();
                if self.last_monitor_update.elapsed() > std::time::Duration::from_millis(500) {
                    self.system.refresh_cpu();
                    self.system.refresh_memory();

                    let cpu = self.system.global_cpu_info().cpu_usage();
                    let mem_used =
                        self.system.used_memory() as f32 / self.system.total_memory() as f32;

                    self.cortex_state.push_data(cpu, mem_used);
                    self.last_monitor_update = std::time::Instant::now();
                }
                Task::none()
            }
            Message::Exit => Task::none(),
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
                        self.show_app_grid = false; // Always close launchpad
                        match app_id {
                            crate::registry::AppId::Terminal => {
                                return Task::done(Message::ToggleTerminal);
                            }
                            crate::registry::AppId::Browser => {
                                BrowserApp::open("https://duckduckgo.com");
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
                            crate::registry::AppId::WebOS => {
                                self.mode = ShellMode::Peak; // Changed from Matrix
                            }
                            crate::registry::AppId::Stremio => {
                                std::process::Command::new("open")
                                    .arg("/Applications/Stremio.app")
                                    .spawn()
                                    .ok();
                            }
                            crate::registry::AppId::Vlc => {
                                std::process::Command::new("open")
                                    .arg("/Applications/VLC.app")
                                    .spawn()
                                    .ok();
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
                                    // Assuming typical MacOS naming
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
                        }
                    }
                }
                Task::none()
            }
            Message::Settings(msg) => {
                match msg {
                    crate::apps::settings::SettingsMessage::ThemeChanged(mode) => {
                        self.settings.theme_mode = mode;
                        self.theme = match mode {
                            crate::apps::settings::ThemeMode::Light => crate::theme::Theme::Light,
                            crate::apps::settings::ThemeMode::Dark => crate::theme::Theme::Dark,
                        };
                    }
                    _ => self.settings.update(msg),
                }
                Task::none()
            }
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
                match msg {
                    crate::apps::turntable::JukeboxMessage::PlayTrack(item) => {
                        self.jukebox.current_track = Some(item.clone());
                        self.jukebox.is_playing = true;
                        let cmd = item.launch_command.replace("play ", "").replace("\"", "");
                        crate::audio::play_track(cmd);
                    }
                    crate::apps::turntable::JukeboxMessage::TogglePlayback => {
                        self.jukebox.is_playing = !self.jukebox.is_playing;
                        crate::audio::toggle_playback();
                    }
                    crate::apps::turntable::JukeboxMessage::NextTrack => {
                        println!("Jukebox: Next Track requested");
                    }
                    crate::apps::turntable::JukeboxMessage::PrevTrack => {
                        println!("Jukebox: Prev Track requested");
                    }
                }
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
                crate::components::spaces_strip::SpacesMessage::SwitchTo(mode) => {
                    return Task::done(Message::SwitchMode(mode));
                }
                crate::components::spaces_strip::SpacesMessage::SwitchDesktop(idx) => {
                    return Task::done(Message::SwitchDesktop(idx));
                }
            },
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
                    OmnibarMessage::Confirm => {
                        if let Some(res) = self.omnibar.get_selected() {
                            self.show_omnibar = false;
                            if let Some(app_id) = res.app_id {
                                Task::done(Message::DockInteraction(dock::DockMessage::Launch(
                                    app_id,
                                )))
                            } else if let Some(path) = &res.path {
                                std::process::Command::new("sh")
                                    .arg("-c")
                                    .arg(format!("open '{}'", path))
                                    .spawn()
                                    .ok();
                                Task::none()
                            } else {
                                Task::none()
                            }
                        } else {
                            Task::none()
                        }
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
                self.library.is_open = !self.library.is_open;
                if self.library.is_open {
                    self.ensure_window_state(crate::registry::AppId::Library, 800.0, 600.0);
                } else {
                    self.close_window(crate::registry::AppId::Library);
                }
                Task::none()
            }
            Message::ToggleJukebox => {
                self.jukebox.is_open = !self.jukebox.is_open;
                if self.jukebox.is_open {
                    self.ensure_window_state(crate::registry::AppId::Turntable, 400.0, 600.0);
                } else {
                    self.close_window(crate::registry::AppId::Turntable);
                }
                Task::none()
            }
            Message::ToggleSettings => {
                self.show_settings = !self.show_settings;
                if self.show_settings {
                    self.ensure_window_state(crate::registry::AppId::Settings, 600.0, 400.0);
                } else {
                    self.close_window(crate::registry::AppId::Settings);
                }
                Task::none()
            }
            Message::ToggleSystemMenu => {
                self.show_system_menu = !self.show_system_menu;
                Task::none()
            }
            Message::ToggleExplorer => {
                let is_open = self
                    .window_states
                    .contains_key(&crate::registry::AppId::FileManager);
                if !is_open {
                    self.ensure_window_state(crate::registry::AppId::FileManager, 600.0, 450.0);
                } else {
                    self.close_window(crate::registry::AppId::FileManager);
                }
                Task::none()
            }
            Message::ToggleStore => {
                let is_open = self
                    .window_states
                    .contains_key(&crate::registry::AppId::Store);
                if !is_open {
                    self.ensure_window_state(crate::registry::AppId::Store, 800.0, 600.0);
                } else {
                    self.close_window(crate::registry::AppId::Store);
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
                let is_open = self
                    .window_states
                    .contains_key(&crate::registry::AppId::Terminal);
                if !is_open {
                    self.ensure_window_state(crate::registry::AppId::Terminal, 640.0, 480.0);
                } else {
                    self.close_window(crate::registry::AppId::Terminal);
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

    pub fn view_desktop(&self) -> Element<'_, Message> {
        let is_light = self.settings.theme_mode == crate::apps::settings::ThemeMode::Light;
        let wallpaper_path = match (self.mode, is_light) {
            (ShellMode::Peak, true) => {
                crate::utils::assets::get_asset_path("wallpapers/mountain_classic_light.jpg")
            }
            (ShellMode::Peak, false) => {
                crate::utils::assets::get_asset_path("wallpapers/mountain_classic.jpg")
            }
            (ShellMode::Poolside, true) => {
                crate::utils::assets::get_asset_path("wallpapers/poolsuite_luxury.jpg")
            }
            (ShellMode::Poolside, false) => {
                crate::utils::assets::get_asset_path("wallpapers/poolsuite_luxury_night.jpg")
            }
        };

        use crate::components::window_chrome;

        // Dynamic z-order Workspace Rendering
        let mut workspace_stack = Stack::new();

        for &app_id in &self.z_order {
            if let Some(state) = self.window_states.get(&app_id) {
                let content: Element<'_, Message> = match app_id {
                    crate::registry::AppId::Terminal => {
                        self.terminal.view(is_light).map(Message::Terminal)
                    }
                    crate::registry::AppId::Library => {
                        self.library.view(&self.games).map(Message::Library)
                    }
                    crate::registry::AppId::Turntable => {
                        self.jukebox.view(&self.games).map(Message::Jukebox)
                    }
                    crate::registry::AppId::Settings => {
                        self.settings.view(is_light).map(Message::Settings)
                    }
                    crate::registry::AppId::FileManager => {
                        self.explorer.view(is_light).map(Message::Explorer)
                    }
                    crate::registry::AppId::Store => self.store.view(is_light).map(Message::Store),
                    _ => container(iced::widget::text("UNSUPPORTED")).into(),
                };

                let title = match app_id {
                    crate::registry::AppId::Terminal => "System Console",
                    crate::registry::AppId::Library => "The Arcade",
                    crate::registry::AppId::Turntable => "The Jukebox",
                    crate::registry::AppId::Settings => "Core Sync",
                    crate::registry::AppId::FileManager => "File System",
                    crate::registry::AppId::Store => "App Store",
                    _ => "Application",
                };

                let on_close = match app_id {
                    crate::registry::AppId::Terminal => Message::ToggleTerminal,
                    crate::registry::AppId::Library => Message::ToggleArcade,
                    crate::registry::AppId::Turntable => Message::ToggleJukebox,
                    crate::registry::AppId::Settings => Message::ToggleSettings,
                    crate::registry::AppId::FileManager => Message::ToggleExplorer,
                    crate::registry::AppId::Store => Message::ToggleStore,
                    _ => Message::Exit,
                };

                workspace_stack = workspace_stack.push(
                    container(
                        container(window_chrome::view(title, content, on_close))
                            .width(state.width)
                            .height(state.height),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(iced::Padding {
                        top: state.y,
                        left: state.x,
                        ..Default::default()
                    })
                    .align_x(iced::alignment::Horizontal::Left)
                    .align_y(iced::alignment::Vertical::Top),
                );
            }
        }

        let main_content = container(workspace_stack)
            .width(Length::Fill)
            .height(Length::Fill);

        let apps = if self.mode == ShellMode::Poolside {
            crate::registry::AppInfo::library()
        } else {
            crate::registry::AppInfo::productivity()
        };
        let dock_element = dock::view(&apps, self.mode, is_light).map(Message::DockInteraction);

        let global_ui = Stack::new()
            .push(
                container(main_content)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(
                container(menubar::view(self.mode, is_light).map(Message::MenubarAction))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(iced::alignment::Vertical::Top),
            )
            .push(
                container(if self.dock_visible {
                    Element::from(dock_element)
                } else {
                    iced::widget::Space::with_height(1.0).into()
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .padding(20)
                .align_y(iced::alignment::Vertical::Bottom),
            );

        let main_ui = desktop_container::view(&wallpaper_path, global_ui.into());

        let mut main_stack = iced::widget::Stack::new().push(main_ui);

        if self.show_omnibar {
            main_stack = main_stack
                .push(
                    container(iced::widget::text(""))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(|_| container::Style {
                            background: Some(iced::Background::Color(Color::from_rgba(
                                0.0, 0.0, 0.0, 0.5,
                            ))),
                            ..Default::default()
                        }),
                )
                .push(
                    container(self.omnibar.view().map(Message::Omnibar))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill),
                );
        }

        if self.show_app_grid {
            main_stack = main_stack
                .push(
                    container(
                        // Backdrop
                        button(text(""))
                            .style(move |_, _| button::Style {
                                background: Some(if is_light {
                                    iced::Color::from_rgb8(242, 242, 247).into()
                                // Solid Light
                                } else {
                                    iced::Color::from_rgb8(20, 20, 20).into() // Solid Dark
                                }),
                                ..Default::default()
                            })
                            .on_press(Message::ToggleAppGrid)
                            .width(Length::Fill)
                            .height(Length::Fill),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill),
                )
                .push(
                    container(view_app_grid(is_light))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill),
                );
        }

        if self.show_switcher {
            main_stack = main_stack
                .push(
                    container(iced::widget::text(""))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(|_| container::Style {
                            background: Some(iced::Background::Color(Color::from_rgba(
                                0.0, 0.0, 0.0, 0.2,
                            ))),
                            ..Default::default()
                        }),
                )
                .push(
                    container(self.switcher.view().map(Message::Switcher))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill),
                );
        }

        if self.show_spaces_selector {
            main_stack = main_stack.push(
                container(
                    crate::components::spaces_strip::view(self.mode, self.current_desktop)
                        .map(Message::SwitchSpace),
                )
                .width(Length::Fill)
                .padding(iced::Padding {
                    top: 50.0,
                    ..Default::default()
                }) // 40px menubar + 10px breathing
                .center_x(Length::Fill)
                .align_y(iced::alignment::Vertical::Top),
            );
        }

        if self.show_system_menu || self.show_reality_menu || self.show_wifi_menu {
            let (text_color, bg) = match self.mode {
                ShellMode::Peak => (
                    Color::from_rgb8(35, 30, 30),    // Warm Deep Grey
                    Color::from_rgb8(247, 245, 242), // Warm Stone Paper
                ),
                ShellMode::Poolside => (
                    Color::from_rgb8(50, 50, 50),    // Retro Dark Grey Text
                    Color::from_rgb8(255, 153, 204), // Retro Pink Bar
                ),
            };

            let menu_button = |label: String, msg: Message, active: bool| -> Element<'_, Message> {
                button(
                    container(text(label).size(13))
                        .width(Length::Fill)
                        .padding([5, 10]),
                )
                .on_press(msg)
                .style(move |_, status| {
                    let is_hovered = status == iced::widget::button::Status::Hovered;
                    button::Style {
                        background: if active {
                            Some(Color::from_rgba(0.0, 0.4, 0.9, 0.8).into()) // Active blue
                        } else if is_hovered {
                            Some(Color::from_rgba(1.0, 1.0, 1.0, 0.1).into())
                        } else {
                            None
                        },
                        text_color: if active { Color::WHITE } else { text_color },
                        border: iced::Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .into()
            };

            if self.show_system_menu {
                let menu = container(
                    iced::widget::column![
                        menu_button("About PeakOS".into(), Message::Navigate(Page::Home), false),
                        menu_button(
                            "System Preferences...".into(),
                            Message::ToggleSettings,
                            false
                        ),
                        iced::widget::Space::with_height(5.0),
                        container(iced::widget::Space::with_height(0.5))
                            .width(Length::Fill)
                            .style(move |_| container::Style {
                                background: Some(
                                    Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.2)
                                        .into()
                                ),
                                ..Default::default()
                            }),
                        iced::widget::Space::with_height(5.0),
                        menu_button("Log Out...".into(), Message::LogOut, false),
                        menu_button("Factory Reset...".into(), Message::FactoryReset, false),
                        menu_button("Restart...".into(), Message::Restart, false),
                        menu_button("Shut Down...".into(), Message::Exit, false), // Uses exit(0) for now
                    ]
                    .width(Length::Fixed(180.0))
                    .padding(5),
                )
                .style(move |_| container::Style {
                    background: Some(Color::from_rgba(bg.r, bg.g, bg.b, 0.92).into()),
                    border: iced::Border {
                        color: Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.1),
                        width: 1.0,
                        radius: 12.0.into(),
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: iced::Vector::new(0.0, 8.0),
                        blur_radius: 16.0,
                    },
                    ..Default::default()
                });

                main_stack = main_stack.push(
                    container(menu)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(iced::Padding {
                            top: 40.0,
                            left: 10.0,
                            ..Default::default()
                        })
                        .align_x(iced::alignment::Horizontal::Left)
                        .align_y(iced::alignment::Vertical::Top),
                );
            }

            if self.show_reality_menu {
                let menu = container(
                    iced::widget::column![
                        menu_button(
                            "Peak".into(),
                            Message::SwitchMode(ShellMode::Peak),
                            self.mode == ShellMode::Peak
                        ),
                        menu_button(
                            "Riviera".into(),
                            Message::SwitchMode(ShellMode::Poolside),
                            self.mode == ShellMode::Poolside
                        ),
                    ]
                    .width(Length::Fixed(140.0))
                    .padding(5),
                )
                .style(move |_| container::Style {
                    background: Some(Color::from_rgba(bg.r, bg.g, bg.b, 0.92).into()),
                    border: iced::Border {
                        color: Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.1),
                        width: 1.0,
                        radius: 12.0.into(),
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: iced::Vector::new(0.0, 8.0),
                        blur_radius: 16.0,
                    },
                    ..Default::default()
                });

                main_stack = main_stack.push(
                    container(menu)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(iced::Padding {
                            top: 40.0,
                            left: 60.0, // Positioned under the mode name
                            ..Default::default()
                        })
                        .align_x(iced::alignment::Horizontal::Left)
                        .align_y(iced::alignment::Vertical::Top),
                );
            }

            if self.show_wifi_menu {
                // Header
                let mut content = iced::widget::column![
                    text("Wi-Fi Networks").size(12).style(move |_| text::Style {
                        color: Some(Color::from_rgba(
                            text_color.r,
                            text_color.g,
                            text_color.b,
                            0.6
                        ))
                    }),
                    iced::widget::Space::with_height(5.0),
                    container(iced::widget::Space::with_height(0.5))
                        .width(Length::Fill)
                        .style(move |_| container::Style {
                            background: Some(
                                Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.2)
                                    .into()
                            ),
                            ..Default::default()
                        }),
                    iced::widget::Space::with_height(5.0),
                ];

                // Network List
                for network in &self.networks {
                    content = content.push(
                        menu_button(
                            network.clone(),
                            Message::MenubarAction(MenubarMessage::ToggleWifiMenu),
                            network == "PeakOS_5G",
                        ), // Mock active
                    );
                }

                let menu =
                    container(content.width(Length::Fixed(200.0)).padding(10)).style(move |_| {
                        container::Style {
                            background: Some(Color::from_rgba(bg.r, bg.g, bg.b, 0.92).into()),
                            border: iced::Border {
                                color: Color::from_rgba(
                                    text_color.r,
                                    text_color.g,
                                    text_color.b,
                                    0.1,
                                ),
                                width: 1.0,
                                radius: 12.0.into(),
                            },
                            shadow: iced::Shadow {
                                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                                offset: iced::Vector::new(0.0, 8.0),
                                blur_radius: 16.0,
                            },
                            ..Default::default()
                        }
                    });

                main_stack = main_stack.push(
                    container(menu)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(iced::Padding {
                            top: 40.0,
                            right: 10.0, // Right aligned
                            ..Default::default()
                        })
                        .align_x(iced::alignment::Horizontal::Right)
                        .align_y(iced::alignment::Vertical::Top),
                );
            }
        }

        main_stack.into()
    }
    pub fn view(&self) -> Element<'_, Message> {
        match &self.state {
            AppState::Setup(wizard_state) => self.view_setup(wizard_state),
            AppState::Login(_) => self.view_login_new(),
            AppState::Desktop => self.view_desktop(),
        }
    }

    fn view_login_new(&self) -> Element<'_, Message> {
        let user_name = self
            .user
            .as_ref()
            .map(|u| u.full_name.clone())
            .unwrap_or("User".to_string());

        let avatar_icon_key = self.user.as_ref().and_then(|u| u.avatar_icon.clone());

        let is_light = self.settings.theme_mode == crate::apps::settings::ThemeMode::Light;
        let text_color = if is_light { Color::BLACK } else { Color::WHITE };

        // Theme Toggle SVG Handle
        let toggle_icon = if is_light {
            // Moon (Black for Light Mode)
            iced::widget::svg::Handle::from_memory(br#"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                </svg>
            "#.to_vec())
        } else {
            // Sun (White for Dark Mode)
            iced::widget::svg::Handle::from_memory(br#"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/>
                    <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
                    <line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/>
                    <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
                </svg>
            "#.to_vec())
        };

        let theme_btn = button(
            iced::widget::svg(toggle_icon)
                .width(Length::Fixed(24.0))
                .height(Length::Fixed(24.0)),
        )
        .on_press(Message::ToggleTheme)
        .padding(10)
        .style(move |_, status| {
            let bg_alpha = if status == iced::widget::button::Status::Hovered {
                0.2
            } else {
                0.0
            };
            iced::widget::button::Style {
                background: Some(Color::from_rgba(0.5, 0.5, 0.5, bg_alpha).into()),
                ..Default::default()
            }
        });

        let top_right = container(theme_btn)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right)
            .align_y(iced::alignment::Vertical::Top)
            .padding(20);

        // 1. Avatar Circle
        let avatar_content: Element<'_, Message> = if let Some(key) = avatar_icon_key {
            let handle = crate::icons::get_avatar_handle(&key, "black");
            container(
                iced::widget::svg(handle)
                    .width(Length::Fixed(70.0))
                    .height(Length::Fixed(70.0)),
            )
            .into()
        } else {
            container(
                text(user_name.chars().next().unwrap_or('?').to_string())
                    .size(60)
                    .style(move |_| text::Style {
                        color: Some(Color::BLACK),
                    }),
            )
            .into()
        };

        let avatar = container(avatar_content)
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(120.0))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(move |_| container::Style {
                // Subtle Glass effect for avatar
                background: Some(if is_light {
                    Color::from_rgba(0.0, 0.0, 0.0, 0.05).into()
                } else {
                    // "Monochrome Pop": White background for avatar in Dark Mode
                    Color::WHITE.into()
                }),
                border: iced::Border {
                    radius: 60.0.into(),
                    width: 1.0,
                    color: if is_light {
                        Color::from_rgba(0.0, 0.0, 0.0, 0.1)
                    } else {
                        Color::BLACK // Contrast border against white circle
                    },
                },
                ..Default::default()
            });

        let content = iced::widget::column![
            avatar,
            iced::widget::Space::with_height(20.0),
            // 2. Name
            text(user_name)
                .size(24)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .style(move |_| text::Style {
                    color: Some(text_color)
                }),
            iced::widget::Space::with_height(20.0),
            // 3. Password Input
            iced::widget::text_input(
                "Enter Password",
                if let AppState::Login(ref p) = self.state {
                    p
                } else {
                    ""
                }
            )
            .on_input(Message::UpdateLoginPassword)
            .on_submit(Message::SubmitLogin)
            .secure(true)
            .width(Length::Fixed(280.0))
            .padding(15)
            .style(move |_, status| crate::styles::style_soft_input(status, is_light)),
            iced::widget::Space::with_height(30.0),
            // 4. Login Button
            button(
                container(text("Login").size(16))
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center)
            )
            .on_press(Message::SubmitLogin)
            .padding([12, 50])
            .width(Length::Fixed(280.0)) // Match input width
            .style(move |_, status| crate::styles::style_pill_button(status, is_light))
        ]
        .align_x(iced::Alignment::Center)
        .spacing(0);

        // WRAP IN GLASS CARD
        let card = container(content)
            .padding(40)
            .style(move |_| crate::styles::style_glass_card(is_light));

        let centered_content = container(card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        // Vector Background Layer
        Stack::new()
            .push(self.vector_bg.view(is_light))
            .push(centered_content)
            .push(top_right)
            .into()
    }

    fn view_setup(&self, state: &crate::apps::wizard::WizardState) -> Element<'_, Message> {
        let is_light = self.settings.theme_mode == crate::apps::settings::ThemeMode::Light;

        // --- Steps ---

        let content = match state.current_step {
            crate::apps::wizard::WizardStep::Welcome => iced::widget::column![
                iced::widget::image(iced::widget::image::Handle::from_path(
                    crate::utils::assets::get_asset_path(if is_light {
                        "icons/menubar/peak_logo.png"
                    } else {
                        "icons/menubar/peak_logo_dark.png"
                    })
                ))
                .width(Length::Fixed(120.0)) // Slightly larger for main welcome screen
                .height(Length::Fixed(120.0)),
                text("Welcome to Peak.")
                    .size(36)
                    .font(iced::Font {
                        family: iced::font::Family::SansSerif,
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .style(move |_| text::Style {
                        color: Some(if is_light {
                            Color::from_rgb8(26, 26, 26)
                        } else {
                            Color::WHITE
                        })
                    }),
                text("Let's set up your new home.")
                    .size(18)
                    .style(move |_| text::Style {
                        color: Some(Color::from_rgb8(102, 102, 102)) // #666
                    }),
                iced::widget::Space::with_height(40.0),
                iced::widget::button(text("Get Started").size(16))
                    .on_press(Message::Wizard(
                        crate::apps::wizard::WizardMessage::NextStep
                    ))
                    .padding([15, 60])
                    .style(move |_, status| crate::styles::style_pill_button(status, is_light))
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
            crate::apps::wizard::WizardStep::Identity => {
                iced::widget::column![
                    text("Who are you?")
                        .size(24)
                        .font(iced::Font {
                            family: iced::font::Family::SansSerif,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(move |_| text::Style {
                            color: Some(if is_light {
                                Color::from_rgb8(26, 26, 26)
                            } else {
                                Color::WHITE
                            })
                        }),
                    iced::widget::Space::with_height(20.0),
                    // Group 1: Identity
                    iced::widget::column![
                        text("Full Name").size(12).style(move |_| text::Style {
                            color: Some(Color::from_rgb8(120, 120, 120))
                        }),
                        iced::widget::text_input("John Appleseed", &state.full_name_input)
                            .on_input(|s| Message::Wizard(
                                crate::apps::wizard::WizardMessage::UpdateFullName(s)
                            ))
                            .padding(12)
                            .style(move |_, status| crate::styles::style_soft_input(
                                status, is_light
                            )),
                        iced::widget::Space::with_height(10.0),
                        text("Account Name").size(12).style(move |_| text::Style {
                            color: Some(Color::from_rgb8(120, 120, 120))
                        }),
                        iced::widget::text_input("john", &state.username_input)
                            .on_input(|s| Message::Wizard(
                                crate::apps::wizard::WizardMessage::UpdateUsername(s)
                            ))
                            .padding(12)
                            .style(move |_, status| crate::styles::style_soft_input(
                                status, is_light
                            )),
                    ]
                    .spacing(5),
                    iced::widget::Space::with_height(20.0),
                    // Error Message
                    if let Some(error) = &state.error_message {
                        text(error.clone()).size(14).style(move |_| text::Style {
                            color: Some(Color::from_rgb8(255, 59, 48)), // System Red
                        })
                    } else {
                        text(" ").size(14)
                    },
                    iced::widget::Space::with_height(20.0),
                    // Buttons
                    iced::widget::row![
                        iced::widget::button(text("Back").size(16))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::PrevStep
                            ))
                            .style(move |_, status| crate::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Next").size(16))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::NextStep
                            ))
                            .padding([12, 40])
                            .style(move |_, status| crate::styles::style_pill_button(
                                status, is_light
                            ))
                    ]
                    .spacing(20)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::alignment::Horizontal::Center)
            }
            crate::apps::wizard::WizardStep::Security => {
                iced::widget::column![
                    text("Secure your account")
                        .size(24)
                        .font(iced::Font {
                            family: iced::font::Family::SansSerif,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(move |_| text::Style {
                            color: Some(if is_light {
                                Color::from_rgb8(26, 26, 26)
                            } else {
                                Color::WHITE
                            })
                        }),
                    iced::widget::Space::with_height(20.0),
                    // Group 2: Security
                    iced::widget::column![
                        text("Password").size(12).style(move |_| text::Style {
                            color: Some(Color::from_rgb8(120, 120, 120))
                        }),
                        iced::widget::text_input("Required", &state.password_input)
                            .on_input(|s| Message::Wizard(
                                crate::apps::wizard::WizardMessage::UpdatePassword(s)
                            ))
                            .padding(12)
                            .secure(true)
                            .style(move |_, status| crate::styles::style_soft_input(
                                status, is_light
                            )),
                        iced::widget::Space::with_height(10.0),
                        text("Verify").size(12).style(move |_| text::Style {
                            color: Some(Color::from_rgb8(120, 120, 120))
                        }),
                        iced::widget::text_input("Required", &state.password_confirm_input)
                            .on_input(|s| Message::Wizard(
                                crate::apps::wizard::WizardMessage::UpdatePasswordConfirm(s)
                            ))
                            .padding(12)
                            .secure(true)
                            .style(move |_, status| crate::styles::style_soft_input(
                                status, is_light
                            )),
                        iced::widget::Space::with_height(10.0),
                        text("Hint").size(12).style(move |_| text::Style {
                            color: Some(Color::from_rgb8(120, 120, 120))
                        }),
                        iced::widget::text_input("Optional", &state.password_hint_input)
                            .on_input(|s| Message::Wizard(
                                crate::apps::wizard::WizardMessage::UpdatePasswordHint(s)
                            ))
                            .padding(12)
                            .style(move |_, status| crate::styles::style_soft_input(
                                status, is_light
                            )),
                    ]
                    .spacing(5),
                    iced::widget::Space::with_height(20.0),
                    // Error Message
                    if let Some(error) = &state.error_message {
                        text(error.clone()).size(14).style(move |_| text::Style {
                            color: Some(Color::from_rgb8(255, 59, 48)), // System Red
                        })
                    } else {
                        text(" ").size(14)
                    },
                    iced::widget::Space::with_height(20.0),
                    // Buttons
                    iced::widget::row![
                        iced::widget::button(text("Back").size(16))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::PrevStep
                            ))
                            .style(move |_, status| crate::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Next").size(16))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::NextStep
                            ))
                            .padding([12, 40])
                            .style(move |_, status| crate::styles::style_pill_button(
                                status, is_light
                            ))
                    ]
                    .spacing(20)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::alignment::Horizontal::Center)
            }
            crate::apps::wizard::WizardStep::WifiConnect => {
                iced::widget::column![
                    text("Get Connected")
                        .size(24)
                        .font(iced::Font {
                            family: iced::font::Family::SansSerif,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(move |_| text::Style {
                            color: Some(if is_light {
                                Color::from_rgb8(26, 26, 26)
                            } else {
                                Color::WHITE
                            })
                        }),
                    text("Select a network to continue.")
                        .size(14)
                        .style(move |_| text::Style {
                            color: Some(Color::from_rgb8(102, 102, 102))
                        }),
                    iced::widget::Space::with_height(20.0),
                    // Valid Network List (Table View)
                    container(iced::widget::scrollable(iced::widget::column(
                        crate::integrations::network::get_available_networks()
                            .into_iter()
                            .enumerate()
                            .map(|(i, net)| {
                                let is_sel = Some(net.clone()) == state.selected_network;
                                let content = iced::widget::button(
                                    iced::widget::row![
                                        iced::widget::svg(crate::icons::wifi_full(if is_sel {
                                            if is_light {
                                                "white"
                                            } else {
                                                "black"
                                            }
                                        } else {
                                            if is_light {
                                                "black"
                                            } else {
                                                "white"
                                            }
                                        }))
                                        .width(Length::Fixed(16.0))
                                        .height(Length::Fixed(16.0)),
                                        text(net.clone()).size(14),
                                        iced::widget::horizontal_space(),
                                        // Lock icon placeholder
                                        // iced::widget::svg(crate::icons::lock(...)),
                                        if is_sel {
                                            text("✓").size(14)
                                        } else {
                                            text("").size(14)
                                        }
                                    ]
                                    .spacing(10)
                                    .align_y(iced::Alignment::Center),
                                )
                                .on_press(Message::Wizard(
                                    crate::apps::wizard::WizardMessage::SelectNetwork(net),
                                ))
                                .width(Length::Fill)
                                .padding(12)
                                .style(move |_, status| {
                                    if is_sel {
                                        iced::widget::button::Style {
                                            background: Some(if is_light {
                                                Color::BLACK.into()
                                            } else {
                                                Color::WHITE.into()
                                            }),
                                            text_color: if is_light {
                                                Color::WHITE
                                            } else {
                                                Color::BLACK
                                            },
                                            ..Default::default()
                                        }
                                    } else if status == iced::widget::button::Status::Hovered {
                                        iced::widget::button::Style {
                                            background: Some(if is_light {
                                                Color::from_rgba(0.0, 0.0, 0.0, 0.05).into()
                                            } else {
                                                Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()
                                            }),
                                            text_color: if is_light {
                                                Color::BLACK
                                            } else {
                                                Color::WHITE
                                            },
                                            ..Default::default()
                                        }
                                    } else {
                                        iced::widget::button::Style {
                                            background: Some(Color::TRANSPARENT.into()),
                                            text_color: if is_light {
                                                Color::BLACK
                                            } else {
                                                Color::WHITE
                                            },
                                            ..Default::default()
                                        }
                                    }
                                });

                                if i > 0 {
                                    iced::widget::column![
                                        iced::widget::Rule::horizontal(1).style(move |_| {
                                            iced::widget::rule::Style {
                                                color: Color::from_rgba(0.5, 0.5, 0.5, 0.2),
                                                width: 1,
                                                radius: 0.0.into(),
                                                fill_mode: iced::widget::rule::FillMode::Full,
                                            }
                                        }),
                                        content
                                    ]
                                    .into()
                                } else {
                                    content.into()
                                }
                            })
                            .collect::<Vec<_>>()
                    )))
                    .height(Length::Fixed(200.0))
                    .style(move |_| container::Style {
                        background: Some(Color::WHITE.into()),
                        border: iced::Border {
                            radius: 12.0.into(),
                            width: 1.0,
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        },
                        ..Default::default()
                    }),
                    iced::widget::Space::with_height(30.0),
                    iced::widget::row![
                        iced::widget::button(text("Back").size(14))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::PrevStep
                            ))
                            .style(move |_, status| crate::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Next").size(14))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::NextStep
                            ))
                            .padding([12, 40])
                            .style(move |_, status| crate::styles::style_pill_button(
                                status, is_light
                            )),
                    ]
                    .spacing(20)
                    .align_y(iced::Alignment::Center)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::Alignment::Center)
            }
            crate::apps::wizard::WizardStep::ThemeSelection => {
                iced::widget::column![
                    text("Personalize")
                        .size(24)
                        .font(iced::Font {
                            family: iced::font::Family::SansSerif,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(move |_| text::Style {
                            color: Some(if is_light {
                                Color::from_rgb8(26, 26, 26)
                            } else {
                                Color::WHITE
                            })
                        }),
                    text("Choose your look.")
                        .size(14)
                        .style(move |_| text::Style {
                            color: Some(Color::from_rgb8(102, 102, 102))
                        }),
                    iced::widget::Space::with_height(20.0),
                    // Avatar Section
                    text("Avatar").size(12).style(move |_| text::Style {
                        color: Some(Color::from_rgb8(120, 120, 120))
                    }),
                    iced::widget::row(
                        crate::icons::AVATAR_OPTIONS
                            .iter()
                            .map(|&key| {
                                let is_selected = state.selected_avatar.as_deref() == Some(key);
                                let color = match key {
                                    "robot" => Color::from_rgb8(0, 122, 255), // Blue
                                    "smile" => Color::from_rgb8(255, 204, 0), // Yellow
                                    "house" => Color::from_rgb8(52, 199, 89), // Green
                                    _ => Color::from_rgb8(142, 142, 147),     // Grey
                                };

                                iced::widget::button(
                                    container(iced::widget::Space::new(
                                        Length::Fixed(40.0),
                                        Length::Fixed(40.0),
                                    ))
                                    .style(move |_| {
                                        container::Style {
                                            background: Some(color.into()),
                                            border: iced::Border {
                                                radius: 20.0.into(),
                                                width: if is_selected { 2.0 } else { 0.0 },
                                                color: if is_light {
                                                    Color::BLACK
                                                } else {
                                                    Color::WHITE
                                                },
                                            },
                                            ..Default::default()
                                        }
                                    }),
                                )
                                .on_press(Message::Wizard(
                                    crate::apps::wizard::WizardMessage::SelectAvatar(
                                        key.to_string(),
                                    ),
                                ))
                                .padding(2)
                                .style(move |_, _status| {
                                    iced::widget::button::Style {
                                        background: None,
                                        border: iced::Border {
                                            radius: 24.0.into(), // Larger radius for hover ring if needed
                                            width: 0.0,
                                            color: Color::TRANSPARENT,
                                        },
                                        ..Default::default()
                                    }
                                })
                                .into()
                            })
                            .collect::<Vec<_>>()
                    )
                    .spacing(15),
                    iced::widget::Space::with_height(20.0),
                    // Theme Section
                    text("Theme").size(12).style(move |_| text::Style {
                        color: Some(Color::from_rgba(0.5, 0.5, 0.5, 0.8))
                    }),
                    iced::widget::row![
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(crate::icons::sun())
                                    .width(Length::Fixed(24.0))
                                    .height(Length::Fixed(24.0)),
                                text("Light").size(12)
                            ]
                            .align_x(iced::Alignment::Center)
                            .spacing(5)
                        )
                        .on_press(Message::ToggleTheme)
                        .width(Length::Fixed(80.0))
                        .padding(10)
                        .style(move |_, _| iced::widget::button::Style {
                            background: if is_light {
                                Some(Color::from_rgba(0.0, 0.48, 1.0, 0.1).into())
                            } else {
                                Some(Color::from_rgba(0.5, 0.5, 0.5, 0.1).into())
                            },
                            border: iced::Border {
                                radius: 12.0.into(),
                                width: if is_light { 2.0 } else { 0.0 },
                                color: Color::from_rgba(0.0, 0.48, 1.0, 1.0)
                            },
                            ..Default::default()
                        }),
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(crate::icons::moon())
                                    .width(Length::Fixed(24.0))
                                    .height(Length::Fixed(24.0)),
                                text("Dark").size(12)
                            ]
                            .align_x(iced::Alignment::Center)
                            .spacing(5)
                        )
                        .on_press(Message::ToggleTheme)
                        .width(Length::Fixed(80.0))
                        .padding(10)
                        .style(move |_, _| iced::widget::button::Style {
                            background: if !is_light {
                                Some(Color::from_rgba(0.0, 0.48, 1.0, 0.1).into())
                            } else {
                                Some(Color::from_rgba(0.5, 0.5, 0.5, 0.1).into())
                            },
                            border: iced::Border {
                                radius: 12.0.into(),
                                width: if !is_light { 2.0 } else { 0.0 },
                                color: Color::from_rgba(0.0, 0.48, 1.0, 1.0)
                            },
                            ..Default::default()
                        }),
                    ]
                    .spacing(10),
                    iced::widget::Space::with_height(40.0),
                    iced::widget::row![
                        iced::widget::button(text("Back").size(14))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::PrevStep
                            ))
                            .style(move |_, status| crate::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Finish").size(14))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::CompleteSetup
                            ))
                            .padding([12, 40])
                            .style(move |_, status| crate::styles::style_pill_button(
                                status, is_light
                            )),
                    ]
                    .spacing(20)
                    .align_y(iced::Alignment::Center)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::Alignment::Center)
            }
            crate::apps::wizard::WizardStep::Complete => iced::widget::column![
                iced::widget::svg(crate::icons::avatar_peak(if is_light {
                    "black"
                } else {
                    "white"
                }))
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0)),
                text("Account Created").size(24).font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                }),
                iced::widget::Space::with_height(20.0),
                iced::widget::button(text("Enter PeakOS").size(16))
                    .on_press(Message::SubmitLogin)
                    .padding([15, 60])
                    .style(move |_, status| crate::styles::style_pill_button(status, is_light))
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
        };

        let card = container(content)
            .padding(40)
            .style(move |_| crate::styles::style_glass_card(is_light));

        Stack::new()
            .push(self.vector_bg.view(is_light))
            .push(
                container(card)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
            )
            .into()
    }

    pub fn theme(&self) -> IcedTheme {
        match self.theme {
            crate::theme::Theme::Light => IcedTheme::Light,
            _ => IcedTheme::Dark,
        }
    }

    fn ensure_window_state(&mut self, app_id: crate::registry::AppId, w: f32, h: f32) {
        if !self.window_states.contains_key(&app_id) {
            let x = (self.screen_size.width - w) / 2.0;
            let y = (self.screen_size.height - h) / 2.0;
            self.window_states.insert(
                app_id,
                WindowState {
                    app_id,
                    x,
                    y,
                    width: w,
                    height: h,
                },
            );
        }
        if !self.z_order.contains(&app_id) {
            self.z_order.push(app_id);
        } else {
            // Move to front
            if let Some(pos) = self.z_order.iter().position(|&id| id == app_id) {
                self.z_order.remove(pos);
                self.z_order.push(app_id);
            }
        }
    }

    fn close_window(&mut self, app_id: crate::registry::AppId) {
        self.window_states.remove(&app_id);
        if let Some(pos) = self.z_order.iter().position(|&id| id == app_id) {
            self.z_order.remove(pos);
        }
    }

    fn handle_snapping(&mut self, app_id: crate::registry::AppId, key: iced::keyboard::Key) {
        if let Some(state) = self.window_states.get_mut(&app_id) {
            let menubar_h = 40.0;
            let dock_h = 100.0;
            let avail_w = self.screen_size.width;
            let avail_h = self.screen_size.height - menubar_h - dock_h;

            match key {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => {
                    state.x = 0.0;
                    state.y = menubar_h;
                    state.width = avail_w / 2.0;
                    state.height = avail_h;
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => {
                    state.x = avail_w / 2.0;
                    state.y = menubar_h;
                    state.width = avail_w / 2.0;
                    state.height = avail_h;
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                    state.x = 0.0;
                    state.y = menubar_h;
                    state.width = avail_w;
                    state.height = avail_h;
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                    state.x = 0.0;
                    state.y = menubar_h + (avail_h / 2.0);
                    state.width = avail_w;
                    state.height = avail_h / 2.0;
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => {
                    // Fullscreen
                    state.x = 0.0;
                    state.y = 0.0;
                    state.width = self.screen_size.width;
                    state.height = self.screen_size.height;
                }
                iced::keyboard::Key::Character(c) if c.to_lowercase() == "c" => {
                    // Center
                    state.width = 800.0;
                    state.height = 600.0;
                    state.x = (self.screen_size.width - state.width) / 2.0;
                    state.y = (self.screen_size.height - state.height) / 2.0;
                }
                _ => {}
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(vec![
            iced::time::every(std::time::Duration::from_millis(16)).map(|_| Message::Tick),
            self.terminal.subscription().map(Message::Terminal),
            iced::event::listen().map(Message::GlobalEvent),
        ])
    }
}

fn view_app_grid<'a>(is_light: bool) -> Element<'a, Message> {
    let apps = crate::registry::AppInfo::all();

    // Close Button (Top Right)
    let close_btn =
        button(
            text("Close")
                .size(16)
                .color(if is_light { Color::BLACK } else { Color::WHITE }),
        )
        .on_press(Message::ToggleAppGrid)
        .style(|_, _| button::Style {
            background: Some(Color::TRANSPARENT.into()),
            ..Default::default()
        })
        .padding(10);

    let top_bar = Row::new()
        .push(iced::widget::Space::with_width(Length::Fill))
        .push(close_btn)
        .padding(20);

    let mut grid_col = Column::new()
        .spacing(30)
        .align_x(iced::alignment::Horizontal::Center);

    // Chunk items into rows of 6
    for chunk in apps.chunks(6) {
        let mut row_el = Row::new()
            .spacing(40)
            .align_y(iced::alignment::Vertical::Top);
        for app in chunk {
            // Icon Logic (Themed)
            let icon_path =
                if app.icon_path.contains("icons/") && !app.icon_path.contains("riviera/") {
                    let base_name = std::path::Path::new(app.icon_path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    let theme_subdir = if is_light { "black" } else { "white" };

                    // Try themed version first
                    let themed_path = if base_name.ends_with(".svg") {
                        format!("assets/icons/menubar/{}/{}", theme_subdir, base_name)
                    } else {
                        String::new()
                    };

                    if !themed_path.is_empty() && std::path::Path::new(&themed_path).exists() {
                        themed_path
                    } else {
                        app.icon_path.to_string()
                    }
                } else {
                    app.icon_path.to_string()
                };

            let icon: Element<Message> =
                if icon_path.ends_with(".png") || icon_path.ends_with(".jpg") {
                    iced::widget::image(icon_path)
                        .width(Length::Fixed(64.0))
                        .height(Length::Fixed(64.0))
                        .into()
                } else {
                    svg(svg::Handle::from_path(icon_path))
                        .width(Length::Fixed(64.0))
                        .height(Length::Fixed(64.0))
                        .into()
                };

            let btn = button(
                Column::new()
                    .push(
                        container(icon)
                            .width(Length::Fixed(80.0))
                            .height(Length::Fixed(80.0))
                            .align_x(iced::alignment::Horizontal::Center)
                            .align_y(iced::alignment::Vertical::Center)
                            .style(move |_| container::Style {
                                background: Some(if is_light {
                                    iced::Color::from_rgba(0.0, 0.0, 0.0, 0.05).into()
                                } else {
                                    iced::Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()
                                }),
                                border: iced::Border {
                                    radius: 20.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                    )
                    .push(
                        text(app.name)
                            .size(14)
                            .color(if is_light { Color::BLACK } else { Color::WHITE })
                            .align_x(iced::alignment::Horizontal::Center)
                            .width(Length::Fixed(100.0)),
                    )
                    .spacing(10)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .on_press(Message::DockInteraction(
                crate::components::dock::DockMessage::Launch(app.id),
            ))
            .style(button::text);

            row_el = row_el.push(btn);
        }
        grid_col = grid_col.push(row_el);
    }

    container(
        Column::new().push(top_bar).push(scrollable(
            container(grid_col)
                .padding(50)
                .width(Length::Fill)
                .center_x(Length::Fill),
        )),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
