use iced::widget::{button, container, row, svg, text, text_input};
use iced::{Alignment, Element, Length};
use peak_core::registry::AppId;
use peak_theme::ThemeTokens;

#[derive(Debug, Clone)]
pub enum AiShellMessage {
    Launch(AppId),
    OpenOmnibar,
    OpenStart,
}

pub fn layout<'a, Message>(
    content: Element<'a, Message>,
    pinned_apps: &[AppId],
    running_apps: &[AppId],
    tokens: ThemeTokens,
    ai_input_value: &str,
    on_input: impl Fn(String) -> Message + 'a + Clone,
    on_submit: Message,
    map_msg: impl Fn(AiShellMessage) -> Message + 'a + Clone,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    // --- TOP DOCK ("Notch" Style) ---
    // Attached to the top bezel, hanging down.
    let mut dock_row = row![].spacing(8).align_y(iced::Alignment::Center);

    // Reuse dock icon logic
    let map_dock_msg = |msg: crate::dock::DockMessage| match msg {
        crate::dock::DockMessage::Launch(id) => AiShellMessage::Launch(id),
        _ => AiShellMessage::Launch(AppId::Terminal), // Fallback/Ignore others
    };

    // Pinned
    for (i, &id) in pinned_apps.iter().enumerate() {
        let icon = crate::dock::render_dock_icon(
            id,
            i,
            true, // is_pinned
            running_apps.contains(&id),
            None,
            false,
            tokens,
        )
        .map(map_dock_msg)
        .map(map_msg.clone());

        dock_row = dock_row.push(icon);
    }

    // Separator
    if !running_apps.is_empty() {
        dock_row = dock_row.push(
            container(iced::widget::Space::with_width(Length::Fixed(1.0)))
                .height(Length::Fixed(24.0))
                .style(move |_| container::Style {
                    background: Some(tokens.colors.divider.into()),
                    ..Default::default()
                }),
        );
    }

    // Running
    for (i, &id) in running_apps.iter().enumerate() {
        if !pinned_apps.contains(&id) {
            let icon = crate::dock::render_dock_icon(
                id,
                pinned_apps.len() + i,
                false, // is_pinned
                true,  // is_running
                None,  // dragging
                false,
                tokens,
            )
            .map(map_dock_msg)
            .map(map_msg.clone());

            dock_row = dock_row.push(icon);
        }
    }

    // Notch container
    let top_dock = container(dock_row)
        .padding([12, 24]) // Wider padding for notch look
        .style(move |_| container::Style {
            // Match the BEZEL color (tokens.colors.background)
            background: Some(tokens.colors.background.into()),
            border: iced::Border {
                // Rounded bottom corners only
                radius: iced::border::Radius {
                    top_left: 0.0,
                    top_right: 0.0,
                    bottom_right: 24.0,
                    bottom_left: 24.0,
                },
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            ..Default::default()
        });

    // --- BOTTOM BAR ---
    let hex_color = format!(
        "#{:02x}{:02x}{:02x}",
        (tokens.colors.text_primary.r * 255.0) as u8,
        (tokens.colors.text_primary.g * 255.0) as u8,
        (tokens.colors.text_primary.b * 255.0) as u8
    );

    // PEAK LOGO (Replaces Robot)
    // If bezel (tokens.background) is Light -> Dark Logo.
    // If bezel is Dark -> Light Logo.
    // We check text color: Light Text -> Dark Background -> Light Logo.
    let logo_path = if tokens.colors.text_primary.r > 0.5 {
        "icons/menubar/peak_logo_dark.png"
    } else {
        "icons/menubar/peak_logo.png"
    };

    let logo_abs_path = peak_core::utils::assets::get_asset_path(logo_path);

    let home_btn = Element::from(
        button(
            iced::widget::image(logo_abs_path)
                .width(Length::Fixed(32.0))
                .height(Length::Fixed(32.0)),
        )
        .on_press(AiShellMessage::OpenStart)
        .padding(4)
        .style(move |_, _| iced::widget::button::Style::default()),
    )
    .map(map_msg.clone());

    // TEXT INPUT ("Ask something...")
    let omni_input = text_input("Ask something...", ai_input_value)
        .on_input(on_input)
        .on_submit(on_submit)
        .padding([10, 20])
        .size(14)
        .style(move |_, _| iced::widget::text_input::Style {
            background: iced::Background::Color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
            border: iced::Border {
                radius: 20.0.into(),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            icon: iced::Color::BLACK,
            placeholder: iced::Color::from_rgb(0.4, 0.4, 0.4),
            value: iced::Color::BLACK,
            selection: iced::Color::from_rgb(0.2, 0.4, 0.8),
        });
    let time = chrono::Local::now().format("%H:%M").to_string();
    let system_tray = row![
        svg(peak_core::icons::get_status_icon("wifi", &hex_color)).width(Length::Fixed(16.0)),
        svg(peak_core::icons::get_status_icon("volume", &hex_color)).width(Length::Fixed(16.0)),
        text(time).size(12).style(move |_| text::Style {
            color: Some(tokens.colors.text_primary)
        }),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let bottom_bar = row![
        container(home_btn).width(Length::Fill),
        container(omni_input)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),
        container(system_tray)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right),
    ]
    .spacing(20)
    .padding([10, 40]) // Padding for the bar in the bezel
    .align_y(Alignment::Center);

    // --- ASSEMBLY ---

    // 1. Desktop Content + Top Dock Overlay
    // Stack: Content (Bottom), Dock (Top)
    let desktop_with_dock = iced::widget::Stack::new().push(content).push(
        container(top_dock)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Top), // Attached to top
    );

    // 2. Bezel Frame

    container(
        iced::widget::column![
            // Inner Desktop (Rounded Screen)
            container(desktop_with_dock)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    // The desktop wallpaper is inside 'content'.
                    // We just provide the clipping checks/radius here.
                    border: iced::Border {
                        radius: 24.0.into(), // Large rounded corners for the "Screen"
                        width: 1.0,
                        color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1), // Subtle border for screen
                    },
                    // Transparent background so wallpaper (passed in content) shows through!
                    // If content is missing, it will be the bezel color behind it.
                    // But we want black if no wallpaper? No, normally wallpaper is there.
                    // Let's use transparent.
                    background: None,
                    ..Default::default()
                })
                .clip(true), // CLIP content to rounded corners!
            // Bottom Bar (In the bezel)
            container(bottom_bar)
                .width(Length::Fill)
                .height(Length::Shrink)
        ]
        .spacing(10),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(20) // The BEZEL thickness
    .style(move |_| container::Style {
        // The Frame Color
        background: Some(tokens.colors.background.into()),
        ..Default::default()
    })
    .into()
}
