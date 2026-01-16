use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Color, Element, Length};
use peak_core::registry::{AppId, AppInfo};

pub fn view<'a, Message: 'a>(
    is_light: bool,
    _running_apps: &[AppId],
    current_app: Option<AppId>,
) -> Element<'a, Message, iced::Theme, iced::Renderer> {
    let bg_color = if is_light {
        Color::from_rgb8(240, 240, 240)
    } else {
        Color::from_rgb8(20, 20, 20)
    };

    let text_color = if is_light { Color::BLACK } else { Color::WHITE };

    let content: Element<'a, Message, iced::Theme, iced::Renderer> =
        if let Some(app_id) = current_app {
            // App is open - show full screen app with navigation bar
            let info = AppInfo::get_info(app_id);
            column![
                container::<Message, iced::Theme, iced::Renderer>(
                    text(info.name).size(20).color(text_color)
                )
                .width(Length::Fill)
                .height(Length::Fixed(50.0))
                .center_x(Length::Fill)
                .center_y(Length::Fill),
                container::<Message, iced::Theme, iced::Renderer>(
                    text("App Content Goes Here").size(16).color(text_color)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            ]
            .into()
        } else {
            // App Grid (Home Screen)
            let mut grid = column![].spacing(20).align_x(Alignment::Center);

            // Mock grid of icons
            let apps = vec![
                AppId::Terminal,
                AppId::Browser,
                AppId::Library,
                AppId::Settings,
                AppId::FileManager,
                AppId::Store,
            ];

            for chunk in apps.chunks(3) {
                let mut r = row![].spacing(30);
                for &id in chunk {
                    let info = AppInfo::get_info(id);
                    r = r.push(
                        column![
                            container::<Message, iced::Theme, iced::Renderer>(text("📱").size(40)) // Placeholder for real icons
                                .width(Length::Fixed(60.0))
                                .height(Length::Fixed(60.0))
                                .center_x(Length::Fill)
                                .center_y(Length::Fill)
                                .style(move |_: &iced::Theme| container::Style {
                                    background: Some(Background::Color(if is_light {
                                        Color::from_rgb8(220, 220, 220)
                                    } else {
                                        Color::from_rgb8(40, 40, 40)
                                    })),
                                    border: iced::Border {
                                        radius: 15.0.into(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }),
                            text(info.name).size(12).color(text_color)
                        ]
                        .align_x(Alignment::Center)
                        .spacing(5),
                    );
                }
                grid = grid.push(r);
            }

            container(grid)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_: &iced::Theme| container::Style {
            background: Some(Background::Color(bg_color)),
            ..Default::default()
        })
        .into()
}
