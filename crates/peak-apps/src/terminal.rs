#![allow(dead_code, unused_imports)]

use iced::widget::{container, scrollable, text, text_input, Column};
use iced::{Background, Color, Element, Length, Task};
pub use peak_core::apps::terminal::{TerminalApp, TerminalMessage};
use peak_core::theme::Theme;

pub trait TerminalDesktopView {
    fn view<'a>(
        &'a self,
        theme: &Theme,
    ) -> Element<'a, TerminalMessage, iced::Theme, iced::Renderer>;
}

impl TerminalDesktopView for TerminalApp {
    fn view<'a>(
        &'a self,
        theme: &Theme,
    ) -> Element<'a, TerminalMessage, iced::Theme, iced::Renderer> {
        let palette = theme.palette();
        let text_color = palette.text;

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

        let input_row = iced::widget::row![
            text("> ")
                .font(iced::Font::MONOSPACE)
                .color(text_color)
                .size(12),
            input
        ]
        .spacing(0)
        .align_y(iced::Alignment::Center);

        let term_content = Column::new()
            .push(scrollable(output).height(Length::Fill).width(Length::Fill))
            .push(input_row);

        container(term_content)
            .padding([8, 12])
            .style(move |_| container::Style {
                background: Some(Color::TRANSPARENT.into()),
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
// Wrapper for Registry
pub struct DesktopTerminalApp(pub TerminalApp);
impl Default for DesktopTerminalApp {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopTerminalApp {
    pub fn new() -> Self {
        Self(TerminalApp::new())
    }
}

use peak_core::app_traits::{PeakApp, ShellContext};

impl PeakApp for DesktopTerminalApp {
    type Message = TerminalMessage;

    fn title(&self) -> String {
        self.0.title()
    }

    fn update(
        &mut self,
        message: Self::Message,
        context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        self.0.update(message, context)
    }

    fn view(&self, theme: &Theme) -> Element<'_, Self::Message> {
        TerminalDesktopView::view(&self.0, theme)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        self.0.subscription()
    }
}
