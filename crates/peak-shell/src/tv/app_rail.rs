// TV App Rail - Apple TV-style large app icons
// Horizontally scrollable row of large, colorful app tiles

use iced::widget::{button, container, row, svg};
use iced::{Alignment, Element, Length};
use peak_core::registry::AppId;
use peak_theme::ThemeTokens;

#[derive(Debug, Clone)]
pub enum AppRailMessage {
    SelectApp(AppId),
    LaunchApp(AppId),
}

pub fn view<'a>(
    apps: &[AppId],
    selected_index: usize,
    _tokens: ThemeTokens,
) -> Element<'a, AppRailMessage> {
    let hex_color = "#FFFFFF"; // Always white on TV for visibility

    let mut app_row = row![].spacing(12).align_y(Alignment::Center);

    for (i, &app_id) in apps.iter().take(8).enumerate() {
        let is_selected = i == selected_index;
        let size = if is_selected { 110.0 } else { 100.0 };

        // Get app color for background
        let bg_color = get_app_color(app_id);

        let icon: Element<AppRailMessage> =
            match peak_core::icons::IconResolver::resolve_app_icon(app_id, hex_color) {
                peak_core::icons::AppIcon::Svg(handle) => svg(handle)
                    .width(Length::Fixed(48.0))
                    .height(Length::Fixed(48.0))
                    .into(),
                peak_core::icons::AppIcon::Image(handle) => iced::widget::image(handle)
                    .width(Length::Fixed(48.0))
                    .height(Length::Fixed(48.0))
                    .into(),
            };

        let app_tile = button(
            container(icon)
                .width(Length::Fixed(size))
                .height(Length::Fixed(size))
                .center_x(size)
                .center_y(size)
                .style(move |_| container::Style {
                    background: Some(bg_color.into()),
                    border: iced::Border {
                        radius: 20.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
        )
        .on_press(AppRailMessage::LaunchApp(app_id))
        .padding(4)
        .style(move |_, _| {
            let shadow = if is_selected {
                iced::Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                    offset: iced::Vector::new(0.0, 8.0),
                    blur_radius: 20.0,
                }
            } else {
                iced::Shadow::default()
            };

            iced::widget::button::Style {
                background: None,
                border: iced::Border::default(),
                shadow,
                ..Default::default()
            }
        });

        app_row = app_row.push(app_tile);
    }

    container(app_row)
        .padding([20, 60])
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3).into()),
            border: iced::Border {
                radius: 30.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

fn get_app_color(app_id: AppId) -> iced::Color {
    match app_id {
        AppId::Terminal => iced::Color::from_rgb8(0, 0, 0), // Black
        AppId::Browser => iced::Color::from_rgb8(52, 199, 89), // Green
        AppId::Library => iced::Color::from_rgb8(255, 45, 85), // Red
        AppId::Turntable => iced::Color::from_rgb8(255, 55, 95), // Pink
        AppId::Store => iced::Color::from_rgb8(0, 122, 255), // Blue
        AppId::FileManager => iced::Color::from_rgb8(88, 86, 214), // Purple
        AppId::Settings => iced::Color::from_rgb8(142, 142, 147), // Gray
        AppId::Editor => iced::Color::from_rgb8(255, 159, 10), // Orange
        _ => iced::Color::from_rgb8(100, 100, 100),         // Default gray
    }
}
