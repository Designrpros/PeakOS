use crate::core::{Backend, Context, IcedBackend, View};
use crate::modifiers::Intent;
use iced::{Alignment, Color, Length};
use std::marker::PhantomData;

pub struct Text<B: Backend = IcedBackend> {
    content: String,
    size: f32,
    color: Option<Color>,
    intent: Option<Intent>,
    is_bold: bool,
    is_dim: bool,
    alignment: Alignment,
    font: iced::Font,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Text<B> {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            size: 14.0,
            color: None,
            intent: None,
            is_bold: false,
            is_dim: false,
            alignment: Alignment::Start,
            font: iced::Font::default(),
            _phantom: PhantomData,
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

    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = Some(intent);
        self
    }

    pub fn bold(mut self) -> Self {
        self.is_bold = true;
        self
    }

    pub fn dim(mut self) -> Self {
        self.is_dim = true;
        self
    }

    pub fn center(mut self) -> Self {
        self.alignment = Alignment::Center;
        self
    }

    pub fn large_title(mut self) -> Self {
        self.size = 32.0;
        self.is_bold = true;
        self
    }

    pub fn title1(mut self) -> Self {
        self.size = 28.0;
        self.is_bold = true;
        self
    }

    pub fn title2(mut self) -> Self {
        self.size = 22.0;
        self.is_bold = true;
        self
    }

    pub fn title3(mut self) -> Self {
        self.size = 20.0;
        self.is_bold = true;
        self
    }

    pub fn headline(mut self) -> Self {
        self.size = 17.0;
        self.is_bold = true;
        self
    }

    pub fn body(mut self) -> Self {
        self.size = 17.0;
        self
    }

    pub fn callout(mut self) -> Self {
        self.size = 16.0;
        self
    }

    pub fn subheadline(mut self) -> Self {
        self.size = 15.0;
        self.is_dim = true;
        self
    }

    pub fn footnote(mut self) -> Self {
        self.size = 13.0;
        self.is_dim = true;
        self
    }

    pub fn caption1(mut self) -> Self {
        self.size = 12.0;
        self.is_dim = true;
        self
    }

    pub fn caption2(mut self) -> Self {
        self.size = 11.0;
        self.is_dim = true;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.is_dim = true;
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Text<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::text(
            self.content.clone(),
            self.size,
            self.color,
            self.is_bold,
            self.is_dim,
            self.intent,
            Some(self.font),
            self.alignment,
            context,
        )
    }
}

pub struct Rectangle<B: Backend = IcedBackend> {
    width: Length,
    height: Length,
    color: Option<Color>,
    radius: f32,
    border_width: f32,
    border_color: Option<Color>,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Rectangle<B> {
    pub fn new(width: Length, height: Length) -> Self {
        Self {
            width,
            height,
            color: None,
            radius: 0.0,
            border_width: 0.0,
            border_color: None,
            _phantom: PhantomData,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn border(mut self, width: f32, color: Color) -> Self {
        self.border_width = width;
        self.border_color = Some(color);
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Rectangle<B> {
    fn view(&self, _context: &Context) -> B::AnyView<Message> {
        B::rectangle(
            self.width,
            self.height,
            self.color,
            self.radius,
            self.border_width,
            self.border_color,
        )
    }
}

pub struct Space<B: Backend = IcedBackend> {
    width: Length,
    height: Length,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Space<B> {
    pub fn new(width: Length, height: Length) -> Self {
        Self {
            width,
            height,
            _phantom: PhantomData,
        }
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Space<B> {
    fn view(&self, _context: &Context) -> B::AnyView<Message> {
        B::space(self.width, self.height)
    }
}

pub struct Divider<B: Backend = IcedBackend> {
    _phantom: PhantomData<B>,
}

impl<B: Backend> Divider<B> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Divider<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::divider(context)
    }
}

pub struct Icon<B: Backend = IcedBackend> {
    name: String,
    size: f32,
    color: Option<Color>,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Icon<B> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size: 24.0,
            color: None,
            _phantom: PhantomData,
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
}

impl<Message: 'static, B: Backend> View<Message, B> for Icon<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        B::icon(self.name.clone(), self.size, self.color, context)
    }
}

pub struct Image<B: Backend = IcedBackend> {
    path: std::path::PathBuf,
    width: Length,
    height: Length,
    radius: f32,
    _phantom: PhantomData<B>,
}

impl<B: Backend> Image<B> {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            path: path.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            radius: 0.0,
            _phantom: PhantomData,
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

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Image<B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        // Image is complex, stub for TUI
        B::text(
            format!("[IMG: {:?}]", self.path),
            12.0,
            None,
            false,
            true,
            None,
            None,
            Alignment::Center,
            context,
        )
    }
}
