use crate::core::{Context, View};
use iced::{Element, Length, Renderer, Theme};

pub struct ScrollView<Message> {
    content: Box<dyn View<Message>>,
    width: Length,
    height: Length,
}

impl<Message> ScrollView<Message> {
    pub fn new(content: impl View<Message> + 'static) -> Self {
        Self {
            content: Box::new(content),
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<Message: 'static> View<Message> for ScrollView<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        Self::apply_style(
            iced::widget::scrollable(self.content.view(context)),
            &context.theme,
        )
        .width(self.width)
        .height(self.height)
        .into()
    }
}

impl<Message> ScrollView<Message> {
    pub fn apply_style<'a>(
        s: iced::widget::Scrollable<'a, Message, Theme, Renderer>,
        theme: &peak_theme::ThemeTokens,
    ) -> iced::widget::Scrollable<'a, Message, Theme, Renderer> {
        let text_color = theme.text;
        s.style(move |_, _| iced::widget::scrollable::Style {
            container: iced::widget::container::Style::default(),
            vertical_rail: iced::widget::scrollable::Rail {
                background: None,
                border: iced::Border::default(),
                scroller: iced::widget::scrollable::Scroller {
                    color: iced::Color {
                        a: 0.2,
                        ..text_color
                    },
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                },
            },
            horizontal_rail: iced::widget::scrollable::Rail {
                background: None,
                border: iced::Border::default(),
                scroller: iced::widget::scrollable::Scroller {
                    color: iced::Color {
                        a: 0.2,
                        ..text_color
                    },
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                },
            },
            gap: None,
        })
    }
}
