use iced::widget::{
    button, column, container, horizontal_space, image, row, scrollable, stack, svg, text,
    text_input, vertical_space,
};
use iced::{Alignment, Background, Color, Element, Length, Shadow, Vector};
use peak_core::registry::{AppId, AppInfo};

pub fn view<'a, Message: 'a + Clone>(
    is_light: bool,
    _running_apps: &[AppId],
    current_app: Option<AppId>,
    time: &'a str,
    terminal_app: &'a peak_core::apps::terminal::TerminalApp,
    settings_app: &'a peak_core::apps::settings::SettingsApp,
    on_open: impl Fn(AppId) -> Message + 'a,
    on_home: Message,
    on_terminal: impl Fn(peak_core::apps::terminal::TerminalMessage) -> Message + 'a + Clone,
    _on_settings: impl Fn(peak_core::apps::settings::SettingsMessage) -> Message + 'a + Clone,
) -> Element<'a, Message, iced::Theme, iced::Renderer> {
    let wallpaper_path = peak_core::utils::assets::get_asset_path(if is_light {
        "wallpapers/mountain_classic_light.jpg"
    } else {
        "wallpapers/mountain_classic.jpg"
    });

    let text_color = if is_light { Color::BLACK } else { Color::WHITE };

    // --- Components ---

    let status_bar = container(
        row![
            text(time).size(14).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            iced::widget::horizontal_space(),
            row![
                svg(peak_core::icons::get_status_icon(
                    "wifi",
                    if is_light { "#000000" } else { "#FFFFFF" }
                ))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0)),
                iced::widget::Space::with_width(5.0),
                svg(peak_core::icons::get_status_icon(
                    "battery",
                    if is_light { "#000000" } else { "#FFFFFF" }
                ))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0)),
            ]
            .align_y(Alignment::Center)
        ]
        .padding([5, 20])
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fixed(44.0));

    let home_indicator = container(
        container(iced::widget::Space::new(
            Length::Fixed(140.0),
            Length::Fixed(5.0),
        ))
        .style(move |_| container::Style {
            background: Some(Background::Color(if is_light {
                Color::from_rgba(0.0, 0.0, 0.0, 0.5)
            } else {
                Color::from_rgba(1.0, 1.0, 1.0, 0.5)
            })),
            border: iced::Border {
                radius: 2.5.into(),
                ..Default::default()
            },
            ..Default::default()
        }),
    )
    .width(Length::Fill)
    .padding(10)
    .align_x(iced::alignment::Horizontal::Center);

    let content: Element<'a, Message, iced::Theme, iced::Renderer> = if let Some(app_id) =
        current_app
    {
        // App View Implementation
        let info = AppInfo::get_info(app_id);
        let app_content: Element<'a, Message, iced::Theme, iced::Renderer> = if app_id
            == AppId::Terminal
        {
            // Mobile Terminal View
            let output = text::<iced::Theme, iced::Renderer>(&terminal_app.content)
                .font(iced::Font::MONOSPACE)
                .size(13)
                .color(text_color);

            let on_term = on_terminal.clone();
            let input = text_input::<Message, iced::Theme, iced::Renderer>(
                "Type command...",
                &terminal_app.input_buffer,
            )
            .on_input(move |v| on_term(peak_core::apps::terminal::TerminalMessage::InputChanged(v)))
            .on_submit(on_terminal(
                peak_core::apps::terminal::TerminalMessage::InputSubmitted,
            ))
            .padding(15)
            .size(14)
            .style(move |_, _| text_input::Style {
                background: Color::TRANSPARENT.into(),
                border: iced::Border::default(),
                icon: Color::TRANSPARENT,
                placeholder: Color::from_rgb(0.4, 0.4, 0.4),
                value: text_color,
                selection: Color::from_rgba(0.0, 0.5, 1.0, 0.3),
            });

            column![
                scrollable(container(output).padding(15)).height(Length::Fill),
                container(input)
                    .width(Length::Fill)
                    .style(move |_| container::Style {
                        background: Some(if is_light {
                            Color::from_rgba(0.0, 0.0, 0.0, 0.05).into()
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.05).into()
                        }),
                        border: iced::Border {
                            width: 1.0,
                            color: if is_light {
                                Color::from_rgba(0.0, 0.0, 0.0, 0.1)
                            } else {
                                Color::from_rgba(1.0, 1.0, 1.0, 0.1)
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    })
            ]
            .into()
        } else if app_id == AppId::Settings {
            // Mobile Settings View
            let sections = column![
                mobile_section(
                    "System",
                    column![
                        mobile_row(
                            "Wi-Fi",
                            if settings_app.wifi_enabled {
                                "On"
                            } else {
                                "Off"
                            },
                            text_color
                        ),
                        mobile_row(
                            "Bluetooth",
                            if settings_app.bluetooth_enabled {
                                "On"
                            } else {
                                "Off"
                            },
                            text_color
                        ),
                        mobile_row("Battery", "85%", text_color),
                    ],
                    is_light
                ),
                iced::widget::vertical_space().height(20),
                mobile_section(
                    "Appearance",
                    column![mobile_row(
                        "Theme",
                        match settings_app.theme_mode {
                            peak_core::apps::settings::ThemeMode::Light => "Light",
                            peak_core::apps::settings::ThemeMode::Dark => "Dark",
                        },
                        text_color
                    ),],
                    is_light
                ),
                iced::widget::vertical_space().height(20),
                mobile_section(
                    "About",
                    column![
                        mobile_row("Device Name", "Peak Mobile", text_color),
                        mobile_row("Version", "1.0.0 Alpha", text_color),
                    ],
                    is_light
                ),
            ]
            .padding(20);

            scrollable(sections).into()
        } else {
            container(text(format!("App: {:?}", app_id)).color(text_color))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        };

        column![
            button(
                container(text(format!("← {}", info.name)).size(18).color(text_color))
                    .width(Length::Fill)
                    .height(Length::Fixed(50.0))
                    .padding([0, 20])
                    .align_y(iced::alignment::Vertical::Center)
            )
            .on_press(on_home)
            .style(|_, _| button::Style::default()),
            app_content,
            home_indicator,
        ]
        .into()
    } else {
        // Home Screen Implementation
        let apps = vec![
            AppId::Terminal,
            AppId::Browser,
            AppId::Library,
            AppId::Settings,
            AppId::FileManager,
            AppId::Store,
            AppId::Turntable,
            AppId::Cortex,
        ];

        let mut grid = column![].spacing(25).align_x(Alignment::Center);
        for chunk in apps.chunks(4) {
            let mut r = row![].spacing(20);
            for &id in chunk {
                let info = AppInfo::get_info(id);
                let icon_color = if is_light { "#000000" } else { "#FFFFFF" };

                r = r.push(
                    button(
                        column![
                            container(
                                svg(peak_core::icons::get_app_icon(id, icon_color))
                                    .width(Length::Fixed(35.0))
                                    .height(Length::Fixed(35.0))
                            )
                            .width(Length::Fixed(60.0))
                            .height(Length::Fixed(60.0))
                            .center_x(Length::Fill)
                            .center_y(Length::Fill)
                            .style(move |_| container::Style {
                                background: Some(Background::Color(if is_light {
                                    Color::from_rgba(1.0, 1.0, 1.0, 0.9)
                                } else {
                                    Color::from_rgba(0.2, 0.2, 0.2, 0.8)
                                })),
                                border: iced::Border {
                                    radius: 14.0.into(),
                                    ..Default::default()
                                },
                                shadow: Shadow {
                                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                                    offset: Vector::new(0.0, 2.0),
                                    blur_radius: 4.0,
                                },
                                ..Default::default()
                            }),
                            text(info.name).size(11).color(Color::WHITE)
                        ]
                        .align_x(Alignment::Center)
                        .spacing(6),
                    )
                    .on_press(on_open(id))
                    .style(|_, _| button::Style::default()),
                );
            }
            grid = grid.push(r);
        }

        let dock_apps = vec![
            AppId::Terminal,
            AppId::Browser,
            AppId::Library,
            AppId::Store,
        ];
        let mut dock_row = row![].spacing(20).align_y(Alignment::Center);
        for &id in &dock_apps {
            let icon_color = if is_light { "#000000" } else { "#FFFFFF" };
            dock_row = dock_row.push(
                button(
                    container(
                        svg(peak_core::icons::get_app_icon(id, icon_color))
                            .width(Length::Fixed(30.0))
                            .height(Length::Fixed(30.0)),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(if is_light {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.9)
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.2)
                        })),
                        border: iced::Border {
                            radius: 14.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                )
                .on_press(on_open(id))
                .style(|_, _| button::Style::default())
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0)),
            );
        }

        let dock = container(dock_row)
            .padding(15)
            .style(move |_| container::Style {
                background: Some(Background::Color(if is_light {
                    Color::from_rgba(1.0, 1.0, 1.0, 0.3)
                } else {
                    Color::from_rgba(1.0, 1.0, 1.0, 0.1)
                })),
                border: iced::Border {
                    radius: 24.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });

        column![
            iced::widget::vertical_space(),
            container(grid).width(Length::Fill).padding([0, 20]),
            iced::widget::vertical_space(),
            container(dock)
                .width(Length::Fill)
                .center_x(Length::Fill)
                .padding(20),
            home_indicator,
        ]
        .into()
    };

    stack![
        image(image::Handle::from_path(wallpaper_path))
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Cover),
        column![status_bar, content,]
    ]
    .into()
}

fn mobile_section<'a, Message: 'a>(
    title: &'a str,
    content: impl Into<Element<'a, Message>>,
    is_light: bool,
) -> Element<'a, Message> {
    column![
        text(title)
            .size(14)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .color(if is_light {
                Color::from_rgb(0.4, 0.4, 0.4)
            } else {
                Color::from_rgb(0.6, 0.6, 0.6)
            }),
        vertical_space().height(8),
        container(content)
            .width(Length::Fill)
            .padding(15)
            .style(move |_| container::Style {
                background: Some(if is_light {
                    Color::from_rgba(1.0, 1.0, 1.0, 0.8).into()
                } else {
                    Color::from_rgba(0.2, 0.2, 0.2, 0.8).into()
                }),
                border: iced::Border {
                    radius: 15.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
    ]
    .into()
}

fn mobile_row<'a, Message: 'a>(
    label: &'a str,
    value: &'a str,
    text_color: Color,
) -> Element<'a, Message> {
    row![
        text(label).size(16).color(text_color),
        horizontal_space(),
        text(value).size(16).color(Color::from_rgb(0.5, 0.5, 0.5)),
    ]
    .align_y(Alignment::Center)
    .padding(5)
    .into()
}
