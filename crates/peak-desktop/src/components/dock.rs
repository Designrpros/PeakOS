use iced::widget::{button, container, row, svg, tooltip, tooltip::Position, Column};
use iced::{Border, Color, Element, Length, Shadow, Vector};
use peak_core::registry::{AppId, AppInfo, ShellMode};

// Message for Dock Interactions
#[derive(Debug, Clone)]
pub enum DockMessage {
    Launch(AppId),
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
    is_light: bool,
    mode: ShellMode,
) -> Element<'a, DockMessage> {
    let mut pinned_elements = Vec::new();
    let mut running_elements = Vec::new();
    let mut repo_elements = Vec::new();

    let icon_color = if is_light { "#000000" } else { "#FFFFFF" };

    // 1. Pinned Apps
    for (i, &id) in pinned.iter().enumerate() {
        pinned_elements.push(render_dock_icon(
            id,
            i,
            true, // is_pinned
            true, // is_running (assume for now, or check shell state)
            dragging,
            context_menu == Some(id),
            icon_color,
            is_light,
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
            icon_color,
            is_light,
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
            icon_color,
            is_light,
        ));
    }

    let (bg_color, border_color, shadow_color, divider_color) = match (mode, is_light) {
        (ShellMode::Peak, true) => (
            Color::from_rgba8(247, 245, 242, 0.8),
            Color::from_rgba8(35, 30, 30, 0.1),
            Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            Color::from_rgba(0.0, 0.0, 0.0, 0.1),
        ),
        (ShellMode::Peak, false) => (
            Color::from_rgba8(15, 14, 14, 0.8),
            Color::from_rgba8(235, 230, 225, 0.1),
            Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            Color::from_rgba(1.0, 1.0, 1.0, 0.1),
        ),

        (ShellMode::Poolside, _) => (
            Color::from_rgba8(255, 153, 204, 0.7),
            Color::from_rgba8(255, 255, 255, 0.3),
            Color::from_rgba(1.0, 0.0, 0.5, 0.2),
            Color::from_rgba(1.0, 1.0, 1.0, 0.3),
        ),
    };

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
                    background: Some(divider_color.into()),
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
                    background: Some(divider_color.into()),
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
            background: Some(bg_color.into()),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: Shadow {
                color: shadow_color,
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        })
        .into()
}

fn render_dock_icon<'a>(
    id: AppId,
    index: usize,
    _is_pinned: bool,
    is_running: bool,
    dragging: Option<(AppId, usize)>,
    _is_menu_open: bool,
    icon_color: &str,
    is_light: bool,
) -> Element<'a, DockMessage> {
    let info = AppInfo::get_info(id);
    let icon_handle = peak_core::icons::get_app_icon(id, icon_color);

    let _is_dragging = dragging.map(|(_, idx)| idx == index).unwrap_or(false);

    let icon: Element<DockMessage> = svg(icon_handle)
        .width(Length::Fixed(24.0))
        .height(Length::Fixed(24.0))
        .into();

    let indicator = if is_running {
        container(iced::widget::Space::with_width(Length::Fixed(3.0)))
            .height(Length::Fixed(3.0))
            .style(move |_| container::Style {
                background: Some(if is_light { Color::BLACK } else { Color::WHITE }.into()),
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
                    let hover_bg = if is_light {
                        Color::from_rgba(0.0, 0.0, 0.0, 0.05)
                    } else {
                        Color::from_rgba(1.0, 1.0, 1.0, 0.1)
                    };
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

    let element: Element<DockMessage> = tooltip(content, info.name, Position::Top)
        .style(move |_theme| container::Style {
            text_color: Some(if is_light { Color::BLACK } else { Color::WHITE }),
            ..Default::default()
        })
        .into();

    // if is_dragging {
    //     element = iced::widget::opacity(element, 0.5).into();
    // }

    // Wrap in a mouse area for drag and right click
    iced::widget::mouse_area(element)
        .on_right_press(DockMessage::RightClick(id))
        .on_press(DockMessage::StartDrag(index))
        .on_release(DockMessage::EndDrag)
        .on_enter(DockMessage::UpdateDrag(index))
        .into()
}
