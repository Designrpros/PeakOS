// Redmond Taskbar - Windows 10-style bottom taskbar
// Left-aligned: Start button, search, pinned apps
// Right-aligned: System tray (clock, wifi, battery)

use iced::widget::{button, container, horizontal_space, row, svg, text};
use iced::{Alignment, Element, Length};
use peak_core::registry::AppId;
use peak_theme::ThemeTokens;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskbarMessage {
    OpenStart,
    Search,
    LaunchApp(AppId),
    ShowSystemTray,
    ToggleNotifications,
}

pub fn view<'a>(
    pinned_apps: &[AppId],
    running_apps: &[AppId],
    tokens: ThemeTokens,
) -> Element<'a, TaskbarMessage> {
    let hex_color = format!(
        "#{:02x}{:02x}{:02x}",
        (tokens.text.r * 255.0) as u8,
        (tokens.text.g * 255.0) as u8,
        (tokens.text.b * 255.0) as u8
    );

    // Left section: Start button + Search
    // START BUTTON: Use Peak Logo (PNG)
    // Logic: If text is light (Dark Mode), use the 'Dark Mode' logo (which is usually white/light to contrast)
    // If text is dark (Light Mode), use the 'Standard' logo (which is usually dark/colored)
    let logo_path = if tokens.text.r > 0.5 {
        "icons/menubar/peak_logo_dark.png"
    } else {
        "icons/menubar/peak_logo.png"
    };

    let logo_abs_path = peak_core::utils::assets::get_asset_path(logo_path);

    let start_btn = button(
        iced::widget::image(logo_abs_path)
            .width(Length::Fixed(24.0))
            .height(Length::Fixed(24.0)),
    )
    .on_press(TaskbarMessage::OpenStart)
    .padding(8)
    .style(move |_, status| {
        let hover_bg = tokens.text;
        let mut hover_bg = hover_bg;
        hover_bg.a = 0.1;

        iced::widget::button::Style {
            background: if status == iced::widget::button::Status::Hovered {
                Some(hover_bg.into())
            } else {
                None
            },
            ..Default::default()
        }
    });

    // SEARCH BUTTON: Icon only
    let search_btn = button(
        svg(peak_core::icons::get_ui_icon("search", &hex_color))
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0)),
    )
    .on_press(TaskbarMessage::Search)
    .padding(10)
    .style(move |_, status| {
        let hover_bg = tokens.text;
        let mut hover_bg = hover_bg;
        hover_bg.a = 0.1;

        iced::widget::button::Style {
            background: if status == iced::widget::button::Status::Hovered {
                Some(hover_bg.into())
            } else {
                None
            },
            ..Default::default()
        }
    });

    // Center section: Pinned + Running apps
    let mut app_row = row![].spacing(2).align_y(Alignment::Center);

    for &app_id in pinned_apps {
        let is_running = running_apps.contains(&app_id);
        app_row = app_row.push(render_taskbar_icon(app_id, is_running, tokens, &hex_color));
    }

    // Add running apps that aren't pinned
    for &app_id in running_apps {
        if !pinned_apps.contains(&app_id) {
            app_row = app_row.push(render_taskbar_icon(app_id, true, tokens, &hex_color));
        }
    }

    // Right section: System tray
    let time = chrono::Local::now().format("%H:%M").to_string();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();

    let system_tray = row![
        svg(peak_core::icons::get_status_icon("wifi", &hex_color))
            .width(Length::Fixed(16.0))
            .height(Length::Fixed(16.0)),
        svg(peak_core::icons::get_status_icon("volume", &hex_color))
            .width(Length::Fixed(16.0))
            .height(Length::Fixed(16.0)),
        text(format!("{}\n{}", time, date))
            .size(11)
            .style(move |_| text::Style {
                color: Some(tokens.text),
            }),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    // Assemble taskbar
    container(
        row![
            start_btn,
            search_btn,
            app_row,
            horizontal_space().width(Length::Fill),
            system_tray,
        ]
        .spacing(8)
        .padding([0, 10])
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(40)
    .style(move |_| container::Style {
        background: Some(tokens.glass_bg.into()), // Dynamic Theme Background
        ..Default::default()
    })
    .into()
}

fn render_taskbar_icon<'a>(
    app_id: AppId,
    is_running: bool,
    tokens: ThemeTokens,
    hex_color: &str,
) -> Element<'a, TaskbarMessage> {
    let icon: Element<TaskbarMessage> =
        match peak_core::icons::IconResolver::resolve_app_icon(app_id, hex_color) {
            peak_core::icons::AppIcon::Svg(handle) => iced::widget::svg(handle)
                .width(Length::Fixed(24.0))
                .height(Length::Fixed(24.0))
                .into(),
            peak_core::icons::AppIcon::Image(handle) => iced::widget::image(handle)
                .width(Length::Fixed(24.0))
                .height(Length::Fixed(24.0))
                .into(),
        };

    let indicator_color = if is_running {
        tokens.accent
    } else {
        iced::Color::TRANSPARENT
    };

    container(
        iced::widget::column![
            button(icon)
                .on_press(TaskbarMessage::LaunchApp(app_id))
                .padding(8)
                .style(move |_, status| {
                    let hover_bg = tokens.text;
                    let mut hover_bg = hover_bg;
                    hover_bg.a = 0.15;

                    iced::widget::button::Style {
                        background: if status == iced::widget::button::Status::Hovered {
                            Some(hover_bg.into())
                        } else {
                            None
                        },
                        ..Default::default()
                    }
                }),
            container(iced::widget::Space::with_width(Length::Fixed(20.0)))
                .height(Length::Fixed(3.0))
                .style(move |_| container::Style {
                    background: Some(indicator_color.into()),
                    ..Default::default()
                }),
        ]
        .align_x(Alignment::Center),
    )
    .into()
}
