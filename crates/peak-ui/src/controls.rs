use crate::core::{Context, View};
use crate::modifiers::{Intent, Size, Variant};
use iced::widget::{button, container, row, slider, svg, text, toggler};
use iced::{Alignment, Color, Element, Length, Renderer, Theme};
use peak_core::icons;
use std::sync::Arc;

pub struct Button<'a, Message> {
    label: String,
    icon: Option<String>,
    on_press: Option<Message>,
    intent: Intent,
    variant: Variant,
    size: Size,
    width: Length,
    _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Ghost,
}

impl<'a, Message> Button<'a, Message> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            on_press: None,
            intent: Intent::Primary,
            variant: Variant::Solid,
            size: Size::Medium,
            width: Length::Shrink,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        self.icon = Some(name.into());
        self
    }

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[deprecated(note = "Use .intent() and .variant() instead")]
    pub fn style(mut self, style: ButtonStyle) -> Self {
        match style {
            ButtonStyle::Primary => {
                self.intent = Intent::Primary;
                self.variant = Variant::Solid;
            }
            ButtonStyle::Secondary => {
                self.intent = Intent::Neutral;
                self.variant = Variant::Outline;
            }
            ButtonStyle::Destructive => {
                self.intent = Intent::Danger;
                self.variant = Variant::Solid;
            }
            ButtonStyle::Ghost => {
                self.intent = Intent::Neutral;
                self.variant = Variant::Ghost;
            }
        }
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

impl<'a, Message: Clone + 'static> View<Message> for Button<'a, Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let intent = self.intent;
        let variant = self.variant;
        let size = self.size;

        let mut content = row![].spacing(8).align_y(Alignment::Center);

        // Resolve semantic color
        let base_color = match intent {
            Intent::Primary => theme.colors.primary,
            Intent::Secondary => theme.colors.secondary,
            Intent::Success => theme.colors.success,
            Intent::Warning => theme.colors.warning,
            Intent::Danger => theme.colors.danger,
            Intent::Info => theme.colors.info,
            Intent::Neutral => theme.colors.text_primary, // Or specific neutral color
        };

        if let Some(icon_name) = &self.icon {
            let icon_color = match variant {
                Variant::Solid => theme.colors.on_primary, // Should depend on intent bg contrast
                _ => base_color,
            };

            let hex_color = format!(
                "#{:02X}{:02X}{:02X}",
                (icon_color.r * 255.0) as u8,
                (icon_color.g * 255.0) as u8,
                (icon_color.b * 255.0) as u8
            );

            content = content.push(
                svg(icons::get_ui_icon(icon_name, &hex_color))
                    .width(16)
                    .height(16),
            );
        }

        // Font size based on Size modifier
        let font_size = match size {
            Size::Small => 12.0,
            Size::Medium => 14.0,
            Size::Large => 16.0,
            Size::XLarge => 20.0,
        };

        // Padding based on Size modifier
        let padding = match size {
            Size::Small => [4, 8],
            Size::Medium => [8, 16],
            Size::Large => [12, 24],
            Size::XLarge => [16, 32],
        };

        content = content.push(text(self.label.clone()).size(font_size));

        button(content)
            .on_press_maybe(self.on_press.clone())
            .padding(padding)
            .width(self.width)
            .style(move |_theme, status| {
                let text_color = match variant {
                    Variant::Solid => theme.colors.on_primary, // Need robust verification
                    Variant::Soft => base_color,
                    Variant::Outline => base_color,
                    Variant::Ghost => base_color,
                };

                // Define backgrounds based on status and variant
                let background = match (variant, status) {
                    (Variant::Solid, button::Status::Active) => Some(base_color.into()),
                    (Variant::Solid, button::Status::Hovered) => {
                        Some(base_color.scale_alpha(0.9).into())
                    }
                    (Variant::Solid, button::Status::Pressed) => {
                        Some(base_color.scale_alpha(0.8).into())
                    }

                    (Variant::Soft, button::Status::Active) => {
                        Some(base_color.scale_alpha(0.15).into())
                    }
                    (Variant::Soft, button::Status::Hovered) => {
                        Some(base_color.scale_alpha(0.25).into())
                    }
                    (Variant::Soft, button::Status::Pressed) => {
                        Some(base_color.scale_alpha(0.1).into())
                    }

                    (Variant::Outline, button::Status::Hovered) => {
                        Some(base_color.scale_alpha(0.05).into())
                    }
                    (Variant::Outline, button::Status::Pressed) => {
                        Some(base_color.scale_alpha(0.1).into())
                    }

                    (Variant::Ghost, button::Status::Hovered) => {
                        Some(base_color.scale_alpha(0.05).into())
                    }
                    (Variant::Ghost, button::Status::Pressed) => {
                        Some(base_color.scale_alpha(0.1).into())
                    }

                    _ => None,
                };

                let border_color = if matches!(variant, Variant::Outline) {
                    base_color
                } else {
                    Color::TRANSPARENT
                };

                let shadow = if matches!(variant, Variant::Solid)
                    && matches!(status, button::Status::Active)
                {
                    iced::Shadow {
                        color: theme.shadow_color,
                        offset: iced::Vector::new(0.0, 1.0),
                        blur_radius: 2.0,
                    }
                } else {
                    iced::Shadow::default()
                };

                button::Style {
                    background,
                    text_color,
                    border: iced::Border {
                        radius: 8.0.into(),
                        color: border_color,
                        width: 1.0,
                    },
                    shadow,
                }
            })
            .into()
    }
}

pub struct Toggle<Message> {
    label: String,
    is_active: bool,
    on_toggle: Arc<dyn Fn(bool) -> Message + Send + Sync>,
}

impl<Message> Toggle<Message> {
    pub fn new(
        label: impl Into<String>,
        is_active: bool,
        f: impl Fn(bool) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            is_active,
            on_toggle: Arc::new(f),
        }
    }
}

impl<Message: 'static> View<Message> for Toggle<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let on_toggle = self.on_toggle.clone();

        container(
            row![
                text(self.label.clone())
                    .size(14)
                    .width(Length::Fill)
                    .color(theme.colors.text_primary),
                toggler(self.is_active).on_toggle(move |b| (on_toggle)(b))
            ]
            .spacing(12)
            .align_y(iced::Alignment::Center),
        )
        .into()
    }
}

pub struct Slider<Message> {
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: Arc<dyn Fn(f32) -> Message + Send + Sync>,
    width: Length,
}

impl<Message> Slider<Message> {
    pub fn new(
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        on_change: impl Fn(f32) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            range,
            value,
            on_change: Arc::new(on_change),
            width: Length::Shrink,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

impl<Message: Clone + 'static> View<Message> for Slider<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let on_change = self.on_change.clone();

        container(
            slider(self.range.clone(), self.value, move |v| (on_change)(v)).style(
                move |_theme, _status| slider::Style {
                    rail: slider::Rail {
                        backgrounds: (
                            theme.colors.primary.into(),
                            theme.colors.surface_variant.into(),
                        ),
                        width: 4.0,
                        border: iced::Border {
                            radius: 2.0.into(),
                            ..Default::default()
                        },
                    },
                    handle: slider::Handle {
                        shape: slider::HandleShape::Circle { radius: 10.0 },
                        background: Color::WHITE.into(),
                        border_width: 0.5,
                        border_color: theme.colors.divider,
                    },
                },
            ),
        )
        .width(self.width)
        .padding([8, 0])
        .into()
    }
}

pub struct Stepper<Message> {
    label: String,
    value: i32,
    range: std::ops::RangeInclusive<i32>,
    step: i32,
    on_change: Arc<dyn Fn(i32) -> Message + Send + Sync>,
}

impl<Message> Stepper<Message> {
    pub fn new(
        label: impl Into<String>,
        value: i32,
        on_change: impl Fn(i32) -> Message + Send + Sync + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            value,
            range: 0..=100,
            step: 1,
            on_change: Arc::new(on_change),
        }
    }

    pub fn range(mut self, range: std::ops::RangeInclusive<i32>) -> Self {
        self.range = range;
        self
    }

    pub fn step(mut self, step: i32) -> Self {
        self.step = step;
        self
    }
}

impl<Message: Clone + 'static> View<Message> for Stepper<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let on_change = self.on_change.clone();
        let value = self.value;
        let step = self.step;
        let range = self.range.clone();

        let decrease_disabled = value <= *range.start();
        let increase_disabled = value >= *range.end();

        let decrement = button(
            text("-")
                .size(16)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center),
        )
        .width(32)
        .height(32)
        .padding(0)
        .style(move |_theme, status| {
            let base_color = if decrease_disabled {
                theme.colors.text_secondary.scale_alpha(0.5)
            } else {
                theme.colors.primary
            };

            let bg = match status {
                _ if decrease_disabled => iced::Color::TRANSPARENT,
                button::Status::Active => theme.colors.surface_variant, // Using variant for btn bg
                button::Status::Hovered => theme.colors.surface_variant.scale_alpha(0.8),
                button::Status::Pressed => theme.colors.surface_variant.scale_alpha(0.6),
                _ => iced::Color::TRANSPARENT,
            };

            button::Style {
                background: Some(bg.into()),
                text_color: base_color,
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .on_press_maybe(if decrease_disabled {
            None
        } else {
            Some((on_change)(value - step))
        });

        let increment = button(
            text("+")
                .size(16)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center),
        )
        .width(32)
        .height(32)
        .padding(0)
        .style(move |_theme, status| {
            let base_color = if increase_disabled {
                theme.colors.text_secondary.scale_alpha(0.5)
            } else {
                theme.colors.primary
            };

            let bg = match status {
                _ if increase_disabled => iced::Color::TRANSPARENT,
                button::Status::Active => theme.colors.surface_variant,
                button::Status::Hovered => theme.colors.surface_variant.scale_alpha(0.8),
                button::Status::Pressed => theme.colors.surface_variant.scale_alpha(0.6),
                _ => iced::Color::TRANSPARENT,
            };

            button::Style {
                background: Some(bg.into()),
                text_color: base_color,
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .on_press_maybe(if increase_disabled {
            None
        } else {
            Some((on_change)(value + step))
        });

        container(
            row![
                text(self.label.clone())
                    .width(Length::Fill)
                    .color(theme.colors.text_primary),
                container(row![decrement, increment].spacing(1)).style(move |_| container::Style {
                    background: Some(theme.colors.surface_variant.into()), // Group background
                    border: iced::Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
            ]
            .align_y(iced::Alignment::Center),
        )
        .into()
    }
}
