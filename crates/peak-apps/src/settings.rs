use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, vertical_space, Rule,
};
use iced::{Alignment, Color, Element, Length, Task};
pub use peak_core::apps::settings::{SettingsApp, SettingsMessage, SettingsTab, ThemeMode};
use peak_core::theme::Theme;

pub trait SettingsDesktopView {
    fn view<'a>(&self, theme: &Theme) -> Element<'a, SettingsMessage, iced::Theme, iced::Renderer>;

    fn sidebar_group<'a>(
        &self,
        items: Vec<(&'a str, SettingsTab, &str)>,
        text_color: Color,
        accent_color: Color,
        is_light: bool,
    ) -> Element<'a, SettingsMessage>;

    fn view_tab_content<'a>(
        &self,
        is_light: bool,
        border_color: Color,
    ) -> Element<'a, SettingsMessage>;

    fn section<'a>(
        &self,
        title: &'a str,
        content: impl Into<Element<'a, SettingsMessage>>,
        is_light: bool,
    ) -> Element<'a, SettingsMessage>;

    fn labeled_row<'a>(
        &self,
        label: &'a str,
        widget: impl Into<Element<'a, SettingsMessage>>,
    ) -> Element<'a, SettingsMessage>;

    fn theme_preview<'a>(
        &self,
        label: &'a str,
        mode: ThemeMode,
        is_light: bool,
    ) -> Element<'a, SettingsMessage>;
}

impl SettingsDesktopView for SettingsApp {
    fn view<'a>(&self, theme: &Theme) -> Element<'a, SettingsMessage, iced::Theme, iced::Renderer> {
        let is_light = *theme == Theme::Light;
        let border_color = if is_light {
            Color::from_rgba(0.0, 0.0, 0.0, 0.1)
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.1)
        };

        let sidebar = peak_ui::navigation::Sidebar::new("Settings")
            .with_search(self.search_query.clone(), SettingsMessage::SearchChanged)
            .item(
                "Network",
                "wifi_full.svg",
                SettingsMessage::TabChanged(SettingsTab::WiFi),
                self.current_tab == SettingsTab::WiFi,
            )
            .item(
                "Bluetooth",
                "bluetooth.svg",
                SettingsMessage::TabChanged(SettingsTab::Bluetooth),
                self.current_tab == SettingsTab::Bluetooth,
            )
            .item(
                "General",
                "settings.svg",
                SettingsMessage::TabChanged(SettingsTab::General),
                self.current_tab == SettingsTab::General,
            )
            .item(
                "Appearance",
                "sparkles.svg",
                SettingsMessage::TabChanged(SettingsTab::Appearance),
                self.current_tab == SettingsTab::Appearance,
            )
            .item(
                "Display",
                "display.svg",
                SettingsMessage::TabChanged(SettingsTab::Display),
                self.current_tab == SettingsTab::Display,
            )
            .item(
                "Sound",
                "speaker.svg",
                SettingsMessage::TabChanged(SettingsTab::Sound),
                self.current_tab == SettingsTab::Sound,
            )
            .item(
                "Focus",
                "focus.svg",
                SettingsMessage::TabChanged(SettingsTab::Focus),
                self.current_tab == SettingsTab::Focus,
            )
            .item(
                "Intelligence",
                "sparkles.svg",
                SettingsMessage::TabChanged(SettingsTab::Intelligence),
                self.current_tab == SettingsTab::Intelligence,
            );

        let content = scrollable(
            column![
                text(format!("{:?}", self.current_tab))
                    .size(24)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
                vertical_space().height(20),
                self.view_tab_content(is_light, border_color)
            ]
            .padding(32)
            .max_width(800),
        );

        peak_ui::navigation::NavigationSplitView::new(sidebar, content.into()).view()
    }

    fn sidebar_group<'a>(
        &self,
        items: Vec<(&'a str, SettingsTab, &str)>,
        text_color: Color,
        accent_color: Color,
        is_light: bool,
    ) -> Element<'a, SettingsMessage> {
        let mut col = column![].spacing(2);
        for (label, tab, _icon) in items {
            let is_selected = self.current_tab == tab;
            col = col.push(
                button(
                    row![
                        // Icon placeholder (todo: real icons)
                        container(iced::widget::text("").size(10))
                            .width(20)
                            .height(20)
                            .style(move |_: &iced::Theme| container::Style {
                                background: Some(if is_selected {
                                    accent_color.into()
                                } else {
                                    Color::from_rgb(0.5, 0.5, 0.5).into()
                                }),
                                border: iced::Border {
                                    radius: 4.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                        text(label).size(13)
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                )
                .on_press(SettingsMessage::TabChanged(tab))
                .width(Length::Fill)
                .padding(6)
                .style(move |_: &iced::Theme, status| button::Style {
                    background: if is_selected {
                        Some(if is_light {
                            Color::from_rgba(0.0, 0.0, 0.0, 0.05).into()
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()
                        })
                    } else if status == iced::widget::button::Status::Hovered {
                        Some(if is_light {
                            Color::from_rgba(0.0, 0.0, 0.0, 0.03).into()
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()
                        })
                    } else {
                        None
                    },
                    text_color: text_color,
                    border: iced::Border {
                        radius: 5.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
        }
        col.into()
    }

    fn view_tab_content<'a>(
        &self,
        is_light: bool,
        border_color: Color,
    ) -> Element<'a, SettingsMessage> {
        match self.current_tab {
            SettingsTab::General => column![
                self.section(
                    "About",
                    column![
                        self.labeled_row(
                            "Name",
                            text("PeakOS Device")
                                .size(13)
                                .color(Color::from_rgb(0.5, 0.5, 0.5))
                        ),
                        Rule::horizontal(1).style(move |_: &iced::Theme| {
                            iced::widget::rule::Style {
                                color: border_color,
                                width: 1,
                                radius: 0.0.into(),
                                fill_mode: iced::widget::rule::FillMode::Full,
                            }
                        }),
                        self.labeled_row(
                            "Model",
                            text("Peak Native (Apple Silicon)")
                                .size(13)
                                .color(Color::from_rgb(0.5, 0.5, 0.5))
                        ),
                        Rule::horizontal(1).style(move |_: &iced::Theme| {
                            iced::widget::rule::Style {
                                color: border_color,
                                width: 1,
                                radius: 0.0.into(),
                                fill_mode: iced::widget::rule::FillMode::Full,
                            }
                        }),
                        self.labeled_row(
                            "Version",
                            text("1.0.0 (Alpha)")
                                .size(13)
                                .color(Color::from_rgb(0.5, 0.5, 0.5))
                        ),
                    ],
                    is_light
                ),
                vertical_space().height(20),
                self.section(
                    "Software Update",
                    column![self.labeled_row(
                        "Automatic Updates",
                        text("On").size(13).color(Color::from_rgb(0.5, 0.5, 0.5))
                    ),],
                    is_light
                )
            ]
            .spacing(20)
            .into(),

            SettingsTab::Appearance => column![
                self.section(
                    "Appearance",
                    column![row![
                        self.theme_preview("Light", ThemeMode::Light, is_light),
                        self.theme_preview("Dark", ThemeMode::Dark, is_light),
                    ]
                    .spacing(20)],
                    is_light
                ),
                vertical_space().height(20),
                self.section(
                    "Wallpaper",
                    iced::widget::scrollable(
                        iced::widget::row(self.wallpapers.iter().map(|wp| {
                            let is_selected = self.current_wallpaper == *wp;
                            let border_color = if is_selected {
                                Color::from_rgb8(0, 122, 255)
                            } else {
                                Color::TRANSPARENT
                            };

                            button(
                                container(
                                    iced::widget::image(peak_core::utils::assets::get_asset_path(
                                        &format!("wallpapers/{}", wp),
                                    ))
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .content_fit(iced::ContentFit::Cover),
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .style(move |_: &iced::Theme| container::Style {
                                    border: iced::Border {
                                        radius: 8.0.into(),
                                        width: 3.0,
                                        color: border_color,
                                    },
                                    ..Default::default()
                                }),
                            )
                            .on_press(SettingsMessage::WallpaperChanged(wp.clone()))
                            .width(160)
                            .height(100)
                            .padding(0)
                            .style(|_: &iced::Theme, _| button::Style::default())
                            .into()
                        }))
                        .spacing(15)
                    )
                    .direction(iced::widget::scrollable::Direction::Horizontal(
                        iced::widget::scrollable::Scrollbar::new()
                    ))
                    .height(130),
                    is_light
                )
            ]
            .into(),

            SettingsTab::WiFi => column![
                self.section(
                    "Wi-Fi",
                    column![row![
                        text("Wi-Fi").size(13),
                        horizontal_space(),
                        iced::widget::toggler(self.wifi_enabled)
                            .on_toggle(SettingsMessage::ToggleWiFi)
                            .width(Length::Shrink)
                    ]
                    .align_y(Alignment::Center)],
                    is_light
                ),
                vertical_space().height(10),
                if self.wifi_enabled {
                    self.section(
                        "Known Networks",
                        column![
                            self.labeled_row(
                                "Home WiFi",
                                text("Connected")
                                    .size(13)
                                    .color(Color::from_rgb(0.0, 0.8, 0.0))
                            ),
                            Rule::horizontal(1).style(move |_: &iced::Theme| {
                                iced::widget::rule::Style {
                                    color: border_color,
                                    width: 1,
                                    radius: 0.0.into(),
                                    fill_mode: iced::widget::rule::FillMode::Full,
                                }
                            }),
                            self.labeled_row(
                                "Office",
                                text("Saved").size(13).color(Color::from_rgb(0.5, 0.5, 0.5))
                            ),
                        ],
                        is_light,
                    )
                } else {
                    text("Wi-Fi is off.")
                        .size(13)
                        .color(Color::from_rgb(0.5, 0.5, 0.5))
                        .into()
                }
            ]
            .into(),

            SettingsTab::Bluetooth => column![
                self.section(
                    "Bluetooth",
                    column![row![
                        text("Bluetooth").size(13),
                        horizontal_space(),
                        iced::widget::toggler(self.bluetooth_enabled)
                            .on_toggle(SettingsMessage::ToggleBluetooth)
                            .width(Length::Shrink)
                    ]
                    .align_y(Alignment::Center)],
                    is_light
                ),
                vertical_space().height(10),
                if self.bluetooth_enabled {
                    self.section(
                        "My Devices",
                        column![
                            self.labeled_row(
                                "AirPods Pro",
                                text("Not Connected")
                                    .size(13)
                                    .color(Color::from_rgb(0.5, 0.5, 0.5))
                            ),
                            Rule::horizontal(1).style(move |_: &iced::Theme| {
                                iced::widget::rule::Style {
                                    color: border_color,
                                    width: 1,
                                    radius: 0.0.into(),
                                    fill_mode: iced::widget::rule::FillMode::Full,
                                }
                            }),
                            self.labeled_row(
                                "Magic Keyboard",
                                text("Connected")
                                    .size(13)
                                    .color(Color::from_rgb(0.0, 0.8, 0.0))
                            ),
                        ],
                        is_light,
                    )
                } else {
                    text("Bluetooth is off.")
                        .size(13)
                        .color(Color::from_rgb(0.5, 0.5, 0.5))
                        .into()
                }
            ]
            .into(),

            SettingsTab::Sound => column![self.section(
                "Output & Input",
                column![self.labeled_row(
                    "Output Volume",
                    iced::widget::slider(0.0..=1.0, self.volume, SettingsMessage::VolumeChanged)
                        .width(200)
                ),],
                is_light
            )]
            .into(),

            SettingsTab::Intelligence => column![
                self.section(
                    "Peak Intelligence",
                    column![
                        self.labeled_row("Status", {
                            let active_model = self
                                .recommended_models
                                .iter()
                                .chain(self.custom_models.iter())
                                .find(|m| m.is_active);
                            match active_model {
                                Some(m) => text(format!("Active ({})", m.name))
                                    .size(13)
                                    .color(Color::from_rgb(0.0, 0.8, 0.0)),
                                None => text("No Active Model")
                                    .size(13)
                                    .color(Color::from_rgb(0.5, 0.5, 0.5)),
                            }
                        }),
                        Rule::horizontal(1).style(move |_: &iced::Theme| {
                            iced::widget::rule::Style {
                                color: border_color,
                                width: 1,
                                radius: 0.0.into(),
                                fill_mode: iced::widget::rule::FillMode::Full,
                            }
                        }),
                        self.labeled_row(
                            "Captions",
                            iced::widget::toggler(self.captions_enabled)
                                .on_toggle(SettingsMessage::ToggleCaptions)
                                .width(Length::Shrink)
                        ),
                    ],
                    is_light
                ),
                vertical_space().height(20),
                self.section(
                    "Models",
                    column![
                        // Recommended & Custom Models
                        column(
                            self.recommended_models
                                .iter()
                                .chain(self.custom_models.iter())
                                .cloned()
                                .map(|model| {
                                    let is_downloaded = model.is_downloaded;
                                    let is_active = model.is_active;

                                    container(
                                        row![
                                            column![
                                                row![
                                                    text(model.name).size(13).font(iced::Font {
                                                        weight: iced::font::Weight::Bold,
                                                        ..Default::default()
                                                    }),
                                                    if model.min_ram_gb > 0 {
                                                        Element::<
                                                            SettingsMessage,
                                                            iced::Theme,
                                                            iced::Renderer,
                                                        >::from(
                                                            container(
                                                                text(format!(
                                                                    "{}GB+",
                                                                    model.min_ram_gb
                                                                ))
                                                                .size(9)
                                                                .color(Color::WHITE),
                                                            )
                                                            .padding([2, 5])
                                                            .style(|_: &iced::Theme| {
                                                                container::Style {
                                                                    background: Some(
                                                                        Color::from_rgb(
                                                                            0.3, 0.3, 0.35,
                                                                        )
                                                                        .into(),
                                                                    ),
                                                                    border: iced::Border {
                                                                        radius: 8.0.into(),
                                                                        ..Default::default()
                                                                    },
                                                                    ..Default::default()
                                                                }
                                                            }),
                                                        )
                                                    } else {
                                                        horizontal_space().width(0).into()
                                                    }
                                                ]
                                                .spacing(6)
                                                .align_y(iced::Alignment::Center),
                                                text(model.description)
                                                    .size(11)
                                                    .color(Color::from_rgb(0.5, 0.5, 0.5)),
                                                text(model.id.clone())
                                                    .size(10)
                                                    .color(Color::from_rgb(0.7, 0.7, 0.7)),
                                                if let Some(err) = &model.last_error {
                                                    Element::<
                                                        SettingsMessage,
                                                        iced::Theme,
                                                        iced::Renderer,
                                                    >::from(
                                                        text(format!("Error: {}", err))
                                                            .size(10)
                                                            .color(Color::from_rgb(1.0, 0.0, 0.0)),
                                                    )
                                                } else {
                                                    Element::<
                                                        SettingsMessage,
                                                        iced::Theme,
                                                        iced::Renderer,
                                                    >::from(
                                                        horizontal_space().width(0)
                                                    )
                                                }
                                            ]
                                            .spacing(4)
                                            .width(Length::Fill),
                                            if is_active {
                                                Element::<
                                                    SettingsMessage,
                                                    iced::Theme,
                                                    iced::Renderer,
                                                >::from(
                                                    button(text("Active").size(12))
                                                        .style(move |_: &iced::Theme, _| {
                                                            button::Style {
                                                                background: Some(
                                                                    Color::from_rgb(0.0, 0.8, 0.0)
                                                                        .into(),
                                                                ),
                                                                text_color: Color::WHITE,
                                                                border: iced::Border {
                                                                    radius: 12.0.into(),
                                                                    ..Default::default()
                                                                },
                                                                ..Default::default()
                                                            }
                                                        })
                                                        .padding([5, 10]),
                                                )
                                            } else if let Some(progress) = model.download_progress {
                                                Element::<
                                                    SettingsMessage,
                                                    iced::Theme,
                                                    iced::Renderer,
                                                >::from(
                                                    row![
                                                        text(format!(
                                                            "Downloading {:.0}%",
                                                            progress * 100.0
                                                        ))
                                                        .size(12)
                                                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                                                        button(text("✖").size(10))
                                                            .on_press(SettingsMessage::ModelDownloadCancel(
                                                                model.id.clone()
                                                            ))
                                                            .padding([2, 6])
                                                            .style(move |_: &iced::Theme, _| {
                                                                button::Style {
                                                                    background: Some(
                                                                        Color::from_rgb(0.8, 0.2, 0.2)
                                                                            .into(),
                                                                    ),
                                                                    text_color: Color::WHITE,
                                                                    border: iced::Border {
                                                                        radius: 10.0.into(),
                                                                        ..Default::default()
                                                                    },
                                                                    ..Default::default()
                                                                }
                                                            })
                                                    ]
                                                    .spacing(8)
                                                    .align_y(iced::Alignment::Center),
                                                )
                                            } else if is_downloaded {
                                                Element::<
                                                    SettingsMessage,
                                                    iced::Theme,
                                                    iced::Renderer,
                                                >::from(
                                                    button(text("Activate").size(12))
                                                        .on_press(SettingsMessage::ModelActivate(
                                                            model.id.clone(),
                                                        ))
                                                        .style(move |_: &iced::Theme, _| {
                                                            button::Style {
                                                                background: Some(
                                                                    Color::from_rgb(0.2, 0.2, 0.2)
                                                                        .into(),
                                                                ),
                                                                text_color: Color::WHITE,
                                                                border: iced::Border {
                                                                    radius: 12.0.into(),
                                                                    ..Default::default()
                                                                },
                                                                ..Default::default()
                                                            }
                                                        })
                                                        .padding([5, 10]),
                                                )
                                            } else {
                                                Element::<
                                                    SettingsMessage,
                                                    iced::Theme,
                                                    iced::Renderer,
                                                >::from(
                                                    button(text("Download").size(12))
                                                        .on_press(SettingsMessage::ModelDownload(
                                                            model.id.clone(),
                                                        ))
                                                        .style(move |_: &iced::Theme, _| {
                                                            button::Style {
                                                                background: Some(
                                                                    Color::from_rgb(0.0, 0.4, 1.0)
                                                                        .into(),
                                                                ),
                                                                text_color: Color::WHITE,
                                                                border: iced::Border {
                                                                    radius: 12.0.into(),
                                                                    ..Default::default()
                                                                },
                                                                ..Default::default()
                                                            }
                                                        })
                                                        .padding([5, 10]),
                                                )
                                            }
                                        ]
                                        .align_y(iced::Alignment::Center)
                                        .spacing(10),
                                    )
                                    .padding(10)
                                    .style(move |_: &iced::Theme| container::Style {
                                        background: Some(if is_light {
                                            Color::from_rgba(0.0, 0.0, 0.0, 0.02).into()
                                        } else {
                                            Color::from_rgba(1.0, 1.0, 1.0, 0.05).into()
                                        }),
                                        border: iced::Border {
                                            radius: 6.0.into(),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .into()
                                })
                                .collect::<Vec<_>>()
                        )
                        .spacing(10),
                        vertical_space().height(15),
                        text("Add Model")
                            .size(12)
                            .color(Color::from_rgb(0.5, 0.5, 0.5)),
                        row![
                            iced::widget::text_input(
                                "Enter HuggingFace ID (e.g. author/repo)",
                                &self.add_model_input
                            )
                            .on_input(SettingsMessage::AddModelInputChanged)
                            .on_submit(SettingsMessage::AddModelPressed)
                            .padding(8)
                            .width(Length::Fill),
                            button(text("Add").size(13))
                                .on_press(SettingsMessage::AddModelPressed)
                                .padding([8, 15])
                        ]
                        .spacing(10)
                    ],
                    is_light
                )
            ]
            .into(),

            _ => column![text("Placeholder for future settings.")
                .size(13)
                .color(Color::from_rgb(0.5, 0.5, 0.5))]
            .into(),
        }
    }

    fn section<'a>(
        &self,
        title: &'a str,
        content: impl Into<Element<'a, SettingsMessage>>,
        is_light: bool,
    ) -> Element<'a, SettingsMessage> {
        column![
            text(title)
                .size(13)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..Default::default()
                })
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            vertical_space().height(6),
            container(content)
                .width(Length::Fill)
                .padding(10)
                .style(move |_: &iced::Theme| container::Style {
                    background: Some(if is_light {
                        Color::WHITE.into()
                    } else {
                        Color::from_rgb8(30, 30, 30).into()
                    }),
                    border: iced::Border {
                        radius: 6.0.into(),
                        width: 1.0,
                        color: if is_light {
                            Color::from_rgba(0.0, 0.0, 0.0, 0.05)
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.1)
                        },
                    },
                    ..Default::default()
                })
        ]
        .into()
    }

    fn labeled_row<'a>(
        &self,
        label: &'a str,
        widget: impl Into<Element<'a, SettingsMessage>>,
    ) -> Element<'a, SettingsMessage> {
        row![text(label).size(13), horizontal_space(), widget.into()]
            .align_y(Alignment::Center)
            .into()
    }

    fn theme_preview<'a>(
        &self,
        label: &'a str,
        mode: ThemeMode,
        _is_light: bool,
    ) -> Element<'a, SettingsMessage> {
        let is_selected = self.theme_mode == mode;
        let border_color = if is_selected {
            Color::from_rgb8(0, 122, 255)
        } else {
            Color::TRANSPARENT
        };

        button(
            column![
                container(
                    // Preview box
                    container(iced::widget::horizontal_space())
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(move |_: &iced::Theme| container::Style {
                            background: Some(match mode {
                                ThemeMode::Light => Color::WHITE.into(),
                                ThemeMode::Dark => Color::from_rgb8(30, 30, 30).into(),
                            }),
                            border: iced::Border {
                                radius: 4.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                )
                .width(120)
                .height(80)
                .padding(4)
                .style(move |_: &iced::Theme| container::Style {
                    border: iced::Border {
                        width: 2.0,
                        color: border_color,
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                }),
                vertical_space().height(8),
                text(label).size(13)
            ]
            .align_x(Alignment::Center),
        )
        .on_press(SettingsMessage::ThemeChanged(mode))
        .style(|_: &iced::Theme, _| button::Style::default())
        .into()
    }
}
// Wrapper for Registry
pub struct DesktopSettingsApp(pub SettingsApp);

impl DesktopSettingsApp {
    pub fn new() -> Self {
        // Simple hardcoded list for now, relying on assets existing
        // In a real app we'd scan the directory
        let wallpapers = vec![
            "Peak.png".to_string(),
            "Desert.jpeg".to_string(),
            "Florida.jpeg".to_string(),
            "digital_rain.jpg".to_string(),
            "digital_rain_light.jpg".to_string(),
            "mountain_blue_rings.png".to_string(),
            "mountain_classic.jpg".to_string(),
            "mountain_classic_light.jpg".to_string(),
            "mountain_sunset_1.png".to_string(),
            "mountain_sunset_warm.png".to_string(),
            "peak_retro.jpg".to_string(),
            "poolsuite_luxury-kopi.jpg".to_string(),
            "poolsuite_luxury_night.jpg".to_string(),
            "poolsuite_water.png".to_string(),
            "treeInDesert.jpeg".to_string(),
        ];

        let mut app = SettingsApp::new();
        app.wallpapers = wallpapers;
        Self(app)
    }
}

use peak_core::app_traits::{PeakApp, ShellContext};

impl PeakApp for DesktopSettingsApp {
    type Message = SettingsMessage;

    fn title(&self) -> String {
        self.0.title()
    }

    fn update(
        &mut self,
        message: Self::Message,
        context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        self.0.update(message, context)
    }

    fn view(&self, theme: &Theme) -> Element<'_, Self::Message> {
        SettingsDesktopView::view(&self.0, theme)
    }
}
