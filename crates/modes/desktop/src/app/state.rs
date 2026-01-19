// State type definitions for PeakNative application

use crate::components::inspector::InspectorMessage;
use crate::components::omnibar::OmnibarMessage;
use peak_apps::explorer::ExplorerMessage;
use peak_apps::library::LibraryMessage;
use peak_apps::settings::SettingsMessage;
use peak_core::registry::ShellMode;
use peak_shell::app_switcher::SwitcherMessage;
use peak_shell::menubar::MenubarMessage;

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
    DockInteraction(peak_shell::dock::DockMessage),
    SwitchMode(ShellMode),
    SwitchShellStyle(peak_core::registry::ShellStyle), // Switch theme (Cupertino/Redmond/AI)
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
    // Browser removed - using Firefox via opener::open
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
    AssistantBooted(Result<peak_intelligence::brain::Assistant, peak_intelligence::brain::Error>),
    AssistantReply(
        peak_intelligence::brain::assistant::Reply,
        peak_intelligence::brain::assistant::Token,
    ),
    AssistantFinished,
    ConsoleCategory(peak_shell::console::category_bar::CategoryBarMessage),
    ConsoleGame(peak_shell::console::game_rail::GameRailMessage),
    TVApp(peak_shell::tv::app_rail::AppRailMessage),
    RedmondTaskbar(peak_shell::redmond::taskbar::TaskbarMessage),
}
