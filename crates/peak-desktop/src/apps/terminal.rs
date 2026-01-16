#![allow(dead_code, unused_imports)]

use iced::widget::{container, scrollable, text, text_input, Column};
use iced::{Background, Color, Element, Length, Task};
use peak_core::app_traits::AppTheme;
pub use peak_core::apps::terminal::{TerminalApp, TerminalMessage};

pub trait TerminalDesktopView {
    fn view<'a>(
        &'a self,
        theme: &AppTheme,
    ) -> Element<'a, TerminalMessage, iced::Theme, iced::Renderer>;
}

impl TerminalDesktopView for TerminalApp {
    fn view<'a>(
        &'a self,
        theme: &AppTheme,
    ) -> Element<'a, TerminalMessage, iced::Theme, iced::Renderer> {
        let text_color = theme.text_color;
        let bg_color = theme.bg_color;
        let border_color = theme.border_color;

        let output = text(&self.content)
            .font(iced::Font::MONOSPACE)
            .size(12)
            .color(text_color);

        let input = text_input("Type a command...", &self.input_buffer)
            .on_input(TerminalMessage::InputChanged)
            .on_submit(TerminalMessage::InputSubmitted)
            .padding(10)
            .size(12)
            .font(iced::Font::MONOSPACE)
            .style(move |_, _| text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border: iced::Border {
                    width: 0.0,
                    radius: 0.0.into(),
                    color: Color::TRANSPARENT,
                },
                icon: Color::TRANSPARENT,
                placeholder: text_color,
                value: text_color,
                selection: text_color,
            });

        let term_content = Column::new()
            .push(scrollable(output).height(Length::Fill).width(Length::Fill))
            .push(
                container(input)
                    .padding(5)
                    .style(move |_| container::Style {
                        border: iced::Border {
                            width: 1.0,
                            color: border_color,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    }),
            );

        container(term_content)
            .padding(8)
            .style(move |_| container::Style {
                background: Some(bg_color.into()),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub fn strip_ansi(input: &str) -> String {
    peak_core::apps::terminal::strip_ansi(input)
}
