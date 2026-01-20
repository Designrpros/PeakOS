use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Background, Element, Length};

#[derive(Debug, Clone)]
pub enum SpacesMessage {
    SwitchDesktop(usize),
}

pub fn view<'a>(
    tokens: peak_theme::ThemeTokens,
    active_desktop: usize,
) -> Element<'a, SpacesMessage> {
    // Workstation Selection (Desktops)
    let workstation_chip = |idx: usize| {
        let is_active = active_desktop == idx;
        let label = format!("D{}", idx + 1);

        button(text(label).size(10).align_y(iced::Alignment::Center))
            .on_press(SpacesMessage::SwitchDesktop(idx))
            .style(move |_theme, status| {
                let mut bg = if is_active {
                    tokens.colors.primary
                } else {
                    tokens.colors.text_primary
                };

                if !is_active {
                    bg.a = 0.1;
                }

                let mut base = button::Style {
                    background: Some(Background::Color(bg)),
                    text_color: tokens.colors.text_primary,
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                if status == button::Status::Hovered {
                    let mut hover_bg = if is_active {
                        tokens.colors.primary
                    } else {
                        tokens.colors.text_primary
                    };
                    hover_bg.a = 0.8;
                    base.background = Some(Background::Color(hover_bg));
                }
                base
            })
            .width(Length::Fixed(35.0))
            .height(Length::Fixed(25.0))
    };

    let workstations = row![
        workstation_chip(0),
        workstation_chip(1),
        workstation_chip(2),
        workstation_chip(3),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let mut label_color = tokens.colors.text_primary;
    label_color.a = 0.4;

    container(
        column![
            text("DESKTOPS")
                .size(10)
                .font(iced::Font::MONOSPACE)
                .color(label_color),
            workstations
        ]
        .spacing(10)
        .align_x(Alignment::Center),
    )
    .padding(15)
    .style(move |_| container::Style {
        background: Some(Background::Color({
            let mut c = tokens.colors.surface;
            c.a = tokens.glass_opacity;
            c
        })),
        border: iced::Border {
            color: tokens.colors.border,
            width: 1.0,
            radius: tokens.radius.into(),
        },
        shadow: iced::Shadow {
            color: tokens.shadow_color,
            offset: iced::Vector::new(0.0, 10.0),
            blur_radius: 20.0,
        },
        ..Default::default()
    })
    .into()
}
