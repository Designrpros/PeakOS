use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length, Task};
use peak_core::models::{MediaItem, MediaStatus};
use peak_core::theme::Theme;

#[derive(Debug, Clone)]
pub enum LibraryMessage {
    LaunchItem(String), // Renamed from LaunchGame
    ImageLoaded(String, iced::widget::image::Handle),
    ImageLoadFailed(#[allow(dead_code)] String),
    // TabChanged(MediaKind),
}

pub struct LibraryApp {
    pub items: Vec<MediaItem>,
}

impl LibraryApp {
    pub fn new(items: Vec<MediaItem>) -> Self {
        Self { items }
    }

    pub fn view(&self, _theme: &Theme) -> Element<'_, LibraryMessage> {
        Self::view_items(&self.items)
    }

    fn view_items<'a>(items: &'a [MediaItem]) -> Element<'a, LibraryMessage> {
        // Show all items (Games, Apps, etc.)
        let mut filtered_items = items.iter().peekable();

        if filtered_items.peek().is_none() {
            return container(
                column![
                    text("NO APPS DETECTED")
                        .size(18)
                        .color(Color::from_rgb(1.0, 0.4, 0.7)),
                    text("INSTALL APPS FROM THE APP STORE")
                        .size(12)
                        .color(Color::WHITE),
                ]
                .spacing(10)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        }

        let grid = row(filtered_items.map(view_card))
            .spacing(20)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Shrink)
            .wrap();

        container(scrollable(grid))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    #[allow(dead_code)]
    pub fn update(&mut self, _message: LibraryMessage) {
        // No-op for now as tabs are handled by the Dock/Apps
    }
}

use peak_core::app_traits::{PeakApp, ShellContext};

impl PeakApp for LibraryApp {
    type Message = LibraryMessage;

    fn title(&self) -> String {
        String::from("Library")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        self.update(message);
        Task::none()
    }

    fn view(&self, theme: &Theme) -> Element<'_, Self::Message> {
        self.view(theme)
    }
}

pub fn view_card<'a>(item: &'a MediaItem) -> Element<'a, LibraryMessage> {
    use iced::widget::image;
    let status_color = match item.status {
        MediaStatus::Ready => Color::from_rgb(0.0, 1.0, 0.8), // Cyan
        MediaStatus::Running => Color::from_rgb(0.0, 1.0, 0.0), // Green
        MediaStatus::Updating(_) => Color::from_rgb(1.0, 0.8, 0.0), // Amber
    };

    let status_text = match item.status {
        MediaStatus::Ready => "READY".to_string(),
        MediaStatus::Running => "RUNNING".to_string(),
        MediaStatus::Updating(p) => format!("UPDATING {:.0}%", p * 100.0),
    };

    // Card Content: Image OR Placeholder
    let content: Element<_> = if let Some(handle) = &item.image_handle {
        image(handle.clone())
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Cover)
            .into()
    } else {
        column![
            text(&item.title).size(20).color(Color::WHITE),
            text(status_text).size(12).color(status_color)
        ]
        .spacing(10)
        .align_x(Alignment::Center)
        .into()
    };

    let cover = container(content)
        .width(Length::Fixed(200.0))
        .height(Length::Fixed(300.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba8(0, 0, 0, 0.6))),
            border: iced::Border {
                color: Color::from_rgb(1.0, 0.4, 0.7), // Retro Pink Border
                width: 2.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        });

    button(cover)
        .on_press(LibraryMessage::LaunchItem(item.launch_command.clone()))
        .padding(0)
        .style(|_theme, status| {
            let base = button::Style {
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            };
            match status {
                button::Status::Hovered => button::Style {
                    shadow: iced::Shadow {
                        color: Color::from_rgb(1.0, 0.4, 0.7),
                        offset: iced::Vector::new(0.0, 0.0),
                        blur_radius: 10.0,
                    },
                    ..base
                },
                _ => base,
            }
        })
        .into()
}
