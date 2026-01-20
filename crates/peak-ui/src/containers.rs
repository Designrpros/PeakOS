use crate::core::{Context, View};
use iced::widget::{column, container, text};
use iced::{Element, Length, Padding, Renderer, Theme};

pub struct Card<Message> {
    content: Box<dyn View<Message>>,
    padding: Padding,
    width: Length,
    height: Length,
}

impl<Message> Card<Message> {
    pub fn new(content: impl View<Message> + 'static) -> Self {
        Self {
            content: Box::new(content),
            padding: Padding::from(16),
            width: Length::Shrink,
            height: Length::Shrink,
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

impl<Message: 'static> View<Message> for Card<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        // Base container with shadow and outer border
        container(
            // Inner container for the "inner border" highlight effect
            container(self.content.view(context))
                .padding(self.padding)
                .width(self.width)
                .height(self.height)
                .style(move |_theme| container::Style {
                    background: Some(theme.colors.surface.into()),
                    border: iced::Border {
                        radius: theme.radius.into(),
                        color: theme.colors.border.scale_alpha(0.5),
                        width: 1.0,
                    },
                    text_color: Some(theme.colors.text_primary),
                    ..Default::default()
                }),
        )
        .width(self.width)
        .height(self.height)
        .style(move |_theme| container::Style {
            border: iced::Border {
                radius: theme.radius.into(),
                color: theme.colors.border,
                width: 1.0,
            },
            shadow: iced::Shadow {
                color: theme.shadow_color,
                offset: iced::Vector::new(theme.shadow_offset[0], theme.shadow_offset[1]),
                blur_radius: theme.shadow_blur,
            },
            ..Default::default()
        })
        .into()
    }
}

pub struct Section<Message> {
    title: String,
    content: Box<dyn View<Message>>,
    width: Length,
    height: Length,
}

impl<Message> Section<Message> {
    pub fn new(title: impl Into<String>, content: impl View<Message> + 'static) -> Self {
        Self {
            title: title.into(),
            content: Box::new(content),
            width: Length::Fill,
            height: Length::Shrink,
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

impl<Message: 'static> View<Message> for Section<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        container(
            column![
                text(self.title.clone())
                    .size(12)
                    .color(context.theme.colors.text_primary.scale_alpha(0.6)),
                self.content.view(context)
            ]
            .spacing(8),
        )
        .width(self.width)
        .height(self.height)
        .into()
    }
}
