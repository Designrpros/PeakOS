use iced::widget::{button, column, container, row, scrollable, svg, text};
use iced::{Element, Length};
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

impl ExplorerApp {
    pub fn new() -> Self {
        Self {
            current_path: dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
            history: Vec::new(),
        }
    }

    pub fn view<'a>(&self, is_light: bool) -> Element<'a, ExplorerMessage> {
        let (text_color, icon_color) = if is_light {
            (
                iced::Color::from_rgb8(35, 30, 30),
                iced::Color::from_rgb8(100, 100, 100),
            )
        } else {
            (
                iced::Color::from_rgb8(235, 230, 225),
                iced::Color::from_rgb8(150, 150, 150),
            )
        };

        let title = text(format!("{}", self.current_path.display()))
            .size(11)
            .color(icon_color);

        let mut items = column![].spacing(2);

        if let Ok(entries) = std::fs::read_dir(&self.current_path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let path = entry.path();

                items = items.push(
                    button(
                        row![view_icon(&path, is_light), text(name).size(12),]
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

        let back_button = button(text("←").size(14))
            .on_press(ExplorerMessage::Back)
            .style(button::text);

        container(
            column![
                row![back_button, title]
                    .spacing(12)
                    .align_y(iced::Alignment::Center),
                scrollable(items).height(Length::Fill)
            ]
            .spacing(8),
        )
        .padding(8)
        .into()
    }

    pub fn update(&mut self, message: ExplorerMessage) {
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
    }
}

fn view_icon<'a>(path: &PathBuf, is_light: bool) -> Element<'a, ExplorerMessage> {
    let is_dir = path.is_dir();
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let icon_name = if is_dir {
        "folder.svg"
    } else {
        match ext.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "svg" => "image.svg",
            "mp3" | "wav" | "flac" | "ogg" => "media.svg",
            "rs" | "sh" | "toml" | "json" | "md" | "txt" => "document.svg",
            _ => "document.svg",
        }
    };

    let theme_dir = if is_light { "black" } else { "white" };
    // Construct absolute path using get_asset_path helper
    let relative_themed = format!("icons/menubar/{}/{}", theme_dir, icon_name);
    let abs_themed = crate::utils::assets::get_asset_path(&relative_themed);

    let final_path = if std::path::Path::new(&abs_themed).exists() {
        abs_themed
    } else {
        crate::utils::assets::get_asset_path(&format!("icons/{}", icon_name))
    };

    container(
        svg(svg::Handle::from_path(final_path))
            .width(Length::Fixed(16.0))
            .height(Length::Fixed(16.0)),
    )
    .width(Length::Fixed(20.0))
    .align_x(iced::alignment::Horizontal::Center)
    .into()
}
