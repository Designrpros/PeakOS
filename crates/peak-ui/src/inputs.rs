use crate::core::{Context, View};
use iced::widget::{container, text_input};
use iced::{Color, Element, Length, Renderer, Theme};
use std::sync::Arc;

pub struct TextField<Message> {
    label: String,
    value: String,
    placeholder: String,
    on_change: Arc<dyn Fn(String) -> Message + Send + Sync>,
    is_secure: bool,
    width: Length,
}

impl<Message> TextField<Message> {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        on_change: impl Fn(String) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            placeholder: String::new(),
            on_change: Arc::new(on_change),
            is_secure: false,
            width: Length::Fill,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn secure(mut self, secure: bool) -> Self {
        self.is_secure = secure;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

impl<Message: Clone + 'static> View<Message> for TextField<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let on_change = self.on_change.clone();

        let input = text_input(&self.placeholder, &self.value)
            .on_input(move |s| (on_change)(s))
            .secure(self.is_secure)
            .padding(12)
            .width(Length::Fill)
            .style(move |_theme, status| {
                let active_border = if matches!(status, text_input::Status::Focused) {
                    theme.colors.primary
                } else {
                    Color::TRANSPARENT
                };

                text_input::Style {
                    background: theme.colors.surface_variant.into(),
                    border: iced::Border {
                        radius: 8.0.into(),
                        width: 1.0,
                        color: active_border,
                    },
                    icon: theme.colors.text_secondary,
                    placeholder: theme.colors.text_secondary,
                    value: theme.colors.text_primary,
                    selection: theme.colors.primary,
                }
            });

        container(
            iced::widget::column![
                iced::widget::text(self.label.clone())
                    .size(12)
                    .color(theme.colors.text_secondary),
                input
            ]
            .spacing(4),
        )
        .width(self.width)
        .into()
    }
}
