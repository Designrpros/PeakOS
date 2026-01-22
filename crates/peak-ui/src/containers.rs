use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::widget::{column, container, text};
use iced::{Element, Length, Padding, Renderer, Theme};

pub struct Card<Message: 'static, B: Backend = IcedBackend> {
    content: Box<dyn View<Message, B>>,
    padding: Padding,
    width: Length,
    height: Length,
}

impl<Message: 'static> Card<Message, IcedBackend> {
    pub fn new(content: impl View<Message, IcedBackend> + 'static) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static> Card<Message, TermBackend> {
    pub fn new_tui(content: impl View<Message, TermBackend> + 'static) -> Self {
        Self::new_generic(content)
    }
}

impl<Message: 'static, B: Backend> Card<Message, B> {
    pub fn new_generic(content: impl View<Message, B> + 'static) -> Self {
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

impl<Message: 'static> View<Message, IcedBackend> for Card<Message, IcedBackend> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        container(
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

impl<Message: 'static> View<Message, TermBackend> for Card<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        let inner = self.content.view(context);
        let lines: Vec<&str> = inner.lines().collect();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0) + 4;

        let mut out = String::new();
        out.push_str("┌");
        out.push_str(&"─".repeat(width - 2));
        out.push_str("┐\n");

        for line in lines {
            out.push_str("│ ");
            out.push_str(line);
            out.push_str(&" ".repeat(width - 4 - line.len()));
            out.push_str(" │\n");
        }

        out.push_str("└");
        out.push_str(&"─".repeat(width - 2));
        out.push_str("┘");
        out
    }
}

pub struct Section<Message: 'static, B: Backend = IcedBackend> {
    title: String,
    content: Box<dyn View<Message, B>>,
    width: Length,
    height: Length,
}

impl<Message: 'static> Section<Message, IcedBackend> {
    pub fn new(
        title: impl Into<String>,
        content: impl View<Message, IcedBackend> + 'static,
    ) -> Self {
        Self::new_generic(title, content)
    }
}

impl<Message: 'static> Section<Message, TermBackend> {
    pub fn new_tui(
        title: impl Into<String>,
        content: impl View<Message, TermBackend> + 'static,
    ) -> Self {
        Self::new_generic(title, content)
    }
}

impl<Message: 'static, B: Backend> Section<Message, B> {
    pub fn new_generic(title: impl Into<String>, content: impl View<Message, B> + 'static) -> Self {
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

impl<Message: 'static> View<Message, IcedBackend> for Section<Message, IcedBackend> {
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

impl<Message: 'static> View<Message, TermBackend> for Section<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        format!(
            "\x1b[1;2m# {}\x1b[0m\n{}",
            self.title.to_uppercase(),
            self.content.view(context)
        )
    }
}
