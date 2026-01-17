use crate::components::dock::DockMessage;
use iced::widget::{container, text, Column, Row};
use iced::{Alignment, Element, Length};
use peak_core::registry::{AppId, AppInfo};

pub fn view<'a>(apps: &[AppId], is_light: bool) -> Element<'a, DockMessage> {
    let mut grid = Column::new()
        .spacing(40)
        .padding(40)
        .align_x(Alignment::Center);

    // Determine icon color based on theme
    let icon_color = if is_light { "#000000" } else { "#FFFFFF" };

    let chunk_size = 6;
    for chunk in apps.chunks(chunk_size) {
        let mut row = Row::new().spacing(40).align_y(Alignment::Center);

        for app_id in chunk {
            // Use the centralized icon loader for correct coloring
            let icon_handle = peak_core::icons::get_app_icon(*app_id, icon_color);
            let name = AppInfo::get_info(*app_id).name;

            let btn = iced::widget::button(
                Column::new()
                    .spacing(8)
                    .align_x(Alignment::Center)
                    .push(
                        container(
                            iced::widget::svg(icon_handle)
                                .width(Length::Fixed(64.0))
                                .height(Length::Fixed(64.0)),
                        )
                        .style(|_| container::Style {
                            ..Default::default()
                        }),
                    )
                    .push(
                        text(name)
                            .size(13)
                            .color(if is_light {
                                iced::Color::BLACK
                            } else {
                                iced::Color::WHITE
                            })
                            .align_y(Alignment::Center),
                    ),
            )
            .on_press(DockMessage::Launch(*app_id))
            .style(move |_, status| {
                let parse_color = |r, g, b, a| iced::Color { r, g, b, a };
                if status == iced::widget::button::Status::Hovered {
                    iced::widget::button::Style {
                        background: Some(
                            if is_light {
                                parse_color(0.0, 0.0, 0.0, 0.1)
                            } else {
                                parse_color(1.0, 1.0, 1.0, 0.1)
                            }
                            .into(),
                        ),
                        border: iced::Border {
                            radius: 12.0.into(),
                            width: 0.0,
                            color: iced::Color::TRANSPARENT,
                        },
                        ..Default::default()
                    }
                } else {
                    iced::widget::button::Style {
                        background: Some(iced::Color::TRANSPARENT.into()),
                        border: iced::Border {
                            radius: 12.0.into(),
                            width: 0.0,
                            color: iced::Color::TRANSPARENT,
                        },
                        ..Default::default()
                    }
                }
            })
            .padding(16);

            row = row.push(btn);
        }
        grid = grid.push(row);
    }

    container(grid)
        .padding(40)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_| container::Style {
            background: Some(
                if is_light {
                    iced::Color::from_rgba(0.9, 0.9, 0.9, 0.3)
                } else {
                    iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3)
                }
                .into(),
            ),
            ..Default::default()
        })
        .into()
}
