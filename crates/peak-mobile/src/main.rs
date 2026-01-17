use iced::{Element, Size};
use peak_core::app_traits::{PeakApp, ShellContext};
use peak_core::registry::AppId;

mod view_mobile;

pub fn main() -> iced::Result {
    iced::application("Peak Mobile", PeakMobile::update, PeakMobile::view)
        .window(iced::window::Settings {
            size: Size::new(390.0, 844.0), // iPhone 12/13/14 aspect ratio
            resizable: true,
            ..Default::default()
        })
        .subscription(PeakMobile::subscription)
        .run()
}

struct PeakMobile {
    is_light: bool,
    running_apps: Vec<AppId>,
    current_app: Option<AppId>,
    time: String,
    terminal: peak_core::apps::terminal::TerminalApp,
    settings: peak_core::apps::settings::SettingsApp,
}

#[derive(Debug, Clone)]
enum Message {
    OpenApp(AppId),
    GoHome,
    Tick,
    Terminal(peak_core::apps::terminal::TerminalMessage),
    Settings(peak_core::apps::settings::SettingsMessage),
}

impl Default for PeakMobile {
    fn default() -> Self {
        Self {
            is_light: false,
            running_apps: Vec::new(),
            current_app: None,
            time: "12:00".to_string(),
            terminal: peak_core::apps::terminal::TerminalApp::new(),
            settings: peak_core::apps::settings::SettingsApp::new(),
        }
    }
}

struct MobileShellContext {
    _app_id: AppId,
}

impl ShellContext for MobileShellContext {
    fn notify(&self, title: &str, message: &str) {
        println!("[Mobile Notification] {}: {}", title, message);
    }

    fn ask_intelligence(&self, _prompt: &str) -> iced::Task<String> {
        iced::Task::none()
    }

    fn close_app(&self) {
        // Handle mobile app close
    }

    fn request_file_system(&self) -> bool {
        false // Stricter on mobile
    }
}

impl PeakMobile {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::OpenApp(id) => self.current_app = Some(id),
            Message::GoHome => self.current_app = None,
            Message::Tick => {
                self.time = chrono::Local::now().format("%H:%M").to_string();
            }
            Message::Terminal(msg) => {
                let context = MobileShellContext {
                    _app_id: AppId::Terminal,
                };
                return self.terminal.update(msg, &context).map(Message::Terminal);
            }
            Message::Settings(msg) => {
                let context = MobileShellContext {
                    _app_id: AppId::Settings,
                };
                return self.settings.update(msg, &context).map(Message::Settings);
            }
        }
        iced::Task::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(vec![
            iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick),
            self.terminal.subscription().map(Message::Terminal),
        ])
    }

    fn view(&self) -> Element<'_, Message> {
        view_mobile::view(
            self.is_light,
            &self.running_apps,
            self.current_app,
            &self.time,
            &self.terminal,
            &self.settings,
            Message::OpenApp,
            Message::GoHome,
            Message::Terminal,
            Message::Settings,
        )
    }
}
