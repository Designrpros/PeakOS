use crate::app::ShellMode;
use crate::registry::{AppId, AppInfo};
use iced::widget::{button, container, row, svg, tooltip, tooltip::Position};
use iced::{Border, Color, Element, Length, Shadow, Vector};

// Message for Dock Interactions
#[derive(Debug, Clone)]
pub enum DockMessage {
    Launch(AppId),
}

pub fn view<'a>(apps: &[AppInfo], mode: ShellMode, is_light: bool) -> Element<'a, DockMessage> {
    let icons = apps.iter().map(move |app| {
        // 1. Determine icon path based on theme if it's a standard icon
        // 1. Determine icon path based on theme if it's a standard icon
        let icon_path = if app.icon_path.contains("icons/") && !app.icon_path.contains("riviera/") {
            let base_name = std::path::Path::new(app.icon_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            let theme_subdir = if is_light { "black" } else { "white" };

            // Try themed version first
            let themed_path = if base_name.ends_with(".svg") {
                format!("assets/icons/menubar/{}/{}", theme_subdir, base_name)
            } else {
                String::new()
            };

            if !themed_path.is_empty() && std::path::Path::new(&themed_path).exists() {
                themed_path
            } else {
                app.icon_path.to_string()
            }
        } else {
            app.icon_path.to_string()
        };

        let path_for_ext = std::path::Path::new(&icon_path);
        let ext = path_for_ext
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let icon: Element<DockMessage> = if ext == "png" || ext == "jpg" || ext == "jpeg" {
            match std::fs::read(&icon_path) {
                Ok(bytes) => iced::widget::image(iced::widget::image::Handle::from_bytes(bytes))
                    .width(Length::Fixed(20.0)) // Compact
                    .height(Length::Fixed(20.0))
                    .into(),
                Err(_) => container(iced::widget::text(&app.name[0..1]).size(12))
                    .width(Length::Fixed(20.0))
                    .height(Length::Fixed(20.0))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into(),
            }
        } else {
            svg(svg::Handle::from_path(&icon_path))
                .width(Length::Fixed(20.0))
                .height(Length::Fixed(20.0))
                .into()
        };

        tooltip(
            button(icon)
                .on_press(DockMessage::Launch(app.id))
                .style(|_theme, status| iced::widget::button::Style {
                    background: if status == iced::widget::button::Status::Hovered {
                        Some(Color::from_rgba(1.0, 1.0, 1.0, 0.1).into())
                    } else {
                        None
                    },
                    border: Border {
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .padding(4),
            app.name,
            Position::Top,
        )
        .style(move |_theme| container::Style {
            text_color: Some(if is_light { Color::BLACK } else { Color::WHITE }),
            ..Default::default()
        })
        .into()
    });

    let (bg_color, border_color) = match (mode, is_light) {
        (ShellMode::Peak, true) => (
            Color::from_rgba8(247, 245, 242, 0.7),
            Color::from_rgba8(35, 30, 30, 0.1),
        ),
        (ShellMode::Peak, false) => (
            Color::from_rgba8(15, 14, 14, 0.7),
            Color::from_rgba8(235, 230, 225, 0.15),
        ),
        (ShellMode::Poolside, _) => (
            Color::from_rgba8(255, 153, 204, 0.6),
            Color::from_rgba8(255, 255, 255, 0.3),
        ),
    };

    container(row(icons).spacing(8))
        .padding(6)
        .style(move |_theme| container::Style {
            background: Some(bg_color.into()),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 1.0, 0.78, 0.1),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        })
        .into()
}
