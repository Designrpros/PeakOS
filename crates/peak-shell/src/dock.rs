use iced::widget::{button, container, image, row, svg, tooltip, tooltip::Position, Column};
use iced::{Border, Element, Length};
use peak_core::registry::{AppId, AppInfo};
use peak_theme::ThemeTokens;

// Message for Dock Interactions
#[derive(Debug, Clone)]
pub enum DockMessage {
    Launch(AppId),
    LaunchMedia(peak_core::models::MediaItem),
    StartDrag(usize),  // index
    UpdateDrag(usize), // current hover index
    EndDrag,
    RightClick(AppId),
    Pin(AppId),
    Unpin(AppId),
    Quit(AppId),
    CloseContextMenu,
}

pub fn view<'a>(
    pinned: &[AppId],
    running: &[AppId],
    repos: &[AppId],
    dragging: Option<(AppId, usize)>,
    context_menu: Option<AppId>,
    all_running: &[AppId],
    tokens: ThemeTokens,
) -> Element<'a, DockMessage> {
    let mut pinned_elements = Vec::new();
    let mut running_elements = Vec::new();
    let mut repo_elements = Vec::new();

    // 1. Pinned Apps
    for (i, &id) in pinned.iter().enumerate() {
        pinned_elements.push(render_dock_icon(
            id,
            i,
            true,                      // is_pinned
            all_running.contains(&id), // Check if actually running
            dragging,
            context_menu == Some(id),
            tokens,
        ));
    }

    // 2. Running Apps (not pinned, not repos)
    for (i, &id) in running.iter().enumerate() {
        let global_index = pinned.len() + i;
        running_elements.push(render_dock_icon(
            id,
            global_index,
            false, // is_pinned
            true,  // is_running
            dragging,
            context_menu == Some(id),
            tokens,
        ));
    }

    // 3. Repositories
    for (i, &id) in repos.iter().enumerate() {
        let global_index = pinned.len() + running.len() + i;
        repo_elements.push(render_dock_icon(
            id,
            global_index,
            false,
            true, // Usually repos are "active" if they are in this list
            dragging,
            context_menu == Some(id),
            tokens,
        ));
    }

    let mut dock_row = row![].spacing(8).align_y(iced::Alignment::Center);

    for icon in pinned_elements {
        dock_row = dock_row.push(icon);
    }

    if !running.is_empty() {
        // Vertical Divider
        dock_row = dock_row.push(
            container(iced::widget::Space::with_width(Length::Fixed(1.0)))
                .height(Length::Fixed(24.0))
                .style(move |_| container::Style {
                    background: Some(tokens.colors.divider.into()),
                    ..Default::default()
                }),
        );

        for icon in running_elements {
            dock_row = dock_row.push(icon);
        }
    }

    if !repos.is_empty() {
        // Another Vertical Divider for Repos
        dock_row = dock_row.push(
            container(iced::widget::Space::with_width(Length::Fixed(1.0)))
                .height(Length::Fixed(24.0))
                .style(move |_| container::Style {
                    background: Some(tokens.colors.divider.into()),
                    ..Default::default()
                }),
        );

        for icon in repo_elements {
            dock_row = dock_row.push(icon);
        }
    }

    container(dock_row)
        .padding(6)
        .style(move |_theme| container::Style {
            background: Some({
                let mut c = tokens.colors.surface;
                c.a = tokens.glass_opacity;
                c.into()
            }),
            border: Border {
                radius: tokens.radius.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

pub fn render_dock_icon<'a>(
    id: AppId,
    index: usize,
    _is_pinned: bool,
    is_running: bool,
    _dragging: Option<(AppId, usize)>,
    _is_menu_open: bool,
    tokens: ThemeTokens,
) -> Element<'a, DockMessage> {
    let info = AppInfo::get_info(id);
    let hex_color = format!(
        "#{:02x}{:02x}{:02x}",
        (tokens.colors.text_primary.r * 255.0) as u8,
        (tokens.colors.text_primary.g * 255.0) as u8,
        (tokens.colors.text_primary.b * 255.0) as u8
    );
    let icon: Element<DockMessage> =
        match peak_core::icons::IconResolver::resolve_app_icon(id, &hex_color) {
            peak_core::icons::AppIcon::Svg(handle) => svg(handle)
                .width(Length::Fixed(24.0))
                .height(Length::Fixed(24.0))
                .into(),
            peak_core::icons::AppIcon::Image(handle) => image(handle)
                .width(Length::Fixed(24.0))
                .height(Length::Fixed(24.0))
                .into(),
        };

    let indicator = if is_running {
        container(iced::widget::Space::with_width(Length::Fixed(3.0)))
            .height(Length::Fixed(3.0))
            .style(move |_| container::Style {
                background: Some(tokens.colors.text_primary.into()),
                border: Border {
                    radius: 1.5.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
    } else {
        container(iced::widget::Space::with_height(Length::Fixed(3.0)))
    };

    let content = Column::new()
        .push(
            button(icon)
                .on_press(DockMessage::Launch(id))
                .style(move |_theme, status| {
                    let hover_bg = tokens.colors.text_primary;
                    let mut hover_bg = hover_bg;
                    hover_bg.a = 0.1;

                    iced::widget::button::Style {
                        background: if status == iced::widget::button::Status::Hovered {
                            Some(hover_bg.into())
                        } else {
                            None
                        },
                        border: Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .padding(6),
        )
        .push(indicator)
        .spacing(2)
        .align_x(iced::Alignment::Center);

    let text_color = tokens.colors.text_primary;
    let element: Element<DockMessage> = tooltip(content, info.name, Position::Top)
        .style(move |_theme| container::Style {
            text_color: Some(text_color),
            ..Default::default()
        })
        .into();

    // Wrap in a mouse area for drag and right click
    iced::widget::mouse_area(element)
        .on_right_press(DockMessage::RightClick(id))
        .on_press(DockMessage::StartDrag(index))
        .on_release(DockMessage::EndDrag)
        .on_enter(DockMessage::UpdateDrag(index))
        .into()
}
