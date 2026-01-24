use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::{Element, Length, Renderer, Theme};

pub struct ScrollView<Message: 'static, B: Backend = IcedBackend> {
    content: Box<dyn View<Message, B>>,
    width: Length,
    height: Length,
}

impl<Message: 'static> ScrollView<Message, IcedBackend> {
    pub fn new(content: impl View<Message, IcedBackend> + 'static) -> Self {
        Self::new_generic(content)
    }

    pub fn from_boxed(content: Box<dyn View<Message, IcedBackend>>) -> Self {
        Self {
            content,
            width: Length::Fill,
            height: Length::Fill,
        }
    }
}

impl<Message: 'static> ScrollView<Message, TermBackend> {
    pub fn new_tui(content: impl View<Message, TermBackend> + 'static) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static, B: Backend> ScrollView<Message, B> {
    pub fn new_generic(content: impl View<Message, B> + 'static) -> Self {
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

impl<Message: 'static> View<Message, IcedBackend> for ScrollView<Message, IcedBackend> {
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

impl<Message: 'static> View<Message, TermBackend> for ScrollView<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        self.content.view(context)
    }
}

impl<Message: 'static> ScrollView<Message, IcedBackend> {
    pub fn apply_style<'a>(
        s: iced::widget::Scrollable<'a, Message, Theme, Renderer>,
        theme: &peak_theme::ThemeTokens,
    ) -> iced::widget::Scrollable<'a, Message, Theme, Renderer> {
        let text_color = theme.colors.text_primary;
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
                        radius: if cfg!(target_arch = "wasm32") {
                            0.0
                        } else {
                            4.0
                        }
                        .into(),
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
                        radius: if cfg!(target_arch = "wasm32") {
                            0.0
                        } else {
                            4.0
                        }
                        .into(),
                        ..Default::default()
                    },
                },
            },
            gap: None,
        })
    }
}
