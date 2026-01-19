use iced::widget::{button, column, container, row, scrollable, svg, text};
use iced::{Element, Length, Task};
use peak_core::theme::Theme;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ExplorerMessage {
    Navigate(PathBuf),
    Back,
}

pub struct ExplorerApp {
    pub current_path: PathBuf,
    pub history: Vec<PathBuf>,
}

impl Default for ExplorerApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ExplorerApp {
    pub fn new() -> Self {
        Self {
            current_path: dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
            history: Vec::new(),
        }
    }

    pub fn view(&self, theme: &Theme) -> Element<'_, ExplorerMessage, iced::Theme, iced::Renderer> {
        let palette = theme.palette();
        let text_color = palette.text;
        let is_light = *theme == Theme::Light;
        let icon_color = if is_light {
            iced::Color::from_rgb8(100, 100, 100)
        } else {
            iced::Color::from_rgb8(150, 150, 150)
        };

        let title = text(format!("{}", self.current_path.display()))
            .size(11)
            .color(icon_color);

        let mut items = column![].spacing(2);

        if let Ok(entries) = std::fs::read_dir(&self.current_path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let path = entry.path();

                let is_dir = path.is_dir();
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                items = items.push(
                    button(
                        row![view_icon(is_dir, &ext, is_light), text(name).size(12),]
                            .spacing(8)
                            .align_y(iced::Alignment::Center),
                    )
                    .on_press(ExplorerMessage::Navigate(path))
                    .style(move |_, status| button::Style {
                        background: if status == iced::widget::button::Status::Hovered {
                            Some(iced::Color::from_rgba(0.5, 0.5, 0.5, 0.1).into())
                        } else {
                            None
                        },
                        text_color,
                        ..Default::default()
                    })
                    .padding(4)
                    .width(Length::Fill),
                );
            }
        }

        let mut sidebar = column![].spacing(8).width(Length::Fixed(120.0));
        let shortcuts = [
            ("Home", dirs::home_dir()),
            ("Desktop", dirs::desktop_dir()),
            ("Documents", dirs::document_dir()),
            ("Downloads", dirs::download_dir()),
        ];

        for (name, path_opt) in shortcuts {
            if let Some(path) = path_opt {
                sidebar = sidebar.push(
                    button(text(name).size(12))
                        .on_press(ExplorerMessage::Navigate(path.clone()))
                        .style(move |_, status| button::Style {
                            background: if self.current_path == path {
                                Some(iced::Color::from_rgba(0.5, 0.5, 0.5, 0.2).into())
                            } else if status == iced::widget::button::Status::Hovered {
                                Some(iced::Color::from_rgba(0.5, 0.5, 0.5, 0.1).into())
                            } else {
                                None
                            },
                            text_color,
                            ..Default::default()
                        })
                        .padding(4)
                        .width(Length::Fill),
                );
            }
        }

        let back_button = button(text("←").size(14))
            .on_press(ExplorerMessage::Back)
            .style(button::text);

        let sidebar_elem: Element<ExplorerMessage> = sidebar.into();
        let divider: Element<ExplorerMessage> =
            container(iced::widget::Space::new(Length::Fixed(1.0), Length::Fill))
                .style(move |_| container::Style {
                    background: Some(icon_color.scale_alpha(0.1).into()),
                    ..Default::default()
                })
                .into();

        let content: Element<ExplorerMessage> = column![
            row![back_button, title]
                .spacing(12)
                .align_y(iced::Alignment::Center),
            scrollable(items).height(Length::Fill)
        ]
        .spacing(8)
        .width(Length::Fill)
        .into();

        row![sidebar_elem, divider, content]
            .spacing(12)
            .padding(8)
            .into()
    }

    pub fn update(&mut self, message: ExplorerMessage) -> Task<ExplorerMessage> {
        match message {
            ExplorerMessage::Navigate(path) => {
                if path.is_dir() {
                    self.history.push(self.current_path.clone());
                    self.current_path = path;
                }
            }
            ExplorerMessage::Back => {
                if let Some(prev) = self.history.pop() {
                    self.current_path = prev;
                }
            }
        }
        Task::none()
    }
}

fn view_icon<'a>(is_dir: bool, ext: &str, is_light: bool) -> Element<'a, ExplorerMessage> {
    let icon_name = if is_dir {
        "folder"
    } else {
        match ext {
            "png" | "jpg" | "jpeg" | "gif" | "svg" => "image",
            "mp3" | "wav" | "flac" | "ogg" => "media",
            "rs" | "sh" | "toml" | "json" | "md" | "txt" => "document",
            _ => "document",
        }
    };

    let icon_color = if is_light { "#000000" } else { "#FFFFFF" };

    container(
        svg(peak_core::icons::get_ui_icon(icon_name, icon_color))
            .width(Length::Fixed(16.0))
            .height(Length::Fixed(16.0)),
    )
    .width(Length::Fixed(20.0))
    .align_x(iced::alignment::Horizontal::Center)
    .into()
}

use peak_core::app_traits::{PeakApp, ShellContext};

impl PeakApp for ExplorerApp {
    type Message = ExplorerMessage;

    fn title(&self) -> String {
        String::from("Explorer")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        self.update(message)
    }

    fn view(&self, theme: &Theme) -> Element<'_, Self::Message> {
        self.view(theme)
    }
}
