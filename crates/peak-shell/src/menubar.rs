use chrono::Local;
use iced::widget::{button, container, horizontal_space, row, svg, text};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenubarMessage {
    ToggleSettings,
    ToggleOmnibar,
    ToggleSpaces,
    ToggleRealityMenu,
    ToggleSystemMenu,
    ToggleWifiMenu,
    ToggleInspector,
}

use peak_theme::ThemeTokens;

pub fn view<'a>(tokens: ThemeTokens) -> Element<'a, MenubarMessage> {
    // 1. The Clock (Real-time)
    let time = Local::now().format("%H:%M").to_string();
    let date = Local::now().format("%a %b %d").to_string();

    let text_color = tokens.text;
    let bg_color = tokens.glass_bg;

    let hex_color = format!(
        "#{:02x}{:02x}{:02x}",
        (tokens.text.r * 255.0) as u8,
        (tokens.text.g * 255.0) as u8,
        (tokens.text.b * 255.0) as u8
    );

    let switcher = button(text("Peak").size(13)) // Placeholder for mode string, can be passed if needed
        .on_press(MenubarMessage::ToggleRealityMenu)
        .padding([5, 0])
        .style(move |_, _| button::Style {
            text_color,
            ..Default::default()
        });

    let logo_file = if tokens.background.r < 0.2 {
        "peak_logo_dark.png"
    } else {
        "peak_logo.png"
    };

    // 4. The Left Menu (System & App)
    let left_menu = row![
        button(
            iced::widget::image(iced::widget::image::Handle::from_path(
                peak_core::utils::assets::get_asset_path(&format!("icons/menubar/{}", logo_file))
            ))
            .width(Length::Fixed(36.0))
            .height(Length::Fixed(18.0)), // Peak Logo
        )
        .on_press(MenubarMessage::ToggleSystemMenu)
        .style(button::text)
        .padding(0),
        switcher, // The Dropdown
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    // --- Right Menu Icons ---

    // 5. The Right Menu (Status)
    let right_menu = row![
        // Search
        button(
            svg(peak_core::icons::get_ui_icon("search", &hex_color))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0)),
        )
        .on_press(MenubarMessage::ToggleOmnibar)
        .padding(0)
        .style(button::text),
        // WiFi (Full)
        button(
            svg(peak_core::icons::get_status_icon("wifi", &hex_color))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0)),
        )
        .on_press(MenubarMessage::ToggleWifiMenu)
        .padding(0)
        .style(button::text),
        // Robot (AI)
        button(
            svg(peak_core::icons::get_avatar_handle("robot", &hex_color))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0)),
        )
        .on_press(MenubarMessage::ToggleInspector)
        .padding(0)
        .style(button::text),
        // Settings
        button(
            svg(peak_core::icons::get_ui_icon("settings", &hex_color))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0))
        )
        .on_press(MenubarMessage::ToggleSettings)
        .padding(0)
        .style(button::text),
        // Command
        button(
            svg(peak_core::icons::get_ui_icon("cmd", &hex_color))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0))
        )
        .on_press(MenubarMessage::ToggleSpaces)
        .padding(0)
        .style(button::text),
        // Battery (Text Only)
        text("100%").size(13).style(move |_| text::Style {
            color: Some(text_color),
            ..Default::default()
        }),
        // Clock
        text(format!("{}  {}", date, time))
            .size(13)
            .style(move |_| text::Style {
                color: Some(text_color),
                ..Default::default()
            }),
    ]
    .spacing(15)
    .align_y(Alignment::Center);

    // 6. Assemble the Bar
    container(
        row![
            left_menu,
            horizontal_space().width(Length::Fill), // Push apart
            right_menu
        ]
        .padding([0, 15])
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(32) // Slightly taller for the button feel (User suggested 32 in prompt, original was 24)
    .center_y(32) // Explicitly center within the height
    .style(move |_| container::Style {
        background: Some(bg_color.into()),
        ..Default::default()
    })
    .into()
}
