// TV Control Center - Apple TV-style overlay panel
// Power, WiFi, AirPods, Sleep Timer, accessibility options

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use peak_theme::ThemeTokens;

#[derive(Debug, Clone, Copy)]
pub enum ControlCenterMessage {
    PowerOff,
    ToggleWifi,
    ToggleAirPods,
    SetSleepTimer,
    ToggleDoNotDisturb,
    ToggleAccessibility,
    Search,
    CloseControlCenter,
}

pub fn view<'a>(tokens: ThemeTokens) -> Element<'a, ControlCenterMessage> {
    let hex_color = "#FFFFFF";

    // Power button (large)
    let power_btn = control_button(
        "Power Off",
        "power",
        hex_color,
        tokens,
        ControlCenterMessage::PowerOff,
        true,
    );

    // WiFi button
    let wifi_btn = control_button(
        "Wi-Fi\nHome",
        "wifi",
        hex_color,
        tokens,
        ControlCenterMessage::ToggleWifi,
        false,
    );

    // Do Not Disturb
    let dnd_btn = control_button(
        "Do Not\nDisturb",
        "moon",
        hex_color,
        tokens,
        ControlCenterMessage::ToggleDoNotDisturb,
        false,
    );

    // AirPods
    let airpods_btn = control_button(
        "Melody's\nAirPods",
        "headphones",
        hex_color,
        tokens,
        ControlCenterMessage::ToggleAirPods,
        false,
    );

    // Sleep Timer
    let sleep_btn = control_button(
        "Sleep\nTimer",
        "clock",
        hex_color,
        tokens,
        ControlCenterMessage::SetSleepTimer,
        false,
    );

    // Grid layout
    let row1 = row![power_btn, wifi_btn].spacing(12);
    let row2 = row![dnd_btn, airpods_btn].spacing(12);
    let row3 = row![sleep_btn].spacing(12);

    // Bottom icons row
    let bottom_icons = row![
        icon_button("gamepad", hex_color, tokens),
        icon_button("accessibility", hex_color, tokens),
        icon_button("person", hex_color, tokens),
        icon_button("search", hex_color, tokens),
    ]
    .spacing(20)
    .align_y(Alignment::Center);

    let panel = column![row1, row2, row3, bottom_icons]
        .spacing(12)
        .align_x(Alignment::Center);

    container(panel)
        .padding(20)
        .style(move |_| container::Style {
            background: Some(iced::Color::from_rgba8(60, 60, 67, 0.95).into()),
            border: iced::Border {
                radius: 20.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

fn control_button<'a>(
    label: &'a str,
    _icon_name: &'a str,
    _hex_color: &'a str,
    tokens: ThemeTokens,
    msg: ControlCenterMessage,
    is_large: bool,
) -> Element<'a, ControlCenterMessage> {
    let size = if is_large { 100.0 } else { 80.0 };

    button(
        column![
            text("○").size(24).style(move |_| text::Style {
                color: Some(tokens.text),
            }),
            text(label).size(11).style(move |_| text::Style {
                color: Some(tokens.text),
            })
        ]
        .spacing(8)
        .align_x(Alignment::Center),
    )
    .on_press(msg)
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .style(move |_, _| iced::widget::button::Style {
        background: Some(iced::Color::from_rgba8(100, 100, 108, 0.8).into()),
        border: iced::Border {
            radius: 16.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn icon_button<'a>(
    _icon_name: &'a str,
    _hex_color: &'a str,
    tokens: ThemeTokens,
) -> Element<'a, ControlCenterMessage> {
    button(text("◎").size(24).style(move |_| text::Style {
        color: Some(tokens.text),
    }))
    .on_press(ControlCenterMessage::CloseControlCenter)
    .padding(12)
    .style(move |_, _| iced::widget::button::Style {
        background: Some(iced::Color::from_rgba8(100, 100, 108, 0.6).into()),
        border: iced::Border {
            radius: 50.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}
