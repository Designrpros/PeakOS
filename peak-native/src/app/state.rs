// State type definitions for PeakNative application

use crate::apps::explorer::ExplorerMessage;
use crate::apps::library::LibraryMessage;
use crate::apps::settings::SettingsMessage;
use crate::components::app_switcher::SwitcherMessage;
use crate::components::inspector::InspectorMessage;
use crate::components::menubar::MenubarMessage;
use crate::components::omnibar::OmnibarMessage;
use crate::registry::ShellMode;

#[derive(Debug, Clone)]
pub enum AppState {
    Setup(crate::apps::wizard::WizardState),
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
    Jukebox(crate::apps::jukebox::JukeboxMessage),
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
    Store(crate::apps::store::StoreMessage),
    LaunchBrowser(String),
    CloseBrowser,
    Desktop(crate::components::desktop::DesktopMessage),
    Editor(crate::apps::editor::EditorMessage),
    ToggleEditor,
    Maximize(crate::registry::AppId),
    WindowPositionFound(Option<iced::Point>),
    CloseAlert,
    Terminal(crate::apps::terminal::TerminalMessage),
    Restart,
    Wizard(crate::apps::wizard::WizardMessage),
    UpdateLoginPassword(String),
    SubmitLogin,
    FactoryReset,
    ToggleAppGrid,
}
