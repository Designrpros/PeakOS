use iced::widget::{container, text, Column, Row};
use iced::{Alignment, Element, Length};
use peak_core::registry::AppId;
use peak_shell::dock::DockMessage;

pub fn view<'a>(
    apps: &'a [peak_core::models::MediaItem],
    tokens: peak_theme::ThemeTokens,
) -> Element<'a, DockMessage> {
    let mut grid = Column::new()
        .spacing(40)
        .padding(40)
        .align_x(Alignment::Center);

    // Determine icon color based on tokens
    let hex_color = format!(
        "#{:02x}{:02x}{:02x}",
        (tokens.colors.text_primary.r * 255.0) as u8,
        (tokens.colors.text_primary.g * 255.0) as u8,
        (tokens.colors.text_primary.b * 255.0) as u8
    );

    let chunk_size = 6;
    for chunk in apps.chunks(chunk_size) {
        let mut row = Row::new().spacing(40).align_y(Alignment::Center);

        for item in chunk {
            // Try to use app icon if it's a builtin app, otherwise fallback or generic
            let icon_element: Element<DockMessage> =
                if let Ok(app_id) = item.launch_command.parse::<AppId>() {
                    match peak_core::icons::IconResolver::resolve_app_icon(app_id, &hex_color) {
                        peak_core::icons::AppIcon::Svg(h) => iced::widget::svg(h)
                            .width(Length::Fixed(64.0))
                            .height(Length::Fixed(64.0))
                            .into(),
                        peak_core::icons::AppIcon::Image(h) => iced::widget::image(h)
                            .width(Length::Fixed(64.0))
                            .height(Length::Fixed(64.0))
                            .into(),
                    }
                } else {
                    iced::widget::svg(peak_core::icons::get_app_icon(AppId::AppGrid, &hex_color))
                        .width(Length::Fixed(64.0))
                        .height(Length::Fixed(64.0))
                        .into()
                };

            let name = &item.title;

            let btn = iced::widget::button(
                Column::new()
                    .spacing(8)
                    .align_x(Alignment::Center)
                    .push(container(icon_element).style(|_| container::Style {
                        ..Default::default()
                    }))
                    .push(
                        text(name)
                            .size(13)
                            .color(tokens.colors.text_primary) // Use tokens.colors.text_primary
                            .align_y(Alignment::Center),
                    ),
            )
            .on_press(DockMessage::LaunchMedia(item.clone()))
            .style(move |_, status| {
                if status == iced::widget::button::Status::Hovered {
                    let mut hover_bg = tokens.colors.text_primary;
                    hover_bg.a = 0.1;
                    iced::widget::button::Style {
                        background: Some(hover_bg.into()),
                        border: iced::Border {
                            radius: tokens.radius.into(),
                            width: 0.0,
                            color: iced::Color::TRANSPARENT,
                        },
                        ..Default::default()
                    }
                } else {
                    iced::widget::button::Style {
                        background: Some(iced::Color::TRANSPARENT.into()),
                        border: iced::Border {
                            radius: tokens.radius.into(),
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
            background: Some({
                let mut c = tokens.colors.surface;
                c.a = tokens.glass_opacity;
                c.into()
            }),
            ..Default::default()
        })
        .into()
}
