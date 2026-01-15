use crate::app::Message;
use iced::widget::{button, column, container, text};
use iced::Element;

pub struct SystemAlert;

impl SystemAlert {
    pub fn view<'a>(
        title: &'a str,
        body: &'a str,
        on_close: Message,
        is_light: bool,
    ) -> Element<'a, Message> {
        let (bg_color, text_color, btn_bg, btn_text) = if is_light {
            (
                iced::Color::from_rgb8(235, 235, 235), // Plain menu bar grey for Light
                iced::Color::BLACK,
                iced::Color::BLACK, // Opposite for button
                iced::Color::WHITE,
            )
        } else {
            (
                iced::Color::from_rgb8(40, 40, 40), // Plain menu bar grey for Dark
                iced::Color::WHITE,
                iced::Color::WHITE, // Opposite for button
                iced::Color::BLACK,
            )
        };

        let content = column![
            text(title).size(24).color(text_color),
            text(body).size(16).color(text_color),
            button(
                text("OK")
                    .color(btn_text)
                    .align_x(iced::alignment::Horizontal::Center)
            )
            .on_press(on_close)
            .padding([8, 24])
            .style(move |_, status| {
                let opacity = if status == iced::widget::button::Status::Hovered {
                    0.8
                } else {
                    1.0
                };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(iced::Color {
                        a: opacity,
                        ..btn_bg
                    })),
                    text_color: btn_text,
                    border: iced::Border {
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
        ]
        .spacing(20)
        .align_x(iced::Alignment::Center);

        container(content)
            .width(400)
            .padding(30)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(bg_color)),
                border: iced::Border {
                    color: if is_light {
                        iced::Color::from_rgba8(0, 0, 0, 0.1)
                    } else {
                        iced::Color::from_rgba8(255, 255, 255, 0.1)
                    },
                    width: 1.0,
                    radius: 12.0.into(),
                },
                text_color: Some(text_color),
                shadow: iced::Shadow {
                    color: iced::Color::BLACK,
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
            })
            .into()
    }
}
