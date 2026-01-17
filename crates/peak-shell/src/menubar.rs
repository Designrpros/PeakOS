use chrono::Local;
use iced::widget::{button, container, horizontal_space, row, svg, text};
use iced::{Alignment, Background, Color, Element, Length};
use peak_core::registry::ShellMode;

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

pub fn view<'a>(mode: ShellMode, is_light: bool) -> Element<'a, MenubarMessage> {
    // 1. The Clock (Real-time)
    let time = Local::now().format("%H:%M").to_string();
    let date = Local::now().format("%a %b %d").to_string();

    // 2. The Styling Logic
    let (text_color, bg_color) = match (mode, is_light) {
        (ShellMode::Peak, true) => (
            Color::from_rgb8(35, 30, 30),
            Color::from_rgb8(247, 245, 242),
        ),
        (ShellMode::Peak, false) => (
            Color::from_rgb8(235, 230, 225),
            Color::from_rgb8(15, 14, 14),
        ),
        (ShellMode::Poolside, _) => (
            Color::from_rgb8(50, 50, 50),
            Color::from_rgb8(255, 153, 204),
        ),
    };

    let icon_color_hex = if is_light { "#000000" } else { "#FFFFFF" };

    let switcher = button(text(mode.to_string()).size(13))
        .on_press(MenubarMessage::ToggleRealityMenu)
        .padding([5, 0])
        .style(move |_, _| button::Style {
            text_color,
            ..Default::default()
        });

    let logo_file = if is_light {
        "peak_logo.png"
    } else {
        "peak_logo_dark.png"
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
            svg(peak_core::icons::get_ui_icon("search", icon_color_hex))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0)),
        )
        .on_press(MenubarMessage::ToggleOmnibar)
        .padding(0)
        .style(button::text),
        // WiFi (Full)
        button(
            svg(peak_core::icons::get_status_icon("wifi", icon_color_hex))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0)),
        )
        .on_press(MenubarMessage::ToggleWifiMenu)
        .padding(0)
        .style(button::text),
        // Robot (AI)
        button(
            svg(peak_core::icons::get_avatar_handle("robot", icon_color_hex))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0)),
        )
        .on_press(MenubarMessage::ToggleInspector)
        .padding(0)
        .style(button::text),
        // Settings
        button(
            svg(peak_core::icons::get_ui_icon("settings", icon_color_hex))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0))
        )
        .on_press(MenubarMessage::ToggleSettings)
        .padding(0)
        .style(button::text),
        // Command
        button(
            svg(peak_core::icons::get_ui_icon("cmd", icon_color_hex))
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
        background: Some(Background::Color(bg_color)),
        ..Default::default()
    })
    .into()
}
