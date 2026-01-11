use iced::widget::{button, container, row, text};
use iced::{Alignment, Color, Element, Length, Padding};

pub fn view<'a, Message>(
    title: &str,
    content: Element<'a, Message>,
    on_close: Message,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    let title_bar = container(
        row![
            text(title.to_uppercase())
                .size(10) // Ultra compact
                .font(iced::Font::MONOSPACE)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            iced::widget::horizontal_space(),
            row![
                window_control(Color::from_rgb(0.9, 0.9, 0.2)), // Min
                window_control(Color::from_rgb(0.2, 0.9, 0.2)), // Max
                button(window_control(Color::from_rgb(0.9, 0.2, 0.2)))
                    .on_press(on_close)
                    .padding(0)
                    .style(|_theme, status| button::Style {
                        background: if status == iced::widget::button::Status::Hovered {
                            Some(Color::from_rgba(0.9, 0.2, 0.2, 0.2).into())
                        } else {
                            None
                        },
                        border: iced::Border {
                            radius: 10.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
            ]
            .spacing(6)
        ]
        .width(Length::Fill)
        .align_y(Alignment::Center),
    )
    .padding(Padding::from([6, 12]))
    .style(|_| container::Style::default()); // Fluid: No background on title bar

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
