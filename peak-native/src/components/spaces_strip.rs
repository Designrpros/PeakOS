use crate::app::ShellMode;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Background, Color, Element, Length};

#[derive(Debug, Clone)]
pub enum SpacesMessage {
    SwitchTo(ShellMode),
    SwitchDesktop(usize),
}

pub fn view<'a>(active_mode: ShellMode, active_desktop: usize) -> Element<'a, SpacesMessage> {
    // 1. Reality Selection (Spaces)
    let reality_chip = |label: &'static str, mode: ShellMode, icon: &'static str| {
        let is_active = active_mode == mode;

        let content = column![text(icon).size(20), text(label).size(12)]
            .align_x(Alignment::Center)
            .spacing(5);

        button(content)
            .on_press(SpacesMessage::SwitchTo(mode))
            .style(move |_theme, status| {
                let base = button::Style {
                    background: Some(Background::Color(if is_active {
                        Color::from_rgba(1.0, 1.0, 1.0, 0.15)
                    } else {
                        Color::TRANSPARENT
                    })),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        radius: 8.0.into(),
                        color: if is_active {
                            Color::WHITE
                        } else {
                            Color::TRANSPARENT
                        },
                        width: if is_active { 1.0 } else { 0.0 },
                    },
                    ..Default::default()
                };
                match status {
                    button::Status::Hovered => button::Style {
                        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.25))),
                        ..base
                    },
                    _ => base,
                }
            })
            .padding(10)
            .width(Length::Fixed(100.0))
    };

    // 2. Workstation Selection (Desktops)
    let workstation_chip = |idx: usize| {
        let is_active = active_desktop == idx;
        let label = format!("D{}", idx + 1);

        button(text(label).size(10).align_y(Alignment::Center))
            .on_press(SpacesMessage::SwitchDesktop(idx))
            .style(move |_theme, status| {
                let base = button::Style {
                    background: Some(Background::Color(if is_active {
                        Color::from_rgb(0.0, 0.8, 1.0)
                    } else {
                        Color::from_rgba(1.0, 1.0, 1.0, 0.1)
                    })),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                match status {
                    button::Status::Hovered => button::Style {
                        background: Some(Background::Color(Color::from_rgba(0.0, 0.8, 1.0, 0.8))),
                        ..base
                    },
                    _ => base,
                }
            })
            .width(Length::Fixed(35.0))
            .height(Length::Fixed(25.0))
    };

    let realities = row![
        reality_chip("PEAK", ShellMode::Peak, "🏔️"),
        reality_chip("RIVIERA", ShellMode::Poolside, "🌴"),
    ]
    .spacing(15)
    .align_y(Alignment::Center);

    let workstations = row![
        workstation_chip(0),
        workstation_chip(1),
        workstation_chip(2),
        workstation_chip(3),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    container(
        column![
            realities,
            iced::widget::Rule::horizontal(1).style(|_| iced::widget::rule::Style {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                width: 1,
                radius: 0.0.into(),
                fill_mode: iced::widget::rule::FillMode::Full
            }),
            workstations
        ]
        .spacing(15)
        .align_x(Alignment::Center),
    )
    .width(Length::Fixed(280.0))
    .padding(15)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgba(0.03, 0.03, 0.05, 0.9))),
        border: iced::Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: iced::Shadow {
            color: Color::BLACK,
            offset: iced::Vector::new(0.0, 10.0),
            blur_radius: 20.0,
        },
        ..Default::default()
    })
    .into()
}
