use crate::core::{Context, View};
use iced::widget::{button, container, row, slider, svg, text, toggler};
use iced::{Alignment, Color, Element, Length, Renderer, Theme};
use peak_core::icons;
use std::sync::Arc;

pub struct Button<'a, Message> {
    label: String,
    icon: Option<String>,
    on_press: Option<Message>,
    style: ButtonStyle,
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
            style: ButtonStyle::default(),
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

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
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
        let style = self.style;

        let mut content = row![].spacing(8).align_y(Alignment::Center);

        if let Some(icon_name) = &self.icon {
            let icon_color = match style {
                ButtonStyle::Primary | ButtonStyle::Destructive => Color::WHITE,
                _ => theme.text,
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

        content = content.push(text(self.label.clone()).size(14).font(iced::Font {
            weight: if matches!(style, ButtonStyle::Primary) {
                iced::font::Weight::Bold
            } else {
                iced::font::Weight::Normal
            },
            ..Default::default()
        }));

        button(content)
            .on_press_maybe(self.on_press.clone())
            .padding([8, 16])
            .width(self.width)
            .style(move |_theme, status| {
                let base_color = match style {
                    ButtonStyle::Primary => theme.accent,
                    ButtonStyle::Secondary => theme.surface_bg,
                    ButtonStyle::Destructive => Color::from_rgb8(255, 59, 48),
                    ButtonStyle::Ghost => Color::TRANSPARENT,
                };

                let text_color = match style {
                    ButtonStyle::Primary | ButtonStyle::Destructive => Color::WHITE,
                    _ => theme.text,
                };

                match status {
                    button::Status::Active => button::Style {
                        background: Some(base_color.into()),
                        text_color,
                        border: iced::Border {
                            radius: 8.0.into(),
                            color: if matches!(style, ButtonStyle::Secondary) {
                                theme.divider
                            } else {
                                Color::TRANSPARENT
                            },
                            width: 1.0,
                        },
                        shadow: if matches!(style, ButtonStyle::Primary) {
                            iced::Shadow {
                                color: theme.shadow_color,
                                offset: iced::Vector::new(0.0, 1.0),
                                blur_radius: 2.0,
                            }
                        } else {
                            iced::Shadow::default()
                        },
                    },
                    button::Status::Hovered => button::Style {
                        background: Some(
                            base_color
                                .scale_alpha(if matches!(style, ButtonStyle::Ghost) {
                                    0.05
                                } else {
                                    0.9
                                })
                                .into(),
                        ),
                        text_color,
                        border: iced::Border {
                            radius: 8.0.into(),
                            color: theme.divider,
                            width: 1.0,
                        },
                        ..Default::default()
                    },
                    button::Status::Pressed => button::Style {
                        background: Some(base_color.scale_alpha(0.8).into()),
                        text_color,
                        border: iced::Border {
                            radius: 8.0.into(),
                            color: theme.divider,
                            width: 1.0,
                        },
                        ..Default::default()
                    },
                    button::Status::Disabled => button::Style {
                        background: Some(theme.surface_bg.scale_alpha(0.5).into()),
                        text_color: theme.text.scale_alpha(0.3),
                        ..Default::default()
                    },
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
                    .color(theme.text),
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
                        backgrounds: (theme.accent.into(), theme.surface_bg.into()),
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
                        border_color: theme.divider,
                    },
                },
            ),
        )
        .width(self.width)
        .padding([8, 0])
        .into()
    }
}
