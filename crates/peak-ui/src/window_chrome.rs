use iced::widget::{button, container, row, text};
use iced::{Alignment, Color, Element, Length, Padding};

pub fn view<'a, Message>(
    title: &str,
    content: Element<'a, Message>,
    on_close: Message,
    on_maximize: Option<Message>,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    let title_bar = container(
        row![
            row![
                // Red (Close)
                Element::from(
                    button(window_control(Color::from_rgb8(255, 69, 58))) // MacOS Red
                        .on_press(on_close)
                        .padding(0)
                        .style(|_theme, _status| button::Style {
                            background: None,
                            ..Default::default()
                        })
                ),
                // Yellow (Minimize - Placeholder)
                window_control(Color::from_rgb8(255, 186, 10)), // MacOS Yellow
                // Green (Maximize)
                if let Some(msg) = on_maximize {
                    Element::from(
                        button(window_control(Color::from_rgb8(50, 215, 75))) // MacOS Green
                            .on_press(msg)
                            .padding(0)
                            .style(|_theme, _status| button::Style {
                                background: None,
                                ..Default::default()
                            }),
                    )
                } else {
                    Element::from(
                        container(window_control(Color::from_rgb8(50, 215, 75))).padding(0),
                    )
                }
            ]
            .spacing(10)
            .padding(Padding {
                left: 4.0,
                ..Padding::ZERO
            }),
            iced::widget::horizontal_space(),
            text(title.to_uppercase())
                .size(11)
                .font(iced::Font::DEFAULT)
                .color(Color::from_rgb(0.4, 0.4, 0.4)),
            iced::widget::horizontal_space(),
        ]
        .width(Length::Fill)
        .height(Length::Fixed(40.0))
        .align_y(Alignment::Center),
    )
    .padding(Padding {
        left: 16.0,
        right: 16.0,
        ..Padding::ZERO
    })
    .style(|_| container::Style::default());
    let window_body = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(0);

    container(iced::widget::column![title_bar, window_body])
        .style(|theme: &iced::Theme| {
            // Using a simple check for dark mode since we don't have the full custom theme mapping yet
            let is_dark = theme == &iced::Theme::Dark;
            let bg = if is_dark {
                Color::from_rgb8(22, 21, 21)
            } else {
                Color::WHITE
            };
            let border_color = if is_dark {
                Color::from_rgba(1.0, 1.0, 1.0, 0.1)
            } else {
                Color::from_rgba(0.0, 0.0, 0.0, 0.08)
            };

            container::Style {
                background: Some(bg.into()),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 12.0,
                },
                ..Default::default()
            }
        })
        .into()
}

fn window_control<'a, Message>(color: Color) -> Element<'a, Message>
where
    Message: 'a,
{
    container(iced::widget::text(""))
        .width(10)
        .height(10)
        .style(move |_| container::Style {
            background: Some(color.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        })
        .into()
}
