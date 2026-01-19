use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, vertical_space, Rule,
};
use iced::{Alignment, Color, Element, Length, Task};
pub use peak_core::apps::settings::{SettingsApp, SettingsMessage, SettingsTab, ThemeMode};
use peak_core::theme::Theme;
use peak_core::registry::ShellMode;
use peak_theme::{ThemeTokens, ThemeTone};

pub trait SettingsDesktopView {
    fn view<'a>(
        &self,
        theme: &Theme,
        handles: &std::collections::HashMap<String, iced::widget::image::Handle>,
    ) -> Element<'a, SettingsMessage, iced::Theme, iced::Renderer>;

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
        handles: &std::collections::HashMap<String, iced::widget::image::Handle>,
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

    fn mode_preview<'a>(
        &self,
        label: &'a str,
        mode: ShellMode,
        is_light: bool,
    ) -> Element<'a, SettingsMessage>;

    fn shell_style_preview<'a>(
        &self,
        label: &'a str,
        style: peak_core::registry::ShellStyle,
        is_light: bool,
    ) -> Element<'a, SettingsMessage>;
}



impl SettingsDesktopView for SettingsApp {
    fn view<'a>(
        &self,
        theme: &Theme,
        handles: &std::collections::HashMap<String, iced::widget::image::Handle>,
    ) -> Element<'a, SettingsMessage, iced::Theme, iced::Renderer> {
        let is_light = *theme == Theme::Light;
        let tokens = ThemeTokens::get(
            peak_core::registry::ShellMode::Desktop,
            if is_light {
                ThemeTone::Light
            } else {
                ThemeTone::Dark
            }
        );
        let border_color = if is_light {
            Color::from_rgba(0.0, 0.0, 0.0, 0.1)
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.1)
        };

        let sidebar = peak_ui::navigation::Sidebar::new("Settings", tokens)
            .with_search(self.search_query.clone(), SettingsMessage::SearchChanged)
            // ... (rest of sidebar items)
            .item(
                "Network",
                "wifi_full",
                SettingsMessage::TabChanged(SettingsTab::WiFi),
                self.current_tab == SettingsTab::WiFi,
            )
            .item(
                "Bluetooth",
                "wifi_off",
                SettingsMessage::TabChanged(SettingsTab::Bluetooth),
                self.current_tab == SettingsTab::Bluetooth,
            )
            .item(
                "General",
                "settings",
                SettingsMessage::TabChanged(SettingsTab::General),
                self.current_tab == SettingsTab::General,
            )
            .item(
                "Appearance",
                "palette",
                SettingsMessage::TabChanged(SettingsTab::Appearance),
                self.current_tab == SettingsTab::Appearance,
            )
            .item(
                "Display",
                "about",
                SettingsMessage::TabChanged(SettingsTab::Display),
                self.current_tab == SettingsTab::Display,
            )
            .item(
                "Sound",
                "media",
                SettingsMessage::TabChanged(SettingsTab::Sound),
                self.current_tab == SettingsTab::Sound,
            )
            .item(
                "Focus",
                "moon",
                SettingsMessage::TabChanged(SettingsTab::Focus),
                self.current_tab == SettingsTab::Focus,
            )
            .item(
                "Intelligence",
                "sparkles",
                SettingsMessage::TabChanged(SettingsTab::Intelligence),
                self.current_tab == SettingsTab::Intelligence,
            )
            .item(
                "Modes",
                "trigger",
                SettingsMessage::TabChanged(SettingsTab::Modes),
                self.current_tab == SettingsTab::Modes,
            );

        let content = scrollable(
            column![
                text(format!("{:?}", self.current_tab))
                    .size(24)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(if is_light { Color::BLACK } else { Color::WHITE }),
                vertical_space().height(20),
                self.view_tab_content(is_light, border_color, handles)
            ]
            .padding(32)
            .max_width(800),
        );

        peak_ui::navigation::NavigationSplitView::new(sidebar, content.into())
            .theme(is_light)
            .view()
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
                    text_color,
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
        handles: &std::collections::HashMap<String, iced::widget::image::Handle>,
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

            SettingsTab::Appearance => {
                // Build wallpaper grid (4 per row)
                let wallpaper_grid = {
                    let wallpapers_per_row = 4;
                    let rows: Vec<Element<'_, SettingsMessage>> = self.wallpapers
                        .chunks(wallpapers_per_row)
                        .map(|chunk| {
                            row(chunk.iter().map(|wp| {
                                let is_selected = self.current_wallpaper == *wp;
                                let border_color = if is_selected {
                                    Color::from_rgb8(0, 122, 255)
                                } else {
                                    Color::from_rgba(0.0, 0.0, 0.0, 0.1)
                                };
                                let wp_clone = wp.clone();
                                let image_path = peak_core::utils::assets::get_asset_path(
                                    &format!("wallpapers/{}", wp),
                                );

                                // Debug: print path if file doesn't exist
                                if !image_path.exists() {
                                    eprintln!("⚠️ Wallpaper not found: {}", image_path.display());
                                }

                                 button(
                                     container(
                                         if let Some(handle) = handles.get(wp) {
                                             iced::widget::image(handle.clone())
                                         } else {
                                             iced::widget::image(iced::widget::image::Handle::from_path(&image_path))
                                         }
                                         .width(120)
                                         .height(75)
                                         .content_fit(iced::ContentFit::Cover),
                                     )
                                    .width(120)
                                    .height(75)
                                    .clip(true)
                                    .style(move |_: &iced::Theme| container::Style {
                                        background: Some(if is_light {
                                            Color::from_rgb(0.95, 0.95, 0.95).into()
                                        } else {
                                            Color::from_rgb(0.2, 0.2, 0.2).into()
                                        }),
                                        border: iced::Border {
                                            radius: 6.0.into(),
                                            width: if is_selected { 3.0 } else { 1.0 },
                                            color: border_color,
                                        },
                                        ..Default::default()
                                    }),
                                )
                                .on_press(SettingsMessage::WallpaperChanged(wp_clone))
                                .padding(0)
                                .style(|_: &iced::Theme, _| button::Style {
                                    background: None,
                                    ..Default::default()
                                })
                                .into()
                            }))
                            .spacing(10)
                            .into()
                        })
                        .collect();
                    
                    column(rows).spacing(10).width(Length::Fill)
                };

                column![
                    self.section(
                        "Theme",
                        row![
                            text(if self.theme_mode == ThemeMode::Light { "Light" } else { "Dark" }).size(13),
                            horizontal_space(),
                            iced::widget::toggler(self.theme_mode == ThemeMode::Dark)
                                .on_toggle(|is_dark| {
                                    if is_dark {
                                        SettingsMessage::ThemeChanged(ThemeMode::Dark)
                                    } else {
                                        SettingsMessage::ThemeChanged(ThemeMode::Light)
                                    }
                                })
                                .width(Length::Shrink)
                        ]
                        .align_y(Alignment::Center),
                        is_light
                    ),
                    vertical_space().height(20),
                    self.section(
                        "Desktop Interface",
                        scrollable(
                            row![
                                self.shell_style_preview("Cupertino", peak_core::registry::ShellStyle::Cupertino, is_light),
                                self.shell_style_preview("Redmond", peak_core::registry::ShellStyle::Redmond, is_light),
                                self.shell_style_preview("AI", peak_core::registry::ShellStyle::AI, is_light),
                                self.shell_style_preview("Console", peak_core::registry::ShellStyle::Console, is_light),
                                self.shell_style_preview("TV", peak_core::registry::ShellStyle::TV, is_light),
                            ]
                            .spacing(12)
                        ).direction(iced::widget::scrollable::Direction::Horizontal(iced::widget::scrollable::Scrollbar::default())),
                        is_light,
                    ),
                    vertical_space().height(20),
                    text("Wallpaper")
                        .size(13)
                        .font(iced::Font {
                            weight: iced::font::Weight::Medium,
                            ..Default::default()
                        })
                        .color(if is_light { Color::from_rgb(0.4, 0.4, 0.4) } else { Color::from_rgb(0.6, 0.6, 0.6) }),
                    vertical_space().height(6),
                    scrollable(wallpaper_grid)
                        .height(300)
                        .width(Length::Fill)
                ]
                .into()
            }

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
                        Rule::horizontal(1).style(move |_: &iced::Theme| {
                            iced::widget::rule::Style {
                                color: border_color,
                                width: 1,
                                radius: 0.0.into(),
                                fill_mode: iced::widget::rule::FillMode::Full,
                            }
                        }),
                        self.labeled_row(
                            "Voice",
                            iced::widget::toggler(self.voice_enabled)
                                .on_toggle(SettingsMessage::ToggleVoice)
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

            SettingsTab::Modes => column![
                self.section(
                    "OS Modes",
                    column![
                        row![
                            self.mode_preview("Desktop", ShellMode::Desktop, is_light),
                            self.mode_preview("Mobile", ShellMode::Mobile, is_light),
                            self.mode_preview("TV", ShellMode::TV, is_light),
                            self.mode_preview("Console", ShellMode::Console, is_light),
                            self.mode_preview("Kiosk", ShellMode::Kiosk, is_light),
                        ]
                        .spacing(15),
                        row![
                            self.mode_preview("Fireplace", ShellMode::Fireplace, is_light),
                            self.mode_preview("Auto", ShellMode::Auto, is_light),
                            self.mode_preview("Robot", ShellMode::Robot, is_light),
                            self.mode_preview("Server", ShellMode::Server, is_light),
                            self.mode_preview("Home", ShellMode::SmartHome, is_light),
                        ]
                        .spacing(15)
                    ]
                    .spacing(20),
                    is_light
                ),
                vertical_space().height(20),
                text("Select a mode to rethink your desktop experience.")
                    .size(13)
                    .color(if is_light { Color::from_rgb(0.4, 0.4, 0.4) } else { Color::from_rgb(0.6, 0.6, 0.6) }),
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
                .color(if is_light { Color::from_rgb(0.4, 0.4, 0.4) } else { Color::from_rgb(0.6, 0.6, 0.6) }),
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

    fn mode_preview<'a>(
        &self,
        label: &'a str,
        mode: ShellMode,
        is_light: bool,
    ) -> Element<'a, SettingsMessage> {
        let is_selected = self.current_mode == mode;
        let border_color = if is_selected {
            Color::from_rgb8(0, 122, 255)
        } else {
            Color::TRANSPARENT
        };

        button(
            column![
                container(
                    container(
                        iced::widget::svg(peak_core::icons::get_mode_icon(mode, "#FFFFFF"))
                            .width(28)
                            .height(28)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .style(move |_: &iced::Theme| {
                        let tokens = ThemeTokens::get(
                            mode,
                            if is_light {
                                ThemeTone::Light
                            } else {
                                ThemeTone::Dark
                            },
                        );
                        container::Style {
                            background: Some(tokens.accent.into()),
                            border: iced::Border {
                                radius: 6.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    })
                )
                .width(100)
                .height(64)
                .padding(2)
                .style(move |_: &iced::Theme| container::Style {
                    border: iced::Border {
                        width: 2.0,
                        color: border_color,
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                }),
                text(label)
                    .size(11)
                    .color(if is_light { Color::BLACK } else { Color::WHITE })
                    .align_x(Alignment::Center),
            ]
            .spacing(6)
            .align_x(Alignment::Center),
        )
        .on_press(SettingsMessage::ModeChanged(mode))
        .padding(4)
        .style(move |_, _| button::Style {
            background: None,
            ..Default::default()
        })
        .into()
    }

    fn shell_style_preview<'a>(
        &self,
        label: &'a str,
        style: peak_core::registry::ShellStyle,
        is_light: bool,
    ) -> Element<'a, SettingsMessage> {
        let is_selected = self.current_shell_style == style;
        let border_color = if is_selected {
            Color::from_rgb8(0, 122, 255)
        } else {
            Color::TRANSPARENT
        };

        button(
            column![
                container(
                    container(
                        text(match style {
                            peak_core::registry::ShellStyle::Cupertino => "◈",
                            peak_core::registry::ShellStyle::Redmond => "⊞",
                            peak_core::registry::ShellStyle::AI => "✧",
                            peak_core::registry::ShellStyle::Console => "🎮",
                            peak_core::registry::ShellStyle::TV => "📺",
                        })
                        .size(24)
                        .color(if is_light { Color::BLACK } else { Color::WHITE })
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .style(move |_: &iced::Theme| {
                        container::Style {
                            background: Some(if is_selected { 
                                Color::from_rgba(0.0, 0.48, 1.0, 0.2).into() 
                            } else { 
                                Color::from_rgba(0.5, 0.5, 0.5, 0.1).into() 
                            }),
                            border: iced::Border {
                                radius: 6.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    })
                )
                .width(100)
                .height(64)
                .padding(2)
                .style(move |_: &iced::Theme| container::Style {
                    border: iced::Border {
                        width: 2.0,
                        color: border_color,
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                }),
                text(label)
                    .size(11)
                    .color(if is_light { Color::BLACK } else { Color::WHITE })
                    .align_x(Alignment::Center),
            ]
            .spacing(6)
            .align_x(Alignment::Center),
        )
        .on_press(SettingsMessage::ShellStyleChanged(style))
        .padding(4)
        .style(move |_, _| button::Style {
            background: None,
            ..Default::default()
        })
        .into()
    }
}
// Wrapper for Registry
pub struct DesktopSettingsApp {
    pub app: SettingsApp,
    pub handles: std::collections::HashMap<String, iced::widget::image::Handle>,
}

impl Default for DesktopSettingsApp {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopSettingsApp {
    pub fn new() -> Self {
        // Dynamically scan wallpapers directory
        let wallpapers_dir = peak_core::utils::assets::get_asset_path("wallpapers");
        let wallpapers: Vec<String> = std::fs::read_dir(&wallpapers_dir)
            .ok()
            .map(|entries| {
                let mut list: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .filter_map(|e| {
                        let path = e.path();
                        if path.is_file() {
                            let ext = path.extension()?.to_str()?.to_lowercase();
                            if ["jpg", "jpeg", "png", "webp"].contains(&ext.as_str()) {
                                return Some(e.file_name().to_string_lossy().to_string());
                            }
                        }
                        None
                    })
                    .collect();
                list.sort();
                list
            })
            .unwrap_or_else(|| {
                // Fallback to known wallpapers if dir scan fails
                vec![
                    "mountain_classic.jpg".to_string(),
                    "mountain_classic_light.jpg".to_string(),
                ]
            });
        
        // Pre-load handles
        let mut handles = std::collections::HashMap::new();
        for wp in &wallpapers {
            let path = wallpapers_dir.join(wp);
            handles.insert(wp.clone(), iced::widget::image::Handle::from_path(path));
        }

        let mut app = SettingsApp::new();
        app.wallpapers = wallpapers;
        Self { app, handles }
    }
}

use peak_core::app_traits::{PeakApp, ShellContext};



impl PeakApp for DesktopSettingsApp {
    type Message = SettingsMessage;

    fn title(&self) -> String {
        self.app.title()
    }

    fn update(
        &mut self,
        message: Self::Message,
        context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        self.app.update(message, context)
    }

    fn view(&self, theme: &Theme) -> Element<'_, Self::Message> {
        SettingsDesktopView::view(&self.app, theme, &self.handles)
    }
}
