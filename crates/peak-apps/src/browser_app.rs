// In-process browser manager (launches peak-browser binary)
use iced::{Element, Task};
use peak_core::app_traits::{PeakApp, ShellContext, WindowInfo};
use peak_core::theme::Theme;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

pub struct BrowserApp {
    child_process: Option<Arc<Mutex<Child>>>,
    url: String,
    window_info: WindowInfo,
    is_open: bool,
}

#[derive(Debug, Clone)]
pub enum BrowserMessage {
    Navigate(String),
    Close,
    LayoutUpdate(f32, f32), // (root_x, root_y)
}

impl BrowserApp {
    pub fn new() -> Self {
        Self {
            child_process: None,
            url: String::from("https://duckduckgo.com"),
            window_info: WindowInfo::default(),
            is_open: false,
        }
    }

    pub fn launch(&mut self, context: &dyn ShellContext) {
        if self.child_process.is_some() {
            return;
        }

        // Determine path to the binary
        // In dev: target/debug/peak-browser
        // In prod: adjacent to peak-desktop
        let current_exe = std::env::current_exe().unwrap_or_default();
        let bin_dir = current_exe.parent().unwrap_or(std::path::Path::new("."));
        let browser_path = bin_dir.join("peak-browser");

        // Calculate absolute position
        let (root_x, root_y) = context.get_root_window_position();
        let browser_x = root_x + self.window_info.x + 2.0;
        let browser_y = root_y + self.window_info.y + 34.0; // 32px Header + 2px offset
        let browser_width = self.window_info.width - 4.0;
        let browser_height = self.window_info.height - 36.0;

        match Command::new(&browser_path)
            .arg("--url")
            .arg(&self.url)
            .arg("--x")
            .arg(browser_x.to_string())
            .arg("--y")
            .arg(browser_y.to_string())
            .arg("--width")
            .arg(browser_width.to_string())
            .arg("--height")
            .arg(browser_height.to_string())
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            Ok(child) => {
                self.child_process = Some(Arc::new(Mutex::new(child)));
                self.is_open = true;
                println!(
                    "Successfully launched peak-browser (PID: {})",
                    self.child_process.as_ref().unwrap().lock().unwrap().id()
                );
            }
            Err(e) => {
                eprintln!("Failed to launch peak-browser at {:?}: {}", browser_path, e);
                eprintln!("Ensure 'peak-browser' is built: run `cargo build -p peak-browser`");
            }
        }
    }

    pub fn kill(&mut self) {
        if let Some(child_arc) = &self.child_process {
            let mut child = child_arc.lock().unwrap();
            let _ = child.kill();
        }
        self.child_process = None;
        self.is_open = false;
    }
}

impl PeakApp for BrowserApp {
    type Message = BrowserMessage;

    fn title(&self) -> String {
        String::from("Web Browser")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        match message {
            BrowserMessage::Navigate(url) => {
                self.url = url.clone();
                if self.child_process.is_none() {
                    self.launch(_context);
                } else if let Some(child_arc) = &mut self.child_process {
                    if let Ok(mut child) = child_arc.lock() {
                        if let Some(stdin) = &mut child.stdin {
                            use std::io::Write;
                            let cmd = crate::browser::BrowserCommand::Navigate { url };
                            if let Ok(json) = serde_json::to_string(&cmd) {
                                let _ = writeln!(stdin, "{}", json);
                            }
                        }
                    }
                }
            }
            BrowserMessage::Close => self.kill(),
            BrowserMessage::LayoutUpdate(root_x, root_y) => {
                if let Some(child_arc) = &mut self.child_process {
                    // Calculate absolute position (Safe Insets)
                    let browser_x = root_x + self.window_info.x + 2.0;
                    let browser_y = root_y + self.window_info.y + 34.0; // 32px Header + 2px offset
                    let browser_width = self.window_info.width - 4.0;
                    let browser_height = self.window_info.height - 36.0;

                    let cmd = crate::browser::BrowserCommand::Layout {
                        x: browser_x as f64,
                        y: browser_y as f64,
                        width: browser_width as f64,
                        height: browser_height as f64,
                    };

                    if let Ok(mut child) = child_arc.lock() {
                        if let Some(stdin) = &mut child.stdin {
                            use std::io::Write;
                            if let Ok(json) = serde_json::to_string(&cmd) {
                                let _ = writeln!(stdin, "{}", json);
                            }
                        }
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self, is_light: &Theme) -> Element<'_, Self::Message> {
        let is_light_mode = *is_light == Theme::Light;

        // Clean view for Integrated Mode
        // We just show a blank container that matches the theme bg
        // The external process window will overlay this.
        iced::widget::container(iced::widget::Space::new(
            iced::Length::Fill,
            iced::Length::Fill,
        ))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(move |_: &_| iced::widget::container::Style {
            background: Some(if is_light_mode {
                iced::Color::WHITE.into()
            } else {
                iced::Color::from_rgb8(30, 30, 30).into()
            }),
            ..Default::default()
        })
        .into()
    }

    fn window_info(&self) -> WindowInfo {
        self.window_info
    }

    fn on_window_change(&mut self, info: WindowInfo) {
        self.window_info = info;
    }
}
