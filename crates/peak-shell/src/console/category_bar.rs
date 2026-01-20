// Console Category Bar - PS5-style category filter tabs
// Horizontal row of category tabs at top of screen

use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length};
use peak_theme::ThemeTokens;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCategory {
    All,
    Racing,
    Roleplaying,
    Sports,
    Online,
    Shooter,
    Adventure,
    Indie,
}

impl GameCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            GameCategory::All => "ALL",
            GameCategory::Racing => "RACING",
            GameCategory::Roleplaying => "ROLEPLAYING",
            GameCategory::Sports => "SPORTS",
            GameCategory::Online => "ONLINE",
            GameCategory::Shooter => "SHOOTER",
            GameCategory::Adventure => "ADVENTURE",
            GameCategory::Indie => "INDIE",
        }
    }
}

#[derive(Debug, Clone)]
pub enum CategoryBarMessage {
    SelectCategory(GameCategory),
}

pub fn view<'a>(selected: GameCategory, tokens: ThemeTokens) -> Element<'a, CategoryBarMessage> {
    let categories = [
        GameCategory::All,
        GameCategory::Racing,
        GameCategory::Roleplaying,
        GameCategory::Sports,
        GameCategory::Online,
        GameCategory::Shooter,
    ];

    let mut cat_row = row![].spacing(24).align_y(Alignment::Center);

    for cat in categories {
        let is_selected = cat == selected;
        let label = cat.display_name();

        let cat_btn = button(text(label).size(if is_selected { 16 } else { 14 }).style(
            move |_| iced::widget::text::Style {
                color: if is_selected {
                    Some(tokens.colors.text_primary)
                } else {
                    Some(iced::Color {
                        a: 0.6,
                        ..tokens.colors.text_primary
                    })
                },
            },
        ))
        .on_press(CategoryBarMessage::SelectCategory(cat))
        .padding([6, 12])
        .style(move |_, _| {
            let bg_alpha = if is_selected { 0.2 } else { 0.0 };
            iced::widget::button::Style {
                background: Some(
                    iced::Color {
                        a: bg_alpha,
                        ..tokens.colors.text_primary
                    }
                    .into(),
                ),
                border: iced::Border {
                    radius: 16.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        });

        cat_row = cat_row.push(cat_btn);
    }

    container(cat_row)
        .padding([0, 40])
        .width(Length::Fill)
        .into()
}
