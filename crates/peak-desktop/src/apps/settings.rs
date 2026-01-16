// use peak_core::utils::assets;

use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input, vertical_space,
    Rule,
};
use iced::{Alignment, Color, Element, Length, Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeMode::Light => write!(f, "Light"),
            ThemeMode::Dark => write!(f, "Dark"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    General,
    WiFi,
    Bluetooth,
    Battery,
    Appearance,
    Display,
    Sound,
    Focus,
    Privacy,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ThemeChanged(ThemeMode),
    VolumeChanged(f32),
    TabChanged(SettingsTab),
    SearchChanged(String),
    ToggleWiFi(bool),
    ToggleBluetooth(bool),
}

pub struct SettingsApp {
    pub theme_mode: ThemeMode,
    pub current_tab: SettingsTab,
    pub volume: f32,
    pub search_query: String,
    pub wifi_enabled: bool,
    pub bluetooth_enabled: bool,
}

impl SettingsApp {
    pub fn new() -> Self {
        Self {
            theme_mode: ThemeMode::Light,
            current_tab: SettingsTab::General, // Start at General like macOS
            volume: 0.8,
            search_query: String::new(),
            wifi_enabled: true,
            bluetooth_enabled: true,
        }
    }

    pub fn update(&mut self, message: SettingsMessage) -> Task<SettingsMessage> {
        match message {
            SettingsMessage::ThemeChanged(mode) => {
                self.theme_mode = mode;
                Task::none()
            }
            SettingsMessage::VolumeChanged(v) => {
                self.volume = v;
                Task::none()
            }
            SettingsMessage::TabChanged(tab) => {
                self.current_tab = tab;
                Task::none()
            }
            SettingsMessage::SearchChanged(q) => {
                self.search_query = q;
                Task::none()
            }
            SettingsMessage::ToggleWiFi(enabled) => {
                self.wifi_enabled = enabled;
                Task::none()
            }
            SettingsMessage::ToggleBluetooth(enabled) => {
                self.bluetooth_enabled = enabled;
                Task::none()
            }
        }
    }

    pub fn view<'a>(
        &self,
        theme: &peak_core::app_traits::AppTheme,
    ) -> Element<'a, SettingsMessage, iced::Theme, iced::Renderer> {
        let is_light = theme.is_light;
        let (text_color, sidebar_bg, content_bg, accent_color, border_color) = if is_light {
            (
                Color::from_rgb8(35, 30, 30),
                Color::from_rgba8(242, 242, 247, 0.8), // macOS Sidebar Grey
                Color::WHITE,
                Color::from_rgb8(0, 122, 255), // macOS Blue
                Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            )
        } else {
            (
                Color::WHITE,
                Color::from_rgb8(30, 30, 30),
                Color::from_rgb8(20, 20, 20),
                Color::from_rgb8(10, 132, 255),
                Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            )
        };

        // Sidebar
        let sidebar = container(
            column![
                // Search
                container(
                    row![
                        iced::widget::svg(peak_core::icons::get_ui_icon("search", "#999999"))
                            .width(12)
                            .height(12),
                        text_input("Search", &self.search_query)
                            .on_input(SettingsMessage::SearchChanged)
                            .size(13)
                            .padding(0)
                            .style(move |_, _| text_input::Style {
                                background: Color::TRANSPARENT.into(),
                                border: iced::Border::default(),
                                icon: Color::TRANSPARENT,
                                placeholder: Color::from_rgb(0.6, 0.6, 0.6),
                                value: text_color,
                                selection: Color::from_rgba(0.0, 0.5, 1.0, 0.3),
                            })
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center)
                    .padding(6)
                )
                .style(move |_| container::Style {
                    background: Some(if is_light {
                        Color::from_rgba(0.0, 0.0, 0.0, 0.05).into()
                    } else {
                        Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()
                    }),
                    border: iced::Border {
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                vertical_space().height(16),
                // User
                button(
                    row![
                        container(text("VB").size(14).color(Color::WHITE))
                            .width(36)
                            .height(36)
                            .center_x(Length::Fill)
                            .center_y(Length::Fill)
                            .style(|_| container::Style {
                                background: Some(Color::from_rgb8(150, 150, 150).into()),
                                border: iced::Border {
                                    radius: 18.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                        column![
                            text("Vegar Berentsen").size(13).font(iced::Font {
                                weight: iced::font::Weight::Medium,
                                ..Default::default()
                            }),
                            text("Apple ID, iCloud, Media & App Store")
                                .size(11)
                                .color(Color::from_rgb(0.5, 0.5, 0.5)),
                        ]
                        .spacing(2)
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center)
                )
                .padding(4)
                .style(move |_, status| button::Style {
                    background: if status == iced::widget::button::Status::Hovered {
                        Some(Color::from_rgba(0.5, 0.5, 0.5, 0.1).into())
                    } else {
                        None
                    },
                    text_color,
                    ..Default::default()
                }),
                vertical_space().height(16),
                scrollable(column![
                    self.sidebar_group(
                        vec![
                            ("Network", SettingsTab::WiFi, "wifi_full.svg"),
                            ("Bluetooth", SettingsTab::Bluetooth, "cmd.svg"), // Placeholder
                        ],
                        text_color,
                        accent_color,
                        is_light
                    ),
                    vertical_space().height(10),
                    self.sidebar_group(
                        vec![
                            ("General", SettingsTab::General, "settings.svg"),
                            ("Appearance", SettingsTab::Appearance, "sparkles.svg"),
                            ("Display", SettingsTab::Display, "cmd.svg"),
                            ("Sound", SettingsTab::Sound, "robot.svg"),
                            ("Focus", SettingsTab::Focus, "cmd.svg"),
                        ],
                        text_color,
                        accent_color,
                        is_light
                    ),
                    vertical_space().height(10),
                    self.sidebar_group(
                        vec![
                            ("Battery", SettingsTab::Battery, "battery.svg"),
                            ("Privacy & Security", SettingsTab::Privacy, "lock.svg"),
                        ],
                        text_color,
                        accent_color,
                        is_light
                    ),
                ])
            ]
            .padding(16),
        )
        .width(240)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(sidebar_bg.into()),
            border: iced::Border {
                width: 1.0,
                color: border_color,
                ..Default::default()
            },
            ..Default::default()
        });

        // Content Area
        let content = container(scrollable(
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
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(content_bg.into()),
            ..Default::default()
        });

        row![sidebar, content].into()
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
                            .style(move |_| container::Style {
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
                .style(move |_, status| button::Style {
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
                            Color::from_rgba(1.0, 1.0, 1.0, 0.05).into()
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
                        Rule::horizontal(1).style(move |_| iced::widget::rule::Style {
                            color: border_color,
                            width: 1,
                            radius: 0.0.into(),
                            fill_mode: iced::widget::rule::FillMode::Full
                        }),
                        self.labeled_row(
                            "Model",
                            text("Peak Native (Apple Silicon)")
                                .size(13)
                                .color(Color::from_rgb(0.5, 0.5, 0.5))
                        ),
                        Rule::horizontal(1).style(move |_| iced::widget::rule::Style {
                            color: border_color,
                            width: 1,
                            radius: 0.0.into(),
                            fill_mode: iced::widget::rule::FillMode::Full
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

            SettingsTab::Appearance => column![self.section(
                "Appearance",
                column![row![
                    self.theme_preview("Light", ThemeMode::Light, is_light),
                    self.theme_preview("Dark", ThemeMode::Dark, is_light),
                ]
                .spacing(20)],
                is_light
            )]
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
                            Rule::horizontal(1).style(move |_| iced::widget::rule::Style {
                                color: border_color,
                                width: 1,
                                radius: 0.0.into(),
                                fill_mode: iced::widget::rule::FillMode::Full
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
                            Rule::horizontal(1).style(move |_| iced::widget::rule::Style {
                                color: border_color,
                                width: 1,
                                radius: 0.0.into(),
                                fill_mode: iced::widget::rule::FillMode::Full
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
                .style(move |_| container::Style {
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
                        .style(move |_| container::Style {
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
                .style(move |_| container::Style {
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
        .style(|_, _| button::Style::default())
        .into()
    }
}
