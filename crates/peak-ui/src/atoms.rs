use crate::core::{Context, View};
use iced::widget::text;
use iced::{Color, Element, Renderer, Theme};

pub struct Text {
    content: String,
    size: f32,
    color: Option<Color>,
    font: iced::Font,
    is_secondary: bool,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            size: 14.0,  // Default body text
            color: None, // Will use theme default if None
            font: Default::default(),
            is_secondary: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn font(mut self, font: iced::Font) -> Self {
        self.font = font;
        self
    }

    // Semantic modifiers could go here
    pub fn large_title(mut self) -> Self {
        self.size = 32.0;
        self.font.weight = iced::font::Weight::Bold;
        self
    }

    pub fn title(mut self) -> Self {
        self.size = 24.0;
        self.font.weight = iced::font::Weight::Bold;
        self
    }

    pub fn body(mut self) -> Self {
        self.size = 14.0;
        self.font.weight = iced::font::Weight::Normal;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.size = 13.0;
        self.is_secondary = true;
        self
    }
}

impl<Message> View<Message> for Text {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let color = self.color.unwrap_or_else(|| {
            if self.is_secondary {
                context.theme.secondary_text
            } else {
                context.theme.text
            }
        });

        text(self.content.clone())
            .size(self.size)
            .color(color)
            .font(self.font)
            .into()
    }
}
