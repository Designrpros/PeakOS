use iced::widget::{
    button, column, container, radio, row, scrollable, slider, svg as svg_widget, text, Rule,
    Space, TextInput,
};
use iced::{Alignment, Color, Element, Length, Padding};

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
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ThemeChanged(ThemeMode),
    VolumeChanged(f32),
    TabChanged(SettingsTab),
    SearchChanged(String),
}

pub struct SettingsApp {
    pub theme_mode: ThemeMode,
    pub current_tab: SettingsTab,
    pub volume: f32,
    pub search_query: String,
}

impl SettingsApp {
    pub fn new() -> Self {
        Self {
            theme_mode: ThemeMode::Light,
            current_tab: SettingsTab::Appearance,
            volume: 0.8,
            search_query: String::new(),
        }
    }

    pub fn view<'a>(&self, is_light: bool) -> Element<'a, SettingsMessage> {
        let (text_color, sidebar_bg, content_bg, accent_color, _border_color) = if is_light {
            (
                Color::from_rgb8(35, 30, 30),
                Color::from_rgba8(240, 238, 235, 0.5),
                Color::from_rgb8(255, 255, 255),
                Color::from_rgb8(0, 122, 255), // macOS Blue
                Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            )
        } else {
            (
                Color::from_rgb8(235, 230, 225),
                Color::from_rgba8(30, 28, 28, 0.5),
                Color::from_rgb8(22, 21, 21),
                Color::from_rgb8(10, 132, 255), // macOS Blue Dark
                Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            )
        };

        let icon_theme = if is_light { "black" } else { "white" };

        let sidebar_item = |label: &'static str, tab: SettingsTab, icon_name: &'static str| {
            let is_active = self.current_tab == tab;
            button(
                row![
                    container(
                        svg_widget(iced::widget::svg::Handle::from_path(
                            crate::utils::assets::get_asset_path(&format!(
                                "icons/menubar/{}/{}",
                                icon_theme, icon_name
                            ))
                        ))
                        .width(Length::Fixed(14.0))
                        .height(Length::Fixed(14.0))
                    )
                    .width(18)
                    .height(18)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .style(move |_| container::Style {
                        background: if is_active {
                            Some(accent_color.into())
                        } else {
                            Some(Color::from_rgba(0.5, 0.5, 0.5, 0.1).into())
                        },
                        border: iced::Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    text(label).size(12),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .on_press(SettingsMessage::TabChanged(tab))
            .width(Length::Fill)
            .padding(Padding::from([4, 8]))
            .style(move |_, status| button::Style {
                background: if is_active {
                    Some(accent_color.into())
                } else if status == iced::widget::button::Status::Hovered {
                    Some(Color::from_rgba(0.5, 0.5, 0.5, 0.1).into())
                } else {
                    None
                },
                text_color: if is_active { Color::WHITE } else { text_color },
                border: iced::Border {
                    radius: 5.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
        };

        // Sidebar
        let sidebar = container(
            column![
                TextInput::new("Søk", &self.search_query)
                    .on_input(SettingsMessage::SearchChanged)
                    .size(11)
                    .padding(4),
                Space::with_height(10),
                // User Profile Placeholder
                row![
                    container(text("VB").size(10).color(Color::WHITE))
                        .width(32)
                        .height(32)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill)
                        .style(|_| container::Style {
                            background: Some(Color::from_rgb8(180, 180, 180).into()),
                            border: iced::Border {
                                radius: 16.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    column![
                        text("Vegar Berentsen").size(12).color(text_color),
                        text("Apple-konto")
                            .size(10)
                            .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    ]
                    .spacing(2)
                ]
                .spacing(10)
                .align_y(Alignment::Center),
                Space::with_height(15),
                scrollable(
                    column![
                        sidebar_item("Wi-Fi", SettingsTab::WiFi, "wifi_full.svg"),
                        sidebar_item("Bluetooth", SettingsTab::Bluetooth, "cmd.svg"), // Placeholder icon
                        sidebar_item("Batteri", SettingsTab::Battery, "battery.svg"),
                        Space::with_height(10),
                        sidebar_item("Generelt", SettingsTab::General, "settings.svg"),
                        sidebar_item("Utseende", SettingsTab::Appearance, "sparkles.svg"),
                        sidebar_item("Skjermer", SettingsTab::Display, "cmd.svg"),
                        sidebar_item("Lyd", SettingsTab::Sound, "robot.svg"),
                    ]
                    .spacing(1)
                )
            ]
            .padding(10),
        )
        .width(Length::Fixed(180.0))
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(sidebar_bg.into()),
            ..Default::default()
        });

        // Content
        let content = container(
            column![
                text(format!("{:?}", self.current_tab))
                    .size(18)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
                Space::with_height(15),
                match self.current_tab {
                    SettingsTab::Appearance => {
                        column![self.setting_card(
                            "Theme Mode",
                            row![
                                radio(
                                    "Poolside Day",
                                    ThemeMode::Light,
                                    Some(self.theme_mode),
                                    SettingsMessage::ThemeChanged
                                ),
                                radio(
                                    "Poolside Night",
                                    ThemeMode::Dark,
                                    Some(self.theme_mode),
                                    SettingsMessage::ThemeChanged
                                ),
                            ]
                            .spacing(20),
                            is_light
                        ),]
                        .spacing(10)
                    }
                    SettingsTab::Sound => {
                        column![
                            self.setting_card(
                                "Volume",
                                column![
                                    row![
                                        text("0%").size(10),
                                        slider(
                                            0.0..=1.0,
                                            self.volume,
                                            SettingsMessage::VolumeChanged
                                        )
                                        .width(Length::Fill),
                                        text("100%").size(10),
                                    ]
                                    .spacing(10)
                                    .align_y(Alignment::Center),
                                    text(format!("{:.0}%", self.volume * 100.0))
                                        .size(11)
                                        .color(accent_color),
                                ]
                                .spacing(5),
                                is_light
                            ),
                            self.setting_card(
                                "Output Device",
                                text("Default Peak Speakers").size(12),
                                is_light
                            ),
                        ]
                        .spacing(10)
                    }
                    _ => column![text("Denne seksjonen er under arbeid.")
                        .size(12)
                        .color(Color::from_rgb(0.5, 0.5, 0.5))]
                    .into(),
                }
            ]
            .padding(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(content_bg.into()),
            ..Default::default()
        });

        row![sidebar, Rule::vertical(1), content].into()
    }

    fn setting_card<'a, Message: 'a>(
        &self,
        label: &'static str,
        content: impl Into<Element<'a, Message>>,
        is_light: bool,
    ) -> Element<'a, Message> {
        let card_bg = if is_light {
            Color::from_rgba(0.0, 0.0, 0.0, 0.03)
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.03)
        };

        container(column![
            text(label).size(12).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            Space::with_height(5),
            container(content).width(Length::Fill)
        ])
        .padding(10)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(card_bg.into()),
            border: iced::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::ThemeChanged(mode) => self.theme_mode = mode,
            SettingsMessage::VolumeChanged(v) => self.volume = v,
            SettingsMessage::TabChanged(tab) => self.current_tab = tab,
            SettingsMessage::SearchChanged(q) => self.search_query = q,
        }
    }
}
