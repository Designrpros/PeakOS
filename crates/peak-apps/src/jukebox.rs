use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input, vertical_space,
};
use iced::{Alignment, Color, Element, Length, Task};
use peak_core::models::{MediaItem, MediaKind};
use peak_core::styles::WAVEFORM_PINK;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Discovery,
    Player,
}

#[derive(Debug, Clone)]
pub enum JukeboxMessage {
    PlayTrack(MediaItem),
    TogglePlayback,
    NextTrack,
    PrevTrack,
    SwitchView(ViewMode),
    Search(String),
    #[allow(dead_code)]
    ToggleLike(std::path::PathBuf),
}

pub struct JukeboxApp {
    pub is_playing: bool,
    pub current_track: Option<MediaItem>,

    pub view_mode: ViewMode,
    pub search_query: String,
    pub library: Vec<MediaItem>,
}

impl JukeboxApp {
    pub fn new(library: Vec<MediaItem>) -> Self {
        Self {
            is_playing: false,
            current_track: None,

            view_mode: ViewMode::Player,
            search_query: String::new(),
            library,
        }
    }

    pub fn update(&mut self, message: JukeboxMessage) -> Option<JukeboxMessage> {
        match message {
            JukeboxMessage::PlayTrack(track) => {
                self.current_track = Some(track);
                self.is_playing = true;
                self.view_mode = ViewMode::Player;
            }
            JukeboxMessage::TogglePlayback => {
                self.is_playing = !self.is_playing;
            }
            JukeboxMessage::NextTrack => {}
            JukeboxMessage::PrevTrack => {}
            JukeboxMessage::SwitchView(mode) => {
                self.view_mode = mode;
            }
            JukeboxMessage::Search(query) => {
                self.search_query = query;
            }
            JukeboxMessage::ToggleLike(_) => {}
        }
        None
    }

    pub fn view<'a>(&'a self, theme: &peak_core::theme::Theme) -> Element<'a, JukeboxMessage> {
        self.view_internal(&self.library, *theme == peak_core::theme::Theme::Light)
    }

    fn view_internal<'a>(
        &'a self,
        items: &'a [MediaItem],
        _is_light: bool,
    ) -> Element<'a, JukeboxMessage> {
        let music_items: Vec<_> = items
            .iter()
            .filter(|i| i.kind == MediaKind::Music)
            .filter(|i| {
                self.search_query.is_empty()
                    || i.title
                        .to_lowercase()
                        .contains(&self.search_query.to_lowercase())
            })
            .collect();

        // LEFT SIDE: PLAYER
        let track_info = if let Some(track) = &self.current_track {
            column![
                text(&track.title).size(20),
                text("Unknown Artist")
                    .size(14)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
            ]
        } else {
            column![
                text("No Track").size(20),
                text("Select a song")
                    .size(14)
                    .color(Color::from_rgb(0.5, 0.5, 0.5)),
            ]
        }
        .align_x(Alignment::Center)
        .spacing(5);

        // Album art with smaller size
        let album_art = container(
            column![
                text("ALBUM")
                    .size(16)
                    .color(Color::from_rgba(1.0, 1.0, 1.0, 0.4)),
                text("ART")
                    .size(16)
                    .color(Color::from_rgba(1.0, 1.0, 1.0, 0.4)),
            ]
            .align_x(Alignment::Center)
            .spacing(2),
        )
        .width(180)
        .height(180)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_| container::Style {
            background: Some(WAVEFORM_PINK.into()),
            border: iced::Border {
                radius: 20.0.into(),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 8.0),
                blur_radius: 20.0,
            },
            ..Default::default()
        });

        // Waveform visualization
        let waveform = container(
            row![
                container(iced::widget::Space::new(3, 30)).style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.3).into()),
                    ..Default::default()
                }),
                container(iced::widget::Space::new(3, 45)).style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.5).into()),
                    ..Default::default()
                }),
                container(iced::widget::Space::new(3, 25)).style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.3).into()),
                    ..Default::default()
                }),
                container(iced::widget::Space::new(3, 50)).style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.6).into()),
                    ..Default::default()
                }),
                container(iced::widget::Space::new(3, 35)).style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.4).into()),
                    ..Default::default()
                }),
                container(iced::widget::Space::new(3, 42)).style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.5).into()),
                    ..Default::default()
                }),
                container(iced::widget::Space::new(3, 28)).style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.3).into()),
                    ..Default::default()
                }),
            ]
            .spacing(4)
            .align_y(Alignment::End),
        )
        .width(Length::Fill)
        .height(50)
        .center_x(Length::Fill)
        .padding(10)
        .style(|_| container::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.1).into()),
            border: iced::Border {
                radius: 8.0.into(),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            ..Default::default()
        });

        // Enhanced playback controls
        let controls = row![
            button(text("<<").size(20))
                .style(button::text)
                .on_press(JukeboxMessage::PrevTrack),
            button(text(if self.is_playing { "||" } else { ">" }).size(32))
                .style(button::text)
                .on_press(JukeboxMessage::TogglePlayback),
            button(text(">>").size(20))
                .style(button::text)
                .on_press(JukeboxMessage::NextTrack),
        ]
        .spacing(25)
        .align_y(Alignment::Center);

        let player_section = container(
            column![
                vertical_space().height(20),
                album_art,
                vertical_space().height(20),
                track_info,
                vertical_space().height(15),
                waveform,
                vertical_space().height(15),
                controls,
            ]
            .align_x(Alignment::Center),
        )
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(20);

        // RIGHT SIDE: MUSIC LIBRARY
        let search_bar = text_input("Search...", &self.search_query)
            .on_input(JukeboxMessage::Search)
            .padding(10)
            .width(Length::Fill);

        let music_list = if music_items.is_empty() {
            container(
                column![
                    vertical_space().height(40),
                    text("No Music Found")
                        .size(16)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    text(format!(
                        "{} items | {} music",
                        items.len(),
                        music_items.len()
                    ))
                    .size(11)
                    .color(Color::from_rgb(0.4, 0.4, 0.4)),
                ]
                .align_x(Alignment::Center)
                .spacing(8),
            )
            .center_y(Length::Fill)
        } else {
            container(scrollable(
                column(music_items.iter().map(|item| {
                    button(
                        row![
                            container(text("♪").size(16))
                                .width(35)
                                .height(35)
                                .center_x(Length::Fill)
                                .center_y(Length::Fill)
                                .style(|_| container::Style {
                                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.05).into()),
                                    border: iced::Border {
                                        radius: 6.0.into(),
                                        width: 0.0,
                                        color: iced::Color::TRANSPARENT,
                                    },
                                    ..Default::default()
                                }),
                            column![
                                text(&item.title).size(14),
                                text("Unknown Artist")
                                    .size(11)
                                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
                            ]
                            .spacing(2),
                            horizontal_space(),
                            text(">").size(14).color(Color::from_rgb(0.6, 0.6, 0.6)),
                        ]
                        .spacing(12)
                        .padding(10)
                        .align_y(Alignment::Center),
                    )
                    .width(Length::Fill)
                    .style(button::text)
                    .on_press(JukeboxMessage::PlayTrack((*item).clone()))
                    .into()
                }))
                .spacing(3)
                .height(Length::Shrink),
            ))
        };

        let library_section =
            container(column![search_bar, vertical_space().height(10), music_list,])
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .padding(20);

        // BOTTOM: CENTERED NAVIGATION DOTS
        let nav_dots = row![
            horizontal_space(),
            button(
                text("•")
                    .size(14)
                    .color(if self.view_mode == ViewMode::Player {
                        Color::from_rgb(0.2, 0.2, 0.2)
                    } else {
                        Color::from_rgb(0.7, 0.7, 0.7)
                    })
            )
            .style(button::text)
            .on_press(JukeboxMessage::SwitchView(ViewMode::Player)),
            button(
                text("•")
                    .size(14)
                    .color(if self.view_mode == ViewMode::Discovery {
                        Color::from_rgb(0.2, 0.2, 0.2)
                    } else {
                        Color::from_rgb(0.7, 0.7, 0.7)
                    })
            )
            .style(button::text)
            .on_press(JukeboxMessage::SwitchView(ViewMode::Discovery)),
            horizontal_space(),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // MAIN LAYOUT
        container(column![
            row![player_section, library_section,].height(Length::Fill),
            container(nav_dots)
                .width(Length::Fill)
                .center_x(Length::Fill)
                .padding([10, 0]),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

use peak_core::app_traits::{PeakApp, ShellContext};

impl PeakApp for JukeboxApp {
    type Message = JukeboxMessage;

    fn title(&self) -> String {
        String::from("Jukebox")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        if let Some(msg) = self.update(message) {
            Task::done(msg)
        } else {
            Task::none()
        }
    }

    fn view(&self, theme: &peak_core::theme::Theme) -> Element<'_, Self::Message> {
        self.view(theme)
    }
}
