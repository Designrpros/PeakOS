use crate::core::{Context, View};
use crate::modifiers::{ControlSize, Intent, Variant};
use iced::widget::{button, container, row, slider, svg, text, toggler};
use iced::{Alignment, Color, Element, Length, Renderer, Theme};
use peak_core::icons;
use std::sync::Arc;

pub struct Button<Message> {
    content: Box<dyn View<Message>>,
    icon: Option<String>,
    on_press: Option<Message>,
    intent: Intent,
    variant: Variant,
    size: ControlSize,
    width: Length,
}

#[derive(Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Ghost,
}

impl<Message: 'static> Button<Message> {
    pub fn new(content: impl View<Message> + 'static) -> Self {
        Self {
            content: Box::new(content),
            icon: None,
            on_press: None,
            intent: Intent::Primary,
            variant: Variant::Solid,
            size: ControlSize::Medium,
            width: Length::Shrink,
        }
    }

    pub fn label(text: impl Into<String>) -> Self {
        Self::new(crate::atoms::Text::new(text))
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        self.icon = Some(name.into());
        self
    }

    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    pub fn on_press_maybe(mut self, msg: Option<Message>) -> Self {
        self.on_press = msg;
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

    pub fn size(mut self, size: ControlSize) -> Self {
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

impl<Message: Clone + 'static> View<Message> for Button<Message> {
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

        let _font_size = match size {
            ControlSize::Small => 12.0,
            ControlSize::Medium => 14.0,
            ControlSize::Large => 16.0,
            ControlSize::XLarge => 20.0,
        };

        // Padding based on ControlSize modifier
        let padding = match size {
            ControlSize::Small => [4, 8],
            ControlSize::Medium => [8, 16],
            ControlSize::Large => [12, 24],
            ControlSize::XLarge => [16, 32],
        };

        content = content.push(self.content.view(context));

        let radius = context.radius(8.0);
        button(content)
            .on_press_maybe(self.on_press.clone())
            .padding(padding)
            .width(self.width)
            .style({
                let r = radius;
                let v = variant;
                let b_col = base_color;
                let o_pri = theme.colors.on_primary;
                let s_col = theme.shadow_color;

                move |_theme, status| {
                    let text_color = match v {
                        Variant::Solid => o_pri,
                        Variant::Soft => b_col,
                        Variant::Outline => b_col,
                        Variant::Ghost => b_col,
                    };

                    let background = match (v, status) {
                        (Variant::Solid, button::Status::Active) => Some(b_col.into()),
                        (Variant::Solid, button::Status::Hovered) => {
                            Some(b_col.scale_alpha(0.9).into())
                        }
                        (Variant::Solid, button::Status::Pressed) => {
                            Some(b_col.scale_alpha(0.8).into())
                        }

                        (Variant::Soft, button::Status::Active) => {
                            Some(b_col.scale_alpha(0.15).into())
                        }
                        (Variant::Soft, button::Status::Hovered) => {
                            Some(b_col.scale_alpha(0.25).into())
                        }
                        (Variant::Soft, button::Status::Pressed) => {
                            Some(b_col.scale_alpha(0.1).into())
                        }

                        (Variant::Outline, button::Status::Hovered) => {
                            Some(b_col.scale_alpha(0.05).into())
                        }
                        (Variant::Outline, button::Status::Pressed) => {
                            Some(b_col.scale_alpha(0.1).into())
                        }

                        (Variant::Ghost, button::Status::Hovered) => {
                            Some(b_col.scale_alpha(0.05).into())
                        }
                        (Variant::Ghost, button::Status::Pressed) => {
                            Some(b_col.scale_alpha(0.1).into())
                        }

                        _ => None,
                    };

                    let border_color = if matches!(v, Variant::Outline) {
                        b_col
                    } else {
                        Color::TRANSPARENT
                    };

                    let shadow = if !cfg!(target_arch = "wasm32")
                        && matches!(v, Variant::Solid)
                        && matches!(status, button::Status::Active)
                    {
                        iced::Shadow {
                            color: s_col,
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
                            radius: r,
                            color: border_color,
                            width: 1.0,
                        },
                        shadow,
                    }
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

        let rail_radius = context.radius(2.0);
        container(
            slider(self.range.clone(), self.value, move |v| (on_change)(v)).style({
                let r = rail_radius;
                let p_col = theme.colors.primary;
                let v_bg = theme.colors.surface_variant;
                let div_col = theme.colors.divider;
                move |_theme, _status| slider::Style {
                    rail: slider::Rail {
                        backgrounds: (p_col.into(), v_bg.into()),
                        width: 4.0,
                        border: iced::Border {
                            radius: r,
                            ..Default::default()
                        },
                    },
                    handle: slider::Handle {
                        shape: if cfg!(target_arch = "wasm32") {
                            slider::HandleShape::Rectangle {
                                width: 20,
                                border_radius: 0.0.into(),
                            }
                        } else {
                            slider::HandleShape::Circle { radius: 10.0 }
                        },
                        background: Color::WHITE.into(),
                        border_width: 0.5,
                        border_color: div_col,
                    },
                }
            }),
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

        let radius = context.radius(8.0);
        let primary_color = theme.colors.primary;
        let variant_bg_color = theme.colors.surface_variant;
        let text_secondary_color = theme.colors.text_secondary;

        let decrement = button(
            text("-")
                .size(16)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center),
        )
        .width(32)
        .height(32)
        .padding(0)
        .style({
            let r = radius;
            let p_col = primary_color;
            let v_bg = variant_bg_color;
            let t_sec = text_secondary_color;
            let disabled = decrease_disabled;
            move |_theme, status| {
                let base_color = if disabled {
                    t_sec.scale_alpha(0.5)
                } else {
                    p_col
                };

                let bg = match status {
                    _ if disabled => iced::Color::TRANSPARENT,
                    button::Status::Active => v_bg,
                    button::Status::Hovered => v_bg.scale_alpha(0.8),
                    button::Status::Pressed => v_bg.scale_alpha(0.6),
                    _ => iced::Color::TRANSPARENT,
                };

                button::Style {
                    background: Some(bg.into()),
                    text_color: base_color,
                    border: iced::Border {
                        radius: r,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }
        })
        .on_press_maybe(if decrease_disabled {
            None
        } else {
            Some((on_change)(value - step))
        });

        let radius = context.radius(8.0);
        let increment = button(
            text("+")
                .size(16)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center),
        )
        .width(32)
        .height(32)
        .padding(0)
        .style({
            let r = radius;
            let p_col = primary_color;
            let v_bg = variant_bg_color;
            let t_sec = text_secondary_color;
            let disabled = increase_disabled;
            move |_theme, status| {
                let base_color = if disabled {
                    t_sec.scale_alpha(0.5)
                } else {
                    p_col
                };

                let bg = match status {
                    _ if disabled => iced::Color::TRANSPARENT,
                    button::Status::Active => v_bg,
                    button::Status::Hovered => v_bg.scale_alpha(0.8),
                    button::Status::Pressed => v_bg.scale_alpha(0.6),
                    _ => iced::Color::TRANSPARENT,
                };

                button::Style {
                    background: Some(bg.into()),
                    text_color: base_color,
                    border: iced::Border {
                        radius: r,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }
        })
        .on_press_maybe(if increase_disabled {
            None
        } else {
            Some((on_change)(value + step))
        });

        let group_radius = context.radius(8.0);
        let group_bg_color = theme.colors.surface_variant;
        container(
            row![
                text(self.label.clone())
                    .width(Length::Fill)
                    .color(theme.colors.text_primary),
                container(row![decrement, increment].spacing(1)).style({
                    let r = group_radius;
                    let g_bg = group_bg_color;
                    move |_| container::Style {
                        background: Some(g_bg.into()), // Group background
                        border: iced::Border {
                            radius: r,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
            ]
            .align_y(iced::Alignment::Center),
        )
        .into()
    }
}
