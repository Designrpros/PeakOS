// Console Game Rail - PS5-style horizontal game carousel
// Shows game covers in a horizontal scrollable row with dynamic background

use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length};
use peak_core::models::MediaItem;
use peak_theme::ThemeTokens;

#[derive(Debug, Clone)]
pub enum GameRailMessage {
    SelectGame(String),  // Game ID or command
    LaunchGame(String),  // Launch selected game
    ShowDetails(String), // Show game details overlay
    ScrollLeft,
    ScrollRight,
}

pub fn view<'a>(
    games: &'a [MediaItem],
    selected_index: usize,
    tokens: ThemeTokens,
) -> Element<'a, GameRailMessage> {
    let mut game_row = row![].spacing(16).align_y(Alignment::End);

    for (i, game) in games.iter().take(8).enumerate() {
        let is_selected = i == selected_index;
        let scale = if is_selected { 1.2 } else { 1.0 };
        let width = 150.0 * scale;
        let height = 200.0 * scale;

        let cover: Element<GameRailMessage> = if std::path::Path::new(&game.cover_image).exists() {
            iced::widget::image::<iced::widget::image::Handle>(
                iced::widget::image::Handle::from_path(&game.cover_image),
            )
            .width(Length::Fixed(width as f32))
            .height(Length::Fixed(height as f32))
            .into()
        } else {
            // Placeholder for games without local cover
            container(
                text(&game.title)
                    .size(14)
                    .style(move |_| iced::widget::text::Style {
                        color: Some(tokens.colors.text_primary),
                    }),
            )
            .width(Length::Fixed(width as f32))
            .height(Length::Fixed(height as f32))
            .center_x(width as f32)
            .center_y(height as f32)
            .style(move |_| container::Style {
                background: Some(tokens.colors.surface.into()),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        };

        let game_cmd = game.launch_command.clone();
        let game_card: Element<GameRailMessage> = button(cover)
            .on_press(GameRailMessage::SelectGame(game_cmd))
            .padding(0)
            .style(move |_, status| {
                let border_color = if is_selected {
                    tokens.colors.primary
                } else {
                    iced::Color::TRANSPARENT
                };

                iced::widget::button::Style {
                    background: None,
                    border: iced::Border {
                        color: border_color,
                        width: if is_selected { 3.0 } else { 0.0 },
                        radius: 12.0.into(),
                    },
                    shadow: if status == iced::widget::button::Status::Hovered {
                        iced::Shadow {
                            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                            offset: iced::Vector::new(0.0, 8.0),
                            blur_radius: 16.0,
                        }
                    } else {
                        iced::Shadow::default()
                    },
                    ..Default::default()
                }
            })
            .into();

        game_row = game_row.push(game_card);
    }

    container(game_row)
        .padding([0, 40])
        .width(Length::Fill)
        .into()
}
