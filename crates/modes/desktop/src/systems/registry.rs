use crate::app::Message;
use iced::{Element, Subscription, Task};
use peak_core::app_traits::{PeakApp, ShellContext, WindowInfo};
use peak_core::registry::AppId;
use std::collections::HashMap;

/// Context implementation for apps running within the desktop environment
pub struct DesktopShellContext {
    pub _app_id: AppId,
    pub shell_position: (f32, f32),
}

impl DesktopShellContext {
    pub fn new(_app_id: AppId, shell_position: (f32, f32)) -> Self {
        Self {
            _app_id,
            shell_position,
        }
    }
}

impl ShellContext for DesktopShellContext {
    fn notify(&self, title: &str, message: &str) {
        println!("[Notification] {}: {}", title, message);
    }

    fn ask_intelligence(&self, _prompt: &str) -> Task<String> {
        // This will eventually call peak-intelligence
        Task::none()
    }

    fn close_app(&self) {
        // Signal to the shell to close this app
    }

    fn request_file_system(&self) -> bool {
        true
    }

    fn get_root_window_position(&self) -> (f32, f32) {
        self.shell_position
    }
}

/// A type-erased container for any PeakApp
pub trait RunningApp {
    fn title(&self) -> String;
    fn update(&mut self, message: Message, context: &DesktopShellContext) -> Task<Message>;
    fn view(&self, theme: &peak_core::theme::Theme) -> Element<'_, Message>;
    fn subscription(&self) -> Subscription<Message>;
    #[allow(dead_code)]
    fn window_info(&self) -> WindowInfo;
    #[allow(dead_code)]
    fn on_window_change(&mut self, info: WindowInfo);
}

/// Helper to wrap a PeakApp and map its messages
pub struct AppWrapper<App, Msg, M, U>
where
    App: PeakApp<Message = Msg>,
    Msg: Clone + std::fmt::Debug + Send + Sync + 'static,
    M: Fn(Msg) -> Message + Copy + Send + Sync + 'static,
    U: Fn(Message) -> Option<Msg> + Copy + Send + Sync + 'static,
{
    pub app: App,
    pub map_msg: M,
    pub try_unmap: U,
}

impl<App, Msg, M, U> RunningApp for AppWrapper<App, Msg, M, U>
where
    App: PeakApp<Message = Msg>,
    Msg: Clone + std::fmt::Debug + Send + Sync + 'static,
    M: Fn(Msg) -> Message + Copy + Send + Sync + 'static,
    U: Fn(Message) -> Option<Msg> + Copy + Send + Sync + 'static,
{
    fn title(&self) -> String {
        self.app.title()
    }

    fn update(&mut self, message: Message, context: &DesktopShellContext) -> Task<Message> {
        if let Some(app_msg) = (self.try_unmap)(message) {
            return self.app.update(app_msg, context).map(self.map_msg);
        }
        Task::none()
    }

    fn view(&self, theme: &peak_core::theme::Theme) -> Element<'_, Message> {
        self.app.view(theme).map(self.map_msg)
    }

    fn subscription(&self) -> Subscription<Message> {
        // Must use the function item directly to be a ZST
        self.app.subscription().map(self.map_msg)
    }

    fn window_info(&self) -> WindowInfo {
        self.app.window_info()
    }

    fn on_window_change(&mut self, info: WindowInfo) {
        self.app.on_window_change(info);
    }
}

pub struct AppRegistry {
    pub running_apps: HashMap<AppId, Box<dyn RunningApp>>,
}

impl AppRegistry {
    pub fn new() -> Self {
        Self {
            running_apps: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: AppId, app: Box<dyn RunningApp>) {
        self.running_apps.insert(id, app);
    }

    #[allow(dead_code)]
    pub fn unregister(&mut self, id: &AppId) {
        self.running_apps.remove(id);
    }
}
