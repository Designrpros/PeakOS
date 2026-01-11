use crate::models::{MediaItem, MediaKind};
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};

#[derive(Debug, Clone)]
pub enum JukeboxMessage {
    PlayTrack(MediaItem),
    TogglePlayback,
    NextTrack,
    PrevTrack,
}

pub struct JukeboxApp {
    pub is_playing: bool,
    pub current_track: Option<MediaItem>,
    pub is_open: bool,
}

impl JukeboxApp {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            current_track: None,
            is_open: false,
        }
    }

    pub fn view<'a>(&self, items: &'a [MediaItem]) -> Element<'a, JukeboxMessage> {
        let music_items = items.iter().filter(|i| i.kind == MediaKind::Music);

        let track_list = column(music_items.map(|item| {
            button(
                row![
                    text(&item.title).size(16).color(Color::WHITE),
                    iced::widget::horizontal_space(),
                    text("PLAY").size(12).color(Color::from_rgb(0.0, 1.0, 0.8)),
                ]
                .padding(10)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .style(button::text)
            .on_press(JukeboxMessage::PlayTrack(item.clone()))
            .into()
        }))
        .spacing(5);

        let controls = row![
            button(text("⏮").size(30))
                .style(button::text)
                .on_press(JukeboxMessage::PrevTrack),
            button(text(if self.is_playing { "⏸" } else { "▶" }).size(40))
                .style(button::text)
                .on_press(JukeboxMessage::TogglePlayback),
            button(text("⏭").size(30))
                .style(button::text)
                .on_press(JukeboxMessage::NextTrack),
        ]
        .spacing(30)
        .align_y(Alignment::Center);

        let visualizer = container(
            text("//// VISUALIZER SIGNAL ACTIVE ////")
                .color(Color::from_rgb(1.0, 0.4, 0.7))
                .size(14),
        )
        .width(Length::Fill)
        .height(Length::Fixed(100.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba8(20, 20, 30, 0.8))),
            border: iced::Border {
                color: Color::from_rgb8(100, 100, 150),
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        });

        let now_playing = if let Some(track) = &self.current_track {
            format!("NOW PLAYING: {}", track.title)
        } else {
            "IDLE // NO SIGNAL".to_string()
        };

        column(vec![
            text(now_playing)
                .size(12)
                .color(Color::from_rgb(0.0, 1.0, 0.8))
                .into(),
            visualizer.into(),
            scrollable(track_list).height(Length::Fill).into(),
            container(controls)
                .width(Length::Fill)
                .center_x(Length::Fill)
                .padding(10)
                .into(),
        ])
        .spacing(15)
        .padding(20)
        .into()
    }
}
