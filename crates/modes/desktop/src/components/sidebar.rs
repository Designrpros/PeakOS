#![allow(dead_code)]
use crate::app::Message;
use crate::pages::Page;
use iced::widget::{button, column, container, text};
use iced::{Alignment, Element, Length, Theme as IcedTheme};
use peak_core::theme::Theme;

pub fn view<'a>(current_page: &Page, theme: &Theme) -> Element<'a, Message, IcedTheme> {
    let palette = theme.palette();
    let theme_clone = *theme; // Theme is Copy

    let content = column![
        text("PeakOS")
            .size(24)
            .font(iced::font::Font::MONOSPACE)
            .style(move |_| text::Style {
                color: Some(palette.primary)
            }),
        iced::widget::Space::new().height(40),
        nav_button(
            "Library",
            Page::Library,
            matches!(current_page, Page::Library),
            palette,
            theme_clone
        ),
        nav_button(
            "Cortex",
            Page::Cortex,
            matches!(current_page, Page::Cortex),
            palette,
            theme_clone
        ),
        nav_button(
            "Settings",
            Page::Settings,
            matches!(current_page, Page::Settings),
            palette,
            theme_clone
        ),
        iced::widget::Space::new().height(Length::Fill), // Push bottom items down
        button(text("Theme Toggle"))
            .on_press(Message::ToggleTheme)
            .style(move |_, status| secondary_button_style(status, palette)),
        button(text("Exit"))
            .on_press(Message::Exit)
            .style(move |_, status| danger_button_style(status, palette)),
    ]
    .padding(20)
    .spacing(10)
    .width(200)
    .height(Length::Fill)
    .align_x(Alignment::Center);

    container(content)
        .style(move |_| container::Style {
            background: Some(palette.surface.into()),
            text_color: Some(palette.text),
            ..Default::default()
        })
        .into()
}

fn nav_button<'a>(
    label: &'a str,
    page: Page,
    active: bool,
    palette: peak_core::theme::Palette,
    _theme: Theme,
) -> Element<'a, Message, IcedTheme> {
    button(text(label).size(16).style(move |_| text::Style {
        color: Some(if active {
            palette.background
        } else {
            palette.text
        }),
    }))
    .width(Length::Fill)
    .padding(10)
    .on_press(Message::Navigate(page))
    .style(move |_, status| {
        let base = button::Style {
            background: if active {
                Some(palette.primary.into())
            } else {
                None
            },
            text_color: if active {
                palette.background
            } else {
                palette.text
            },
            border: iced::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            snap: false,
            ..Default::default()
        };

        match status {
            button::Status::Hovered => button::Style {
                background: if active {
                    Some(palette.primary.into())
                } else {
                    Some(
                        iced::Color {
                            a: 0.1,
                            ..palette.text
                        }
                        .into(),
                    )
                },
                ..base
            },
            _ => base,
        }
    })
    .into()
}

fn secondary_button_style(
    status: button::Status,
    palette: peak_core::theme::Palette,
) -> button::Style {
    button::Style {
        background: match status {
            button::Status::Hovered => Some(
                iced::Color {
                    a: 0.2,
                    ..palette.text
                }
                .into(),
            ),
            _ => Some(
                iced::Color {
                    a: 0.1,
                    ..palette.text
                }
                .into(),
            ),
        },
        text_color: palette.text,
        border: iced::Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn danger_button_style(
    status: button::Status,
    _palette: peak_core::theme::Palette,
) -> button::Style {
    button::Style {
        background: match status {
            button::Status::Hovered => Some(iced::Color::from_rgb8(255, 50, 50).into()),
            _ => Some(iced::Color::from_rgb8(200, 30, 30).into()),
        },
        text_color: iced::Color::WHITE,
        border: iced::Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
