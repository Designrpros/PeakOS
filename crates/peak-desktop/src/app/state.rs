// State type definitions for PeakNative application

use crate::components::app_switcher::SwitcherMessage;
use crate::components::inspector::InspectorMessage;
use crate::components::menubar::MenubarMessage;
use crate::components::omnibar::OmnibarMessage;
use peak_apps::explorer::ExplorerMessage;
use peak_apps::library::LibraryMessage;
use peak_apps::settings::SettingsMessage;
use peak_core::registry::ShellMode;

#[derive(Debug, Clone)]
pub enum AppState {
    Setup(peak_apps::wizard::WizardState),
    Login(String), // Password Input
    Desktop,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    Library(LibraryMessage),
    Navigate(crate::pages::Page),
    ToggleTheme,
    LaunchGame(String),
    Tick,
    Exit,
    GlobalEvent(iced::Event),
    DockInteraction(crate::components::dock::DockMessage),
    SwitchMode(ShellMode),
    ToggleMode,
    LogOut,
    MenubarAction(MenubarMessage),
    Settings(SettingsMessage),
    Inspector(InspectorMessage),
    ToggleInspector,
    Jukebox(peak_apps::jukebox::JukeboxMessage),
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
    ToggleStore,
    Store(peak_apps::store::StoreMessage),
    Browser(peak_apps::browser_app::BrowserMessage),
    LaunchBrowser(String),
    CloseBrowser,
    Desktop(crate::components::desktop::DesktopMessage),
    Editor(peak_apps::editor::EditorMessage),
    ToggleEditor,
    Maximize(peak_core::registry::AppId),
    WindowPositionFound(Option<iced::Point>),
    CloseAlert,
    Terminal(peak_apps::terminal::TerminalMessage),
    Restart,
    Wizard(peak_apps::wizard::WizardMessage),
    UpdateLoginPassword(String),
    SubmitLogin,
    FactoryReset,
    ToggleAppGrid,
}
