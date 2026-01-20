use crate::core::{Context, View};
use crate::modifiers::Intent;
use iced::widget::{container, text};
use iced::{Color, Element, Length, Renderer, Theme};

pub struct Text {
    content: String,
    size: f32,
    color: Option<Color>,
    font: iced::Font,
    intent: Option<Intent>,
    is_secondary: bool,
    centered: bool,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            size: 14.0,  // Default body text
            color: None, // Will use theme default if None
            font: Default::default(),
            intent: None,
            is_secondary: false,
            centered: false,
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

    pub fn center(mut self) -> Self {
        self.centered = true;
        self
    }

    // Semantic modifiers could go here
    pub fn large_title(mut self) -> Self {
        self.size = 32.0;
        self.font.weight = iced::font::Weight::Bold;
        self
    }

    pub fn title1(mut self) -> Self {
        self.size = 28.0;
        self.font.weight = iced::font::Weight::Bold;
        self
    }

    pub fn title2(mut self) -> Self {
        self.size = 22.0;
        self.font.weight = iced::font::Weight::Bold;
        self
    }

    pub fn title3(mut self) -> Self {
        self.size = 20.0;
        self.font.weight = iced::font::Weight::Semibold;
        self
    }

    pub fn headline(mut self) -> Self {
        self.size = 17.0;
        self.font.weight = iced::font::Weight::Semibold;
        self
    }

    pub fn body(mut self) -> Self {
        self.size = 17.0; // Apple Human Interface Guidelines default body
        self.font.weight = iced::font::Weight::Normal;
        self
    }

    pub fn callout(mut self) -> Self {
        self.size = 16.0;
        self.font.weight = iced::font::Weight::Normal;
        self
    }

    pub fn subheadline(mut self) -> Self {
        self.size = 15.0;
        self.font.weight = iced::font::Weight::Normal;
        self
    }

    pub fn footnote(mut self) -> Self {
        self.size = 13.0;
        self.font.weight = iced::font::Weight::Normal;
        self
    }

    pub fn caption1(mut self) -> Self {
        self.size = 12.0;
        self.font.weight = iced::font::Weight::Normal;
        self
    }

    pub fn caption2(mut self) -> Self {
        self.size = 11.0;
        self.font.weight = iced::font::Weight::Normal;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.is_secondary = true;
        self
    }
}

impl<Message: 'static> View<Message> for Text {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let color = self.color.unwrap_or_else(|| {
            if let Some(intent) = self.intent {
                match intent {
                    Intent::Primary => context.theme.colors.primary,
                    Intent::Secondary => context.theme.colors.secondary,
                    Intent::Success => context.theme.colors.success,
                    Intent::Warning => context.theme.colors.warning,
                    Intent::Danger => context.theme.colors.danger,
                    Intent::Info => context.theme.colors.info,
                    Intent::Neutral => context.theme.colors.text_primary,
                }
            } else if self.is_secondary {
                context.theme.colors.text_secondary
            } else {
                context.theme.colors.text_primary
            }
        });

        let t = text(self.content.clone())
            .size(self.size)
            .color(color)
            .font(self.font);

        if self.centered {
            container(t)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .into()
        } else {
            t.into()
        }
    }
}

pub struct Rectangle {
    width: Length,
    height: Length,
    color: Option<Color>,
    radius: f32,
    border_width: f32,
    border_color: Option<Color>,
}

impl Rectangle {
    pub fn new(width: Length, height: Length) -> Self {
        Self {
            width,
            height,
            color: None,
            radius: 0.0,
            border_width: 0.0,
            border_color: None,
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

impl<Message: 'static> View<Message> for Rectangle {
    fn view(&self, _context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let color = self.color;
        let border_color = self.border_color;
        let border_width = self.border_width;
        let radius = self.radius;

        container(iced::widget::Space::new(Length::Fill, Length::Fill))
            .width(self.width)
            .height(self.height)
            .style(move |_| container::Style {
                background: color.map(iced::Background::Color),
                border: iced::Border {
                    color: border_color.unwrap_or(Color::TRANSPARENT),
                    width: border_width,
                    radius: radius.into(),
                },
                ..Default::default()
            })
            .into()
    }
}

pub struct Space {
    width: Length,
    height: Length,
}

impl Space {
    pub fn new(width: Length, height: Length) -> Self {
        Self { width, height }
    }
}

impl<Message: 'static> View<Message> for Space {
    fn view(&self, _context: &Context) -> Element<'static, Message, Theme, Renderer> {
        iced::widget::Space::new(self.width, self.height).into()
    }
}

pub struct Divider;

impl Divider {
    pub fn new() -> Self {
        Self
    }
}

impl<Message: 'static> View<Message> for Divider {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let divider_color = context.theme.colors.divider;
        container(iced::widget::Rule::horizontal(1))
            .style(move |_| container::Style {
                // Use border color or divider color from theme
                text_color: Some(divider_color),
                ..Default::default()
            })
            .into()
    }
}

pub struct Icon {
    name: String,
    size: f32,
    color: Option<Color>,
}

impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            size: 24.0,
            color: None,
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

impl<Message: 'static> View<Message> for Icon {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let color = self.color.unwrap_or(theme.colors.text_primary);

        let hex_color = format!(
            "#{:02X}{:02X}{:02X}",
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8
        );

        let handle = peak_core::icons::get_ui_icon(&self.name, &hex_color);

        iced::widget::svg(handle)
            .width(self.size)
            .height(self.size)
            .into()
    }
}

pub struct Image {
    path: std::path::PathBuf,
    width: Length,
    height: Length,
    radius: f32,
}

impl Image {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            path: path.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            radius: 0.0,
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

impl<Message: 'static> View<Message> for Image {
    fn view(&self, _context: &Context) -> Element<'static, Message, Theme, Renderer> {
        // In a real app we'd handle handle loading/caching.
        // Here we assume local path or bundled asset.
        let handle = iced::widget::image::Handle::from_path(&self.path);
        let radius = self.radius;

        // Image widget doesn't support border radius natively easily without being in a container that clips?
        // Iced Image widget is simple. We can use container with border radius + overflow hidden if supported?
        // Current Iced container clipping is limited.
        // For now, we render just the image.

        let img = iced::widget::image(handle)
            .width(self.width)
            .height(self.height)
            .content_fit(iced::ContentFit::Cover);

        if radius > 0.0 {
            container(img)
                .style(move |_| container::Style {
                    border: iced::Border {
                        radius: radius.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into()
        } else {
            img.into()
        }
    }
}
