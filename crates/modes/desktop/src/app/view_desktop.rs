// Desktop view rendering

use super::{Message, PeakNative};
use crate::pages::Page;
use iced::widget::{button, container, text, text as t, Stack};
use iced::{Border, Color, Element, Length, Shadow, Vector};
use peak_core::registry::ShellMode;
use peak_shell::{
    console, dock,
    menubar::{self, MenubarMessage},
    tv,
};

impl PeakNative {
    fn view_app_grid(&self, tokens: peak_theme::ThemeTokens) -> Element<'_, Message> {
        let grid = crate::components::app_grid::view(&self.scanned_apps, tokens)
            .map(Message::DockInteraction);

        container(grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    pub fn view_desktop(&self) -> Element<'_, Message> {
        // --- SEPARATE PROCESS MODES ---
        if self.launch_mode == crate::app::LaunchMode::Bar {
            return container(menubar::view(self.tokens).map(Message::MenubarAction))
                .width(Length::Fill)
                .height(Length::Fill) // Layer Shell handles sizing
                .align_y(iced::alignment::Vertical::Top)
                .into();
        }

        if self.launch_mode == crate::app::LaunchMode::Dock {
            let mut standard_pinned = Vec::new();
            let mut repo_pinned = Vec::new();
            for &id in &self.pinned_apps {
                if id.is_repo() {
                    repo_pinned.push(id);
                } else {
                    standard_pinned.push(id);
                }
            }
            let mut standard_running = Vec::new();
            let mut repo_running = Vec::new();
            let all_running: Vec<_> = self.window_manager.window_states.keys().cloned().collect();
            for &id in &all_running {
                if id.is_repo() {
                    if !repo_pinned.contains(&id) {
                        repo_running.push(id);
                    }
                } else if !standard_pinned.contains(&id) {
                    standard_running.push(id);
                }
            }
            let mut repos = repo_pinned;
            for r in repo_running {
                if !repos.contains(&r) {
                    repos.push(r);
                }
            }

            let dock_element = dock::view(
                &standard_pinned,
                &standard_running,
                &repos,
                self.dragging_app,
                self.context_menu_app,
                &all_running,
                self.tokens,
            )
            .map(Message::DockInteraction);

            return container(dock_element)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Bottom)
                .padding(iced::Padding {
                    bottom: 0.0, // Layer shell should handle positioning? Or padding needed?
                    ..Default::default()
                })
                .into();
        }

        // --- DESKTOP MODE (Wallpaper + Windows + Inspector) ---
        // TODO: If using separate processes, we might want to optionally HIDE the bar/dock in this view.
        // For now, if we are in 'Desktop' mode, we render the full desktop environment,
        // but if we spawned subprocesses (checked via some flag? or just always if linux?), we might duplicate.
        // BUT: iced_layershell logic dictates we only get a surface for the "Desktop" layer (Background).
        // So the Bar and Dock overlays won't work in the same "Window" if it is a LayerShell surface at Bottom.

        let is_light = self.theme == peak_core::theme::Theme::Light;
        let wallpaper_path = if let Some(custom) = &self.custom_wallpaper {
            peak_core::utils::assets::get_asset_path(&format!("wallpapers/{}", custom))
        } else {
            // Default wallpaper based on shell mode only (not theme)
            match self.mode {
                ShellMode::Desktop => {
                    peak_core::utils::assets::get_asset_path("wallpapers/mountain_sunset_warm.jpg")
                }
                ShellMode::Mobile => {
                    peak_core::utils::assets::get_asset_path("wallpapers/Florida.jpeg")
                }
                ShellMode::TV => {
                    peak_core::utils::assets::get_asset_path("wallpapers/treeInDesert.jpeg")
                }
                ShellMode::Console => peak_core::utils::assets::get_asset_path(
                    "wallpapers/poolsuite_luxury_night.jpg",
                ),
                ShellMode::Kiosk => peak_core::utils::assets::get_asset_path(
                    "wallpapers/mountain_classic_light.jpg",
                ),
                ShellMode::Fireplace => {
                    peak_core::utils::assets::get_asset_path("wallpapers/poolsuite_luxury-kopi.jpg")
                }
                ShellMode::Auto => {
                    peak_core::utils::assets::get_asset_path("wallpapers/mountain_classic.jpg")
                }
                ShellMode::Robot => {
                    peak_core::utils::assets::get_asset_path("wallpapers/Desert.jpeg")
                }
                ShellMode::Server => {
                    peak_core::utils::assets::get_asset_path("wallpapers/treeInDesert.jpeg")
                }
                ShellMode::SmartHome => {
                    peak_core::utils::assets::get_asset_path("wallpapers/Florida.jpeg")
                }
            }
        };

        // Convert PeakOS ThemeTokens to PeakUI ThemeTokens
        let ui_tokens = peak_ui_theme::ThemeTokens {
            colors: peak_ui_theme::PeakColors {
                primary: self.tokens.colors.primary,
                on_primary: self.tokens.colors.on_primary,
                primary_container: self.tokens.colors.primary_container,
                on_primary_container: self.tokens.colors.on_primary_container,

                secondary: self.tokens.colors.secondary,
                on_secondary: self.tokens.colors.on_secondary,
                secondary_container: self.tokens.colors.secondary_container,
                on_secondary_container: self.tokens.colors.on_secondary_container,

                accent: self.tokens.colors.accent,
                on_accent: self.tokens.colors.on_accent,

                success: self.tokens.colors.success,
                warning: self.tokens.colors.warning,
                danger: self.tokens.colors.danger,
                info: self.tokens.colors.info,

                surface: self.tokens.colors.surface,
                on_surface: self.tokens.colors.on_surface,
                surface_variant: self.tokens.colors.surface_variant,
                on_surface_variant: self.tokens.colors.on_surface_variant,

                background: self.tokens.colors.background,
                on_background: self.tokens.colors.on_background,

                border: self.tokens.colors.border,
                divider: self.tokens.colors.divider,
                overlay: self.tokens.colors.overlay,

                text_primary: self.tokens.colors.text_primary,
                text_secondary: self.tokens.colors.text_secondary,
                text_tertiary: self.tokens.colors.text_tertiary,
                text_disabled: self.tokens.colors.text_disabled,
            },
            tone: match self.tokens.tone {
                peak_theme::ThemeTone::Light => peak_ui_theme::ThemeTone::Light,
                peak_theme::ThemeTone::Dark => peak_ui_theme::ThemeTone::Dark,
            },
            glass_opacity: self.tokens.glass_opacity,
            blur_radius: self.tokens.blur_radius,
            radius: self.tokens.radius,
            shadow_color: self.tokens.shadow_color,
            shadow_offset: self.tokens.shadow_offset,
            shadow_blur: self.tokens.shadow_blur,
            spacing_unit: self.tokens.spacing_unit,
            scaling: self.tokens.scaling,
        };

        // Dynamic z-order Workspace Rendering
        let context = peak_ui::core::Context::new(
            peak_ui::core::ShellMode::Desktop,
            ui_tokens,
            self.window_manager.screen_size,
            Default::default(),
        );

        let mut workspace_stack = Stack::new().push(
            peak_ui::core::View::<
                crate::components::desktop::DesktopMessage,
                peak_ui::core::IcedBackend,
            >::view(&self.desktop, &context)
            .map(Message::Desktop),
        );

        // Integrated Desktop UI
        // For Linux, we only do this in Desktop mode (to show the frame/rail over the wallpaper)
        // because Bar/Dock are separate processes.
        let should_render_integrated = if cfg!(target_os = "linux") {
            self.launch_mode == crate::app::LaunchMode::Desktop
        } else {
            true
        };

        if should_render_integrated {
            match self.shell_style {
                peak_core::registry::ShellStyle::Cupertino => {
                    // --- CUPERTINO: Menubar (Top) + Dock (Bottom) ---

                    // Add Menubar (Top)
                    workspace_stack = workspace_stack.push(
                        container(menubar::view(self.tokens).map(Message::MenubarAction))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_y(iced::alignment::Vertical::Top),
                    );

                    // Add Dock (Bottom)
                    let mut standard_pinned = Vec::new();
                    let mut repo_pinned = Vec::new();
                    for &id in &self.pinned_apps {
                        if id.is_repo() {
                            repo_pinned.push(id);
                        } else {
                            standard_pinned.push(id);
                        }
                    }
                    let mut standard_running = Vec::new();
                    let mut repo_running = Vec::new();
                    let all_running: Vec<_> =
                        self.window_manager.window_states.keys().cloned().collect();
                    for &id in &all_running {
                        if id.is_repo() {
                            if !repo_pinned.contains(&id) {
                                repo_running.push(id);
                            }
                        } else if !standard_pinned.contains(&id) {
                            standard_running.push(id);
                        }
                    }
                    let mut repos = repo_pinned;
                    for r in repo_running {
                        if !repos.contains(&r) {
                            repos.push(r);
                        }
                    }

                    let dock_element = dock::view(
                        &standard_pinned,
                        &standard_running,
                        &repos,
                        self.dragging_app,
                        self.context_menu_app,
                        &all_running,
                        self.tokens,
                    )
                    .map(Message::DockInteraction);

                    workspace_stack = workspace_stack.push(
                        container(dock_element)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(iced::alignment::Horizontal::Center)
                            .align_y(iced::alignment::Vertical::Bottom)
                            .padding(iced::Padding {
                                bottom: 0.0,
                                ..Default::default()
                            }),
                    );
                }
                peak_core::registry::ShellStyle::Redmond => {
                    // --- REDMOND: Taskbar (Bottom) ---
                    // Note: No top menubar in Redmond style

                    let taskbar_element = peak_shell::redmond::taskbar::view(
                        &self.pinned_apps,
                        &self.running_apps, // Use running_apps vec instead of z_order check for simplicity first
                        self.tokens,
                    )
                    .map(Message::RedmondTaskbar);

                    workspace_stack = workspace_stack.push(
                        container(taskbar_element)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_y(iced::alignment::Vertical::Bottom),
                    );
                }
                peak_core::registry::ShellStyle::AI => {
                    // --- PEAK AI: Warmwind UI (Frame Layout) ---
                    // Wraps the entire desktop content in a rounded frame + bottom bar

                    // Apply Wallpaper INSIDE the frame (Screen) with Rounded Corners
                    // We inline the desktop container logic here to apply border radius
                    let wallpaper = container(
                        iced::widget::image(iced::widget::image::Handle::from_path(
                            &wallpaper_path,
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(iced::ContentFit::Cover),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .clip(true) // Helper check
                    .style(|_| container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb8(
                            10, 10, 15,
                        ))),
                        border: iced::Border {
                            radius: 24.0.into(), // Force rounded corners on the wallpaper container itself
                            width: 0.0,
                            color: iced::Color::TRANSPARENT,
                        },
                        ..Default::default()
                    });

                    let content_with_wallpaper = iced::widget::stack![
                        wallpaper,
                        container(workspace_stack)
                            .width(Length::Fill)
                            .height(Length::Fill)
                    ];

                    let desktop_content = container(content_with_wallpaper)
                        .width(Length::Fill)
                        .height(Length::Fill); // Wrap current stack

                    let framed = peak_shell::ai::layout(
                        desktop_content.into(),
                        &self.pinned_apps,
                        &self.running_apps,
                        self.tokens,
                        &self.ai_input_text,
                        |input| Message::AiInputChange(input),
                        Message::AiSubmit,
                        |msg| match msg {
                            peak_shell::ai::AiShellMessage::Launch(id) => {
                                Message::DockInteraction(peak_shell::dock::DockMessage::Launch(id))
                            }
                            peak_shell::ai::AiShellMessage::OpenOmnibar => Message::ToggleOmnibar,
                            peak_shell::ai::AiShellMessage::OpenStart => Message::ToggleAppGrid,
                        },
                    );

                    // Re-initialize stack with the framed content
                    workspace_stack = Stack::new().push(framed);
                }
                peak_core::registry::ShellStyle::Console => {
                    // --- CONSOLE: AI Shell Frame + Game Rail ---

                    // 1. Content (Game Rail)
                    let console_content = container(
                        iced::widget::column![
                            console::category_bar::view(
                                console::category_bar::GameCategory::All,
                                self.tokens
                            )
                            .map(Message::ConsoleCategory),
                            console::game_rail::view(&self.games, 0, self.tokens)
                                .map(Message::ConsoleGame),
                        ]
                        .spacing(20),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(iced::alignment::Vertical::Bottom)
                    .padding(40);

                    // 2. Wallpaper (Inner Screen)
                    let wallpaper = container(
                        iced::widget::image(iced::widget::image::Handle::from_path(
                            &wallpaper_path,
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(iced::ContentFit::Cover),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .clip(true)
                    .style(|_| container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb8(
                            10, 10, 15,
                        ))),
                        border: iced::Border {
                            radius: 24.0.into(),
                            width: 0.0,
                            color: iced::Color::TRANSPARENT,
                        },
                        ..Default::default()
                    });

                    // 3. Stack Content on Wallpaper
                    // NOTE: workspace_stack contains windows. We put them UNDER the game rail?
                    // Or do we assume Console mode has no windows?
                    // Let's stack: Wallpaper -> Windows -> Game Rail
                    let content_with_wallpaper = iced::widget::stack![
                        wallpaper,
                        container(workspace_stack)
                            .width(Length::Fill)
                            .height(Length::Fill),
                        console_content
                    ];

                    let desktop_content = container(content_with_wallpaper)
                        .width(Length::Fill)
                        .height(Length::Fill);

                    // 4. Wrap with Frame
                    let framed = peak_shell::console::layout::layout(
                        desktop_content.into(),
                        &self.pinned_apps,
                        &self.running_apps,
                        self.tokens,
                        &self.ai_input_text,
                        |input| Message::AiInputChange(input),
                        Message::AiSubmit,
                        |msg| match msg {
                            peak_shell::console::layout::ConsoleShellMessage::Launch(id) => {
                                Message::DockInteraction(peak_shell::dock::DockMessage::Launch(id))
                            }
                            peak_shell::console::layout::ConsoleShellMessage::OpenOmnibar => {
                                Message::ToggleOmnibar
                            }
                            peak_shell::console::layout::ConsoleShellMessage::OpenStart => {
                                Message::ToggleAppGrid
                            }
                        },
                    );

                    workspace_stack = Stack::new().push(framed);
                }
                peak_core::registry::ShellStyle::TV => {
                    // --- TV: AI Shell Frame + App Rail ---

                    // 1. Content (App Rail)
                    let tv_content = container(
                        tv::app_rail::view(&self.pinned_apps, 0, self.tokens).map(Message::TVApp),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(iced::alignment::Vertical::Bottom)
                    .padding(60);

                    // 2. Wallpaper (Inner Screen)
                    let wallpaper = container(
                        iced::widget::image(iced::widget::image::Handle::from_path(
                            &wallpaper_path,
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(iced::ContentFit::Cover),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .clip(true)
                    .style(|_| container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb8(
                            10, 10, 15,
                        ))),
                        border: iced::Border {
                            radius: 24.0.into(),
                            width: 0.0,
                            color: iced::Color::TRANSPARENT,
                        },
                        ..Default::default()
                    });

                    // 3. Stack Content on Wallpaper
                    let content_with_wallpaper = iced::widget::stack![
                        wallpaper,
                        container(workspace_stack)
                            .width(Length::Fill)
                            .height(Length::Fill),
                        tv_content
                    ];

                    let desktop_content = container(content_with_wallpaper)
                        .width(Length::Fill)
                        .height(Length::Fill);

                    // 4. Wrap with Frame
                    let framed = peak_shell::tv::layout::layout(
                        desktop_content.into(),
                        &self.pinned_apps,
                        &self.running_apps,
                        self.tokens,
                        &self.ai_input_text,
                        |input| Message::AiInputChange(input),
                        Message::AiSubmit,
                        |msg| match msg {
                            peak_shell::tv::layout::TvShellMessage::Launch(id) => {
                                Message::DockInteraction(peak_shell::dock::DockMessage::Launch(id))
                            }
                            peak_shell::tv::layout::TvShellMessage::OpenOmnibar => {
                                Message::ToggleOmnibar
                            }
                            peak_shell::tv::layout::TvShellMessage::OpenStart => {
                                Message::ToggleAppGrid
                            }
                        },
                    );

                    workspace_stack = Stack::new().push(framed);
                }
            }
        }
        use crate::components::window_compositor::WindowCompositor;
        let compositor = WindowCompositor::new(
            &self.window_manager,
            &self.registry,
            &self.theme,
            self.mode,
            self.current_desktop,
            self.is_desktop_revealed,
        );
        let workspace_stack = compositor.view(workspace_stack.into());

        // Simplification: We remove Dock/Bar from THIS view.

        // Check if wallpaper needs to be applied (For AI Shell, it's already inside the frame)
        let workspace = match self.shell_style {
            peak_core::registry::ShellStyle::AI
            | peak_core::registry::ShellStyle::Console
            | peak_core::registry::ShellStyle::TV => workspace_stack.into(),

            _ => {
                crate::components::desktop_container::view(&wallpaper_path, workspace_stack.into())
            }
        };

        let mut workspace_stack = Stack::new().push(workspace);

        if self.inspector.is_visible {
            workspace_stack = workspace_stack.push(
                container(self.inspector.view(self.tokens).map(Message::Inspector))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(iced::Padding {
                        top: 45.0,
                        right: 12.0,
                        bottom: 12.0,
                        left: 0.0,
                    })
                    .align_x(iced::alignment::Horizontal::Right)
                    .align_y(iced::alignment::Vertical::Top),
            );
        }

        // Menubar overlay (top) - Only for Cupertino/Redmond styles
        // Menubar overlay (top) - DELEGATED TO SEPARATE PROCESS FOR CUPERTINO/REDMOND
        let mut final_view = workspace_stack;

        // Bottom Shell overlay (Dock / Taskbar / Rail)
        final_view = match self.shell_style {
            peak_core::registry::ShellStyle::Cupertino => final_view, // Delegated
            peak_core::registry::ShellStyle::Redmond => final_view,   // Delegated
            peak_core::registry::ShellStyle::Console => final_view,
            peak_core::registry::ShellStyle::TV => final_view,
            peak_core::registry::ShellStyle::AI => {
                // Placeholder for AI input / dock
                final_view
            }
        };

        if self.show_omnibar {
            final_view = final_view.push(
                container(self.omnibar.view(self.tokens).map(Message::Omnibar))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(100.0),
            );
        }

        if self.show_app_grid {
            final_view = final_view
                .push(
                    container(
                        button(text(""))
                            .style(move |_, _| button::Style {
                                background: Some(self.tokens.colors.background.into()),
                                ..Default::default()
                            })
                            .on_press(Message::ToggleAppGrid)
                            .width(Length::Fill)
                            .height(Length::Fill),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill),
                )
                .push(
                    container(self.view_app_grid(self.tokens))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill),
                );
        }

        if self.show_switcher {
            final_view = final_view
                .push(
                    container(iced::widget::text(""))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(|_| container::Style {
                            background: Some(iced::Background::Color(Color::from_rgba(
                                0.0, 0.0, 0.0, 0.2,
                            ))),
                            ..Default::default()
                        }),
                )
                .push(
                    container(self.switcher.view(self.tokens).map(Message::Switcher))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill),
                );
        }

        if self.show_spaces_selector {
            final_view = final_view.push(
                container(
                    crate::components::spaces_strip::view(self.tokens, self.current_desktop)
                        .map(Message::SwitchSpace),
                )
                .width(Length::Fill)
                .padding(iced::Padding {
                    top: 50.0,
                    ..Default::default()
                })
                .center_x(Length::Fill)
                .align_y(iced::alignment::Vertical::Top),
            );
        }

        if self.show_system_menu || self.show_reality_menu || self.show_wifi_menu {
            let tokens = self.tokens;
            let text_color = tokens.colors.text_primary;
            let bg = tokens.colors.surface;

            let menu_button =
                move |label: String, msg: Message, active: bool| -> Element<'_, Message> {
                    button(
                        container(text(label).size(13))
                            .width(Length::Fill)
                            .padding([5, 10]),
                    )
                    .on_press(msg)
                    .style(move |_, status| {
                        let is_hovered = status == iced::widget::button::Status::Hovered;
                        let final_bg = if active {
                            let mut c = tokens.colors.primary;
                            c.a = 0.2;
                            c
                        } else if is_hovered {
                            let mut c = tokens.colors.text_primary;
                            c.a = 0.1;
                            c
                        } else {
                            Color::TRANSPARENT
                        };

                        button::Style {
                            background: Some(final_bg.into()),
                            text_color: if active {
                                tokens.colors.primary
                            } else {
                                tokens.colors.text_primary
                            },
                            border: iced::Border {
                                radius: 4.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    })
                    .into()
                };

            if self.show_system_menu {
                let menu = container(
                    iced::widget::column![
                        menu_button("About PeakOS".into(), Message::Navigate(Page::Home), false),
                        menu_button(
                            "System Preferences...".into(),
                            Message::ToggleSettings,
                            false
                        ),
                        iced::widget::Space::new().height(5.0),
                        container(iced::widget::Space::new().height(0.5))
                            .width(Length::Fill)
                            .style(move |_| container::Style {
                                background: Some(
                                    Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.2)
                                        .into()
                                ),
                                ..Default::default()
                            }),
                        iced::widget::Space::new().height(5.0),
                        menu_button("Log Out...".into(), Message::LogOut, false),
                        menu_button("Factory Reset...".into(), Message::FactoryReset, false),
                        menu_button("Restart...".into(), Message::Restart, false),
                        menu_button("Shut Down...".into(), Message::Exit, false),
                    ]
                    .width(Length::Fixed(180.0))
                    .padding(5),
                )
                .style(move |_| container::Style {
                    background: Some(Color::from_rgba(bg.r, bg.g, bg.b, 0.92).into()),
                    border: iced::Border {
                        color: Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.1),
                        width: 1.0,
                        radius: 12.0.into(),
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: iced::Vector::new(0.0, 8.0),
                        blur_radius: 16.0,
                    },
                    ..Default::default()
                });

                final_view = final_view.push(
                    container(menu)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(iced::Padding {
                            top: 40.0,
                            left: 10.0,
                            ..Default::default()
                        })
                        .align_x(iced::alignment::Horizontal::Left)
                        .align_y(iced::alignment::Vertical::Top),
                );
            }

            if self.show_reality_menu {
                let menu = container(
                    iced::widget::column![
                        menu_button(
                            "Desktop".into(),
                            Message::SwitchMode(ShellMode::Desktop),
                            self.mode == ShellMode::Desktop
                        ),
                        menu_button(
                            "Mobile".into(),
                            Message::SwitchMode(ShellMode::Mobile),
                            self.mode == ShellMode::Mobile
                        ),
                        menu_button(
                            "TV".into(),
                            Message::SwitchMode(ShellMode::TV),
                            self.mode == ShellMode::TV
                        ),
                        menu_button(
                            "Console".into(),
                            Message::SwitchMode(ShellMode::Console),
                            self.mode == ShellMode::Console
                        ),
                        menu_button(
                            "Kiosk".into(),
                            Message::SwitchMode(ShellMode::Kiosk),
                            self.mode == ShellMode::Kiosk
                        ),
                        menu_button(
                            "Fireplace".into(),
                            Message::SwitchMode(ShellMode::Fireplace),
                            self.mode == ShellMode::Fireplace
                        ),
                        menu_button(
                            "Auto".into(),
                            Message::SwitchMode(ShellMode::Auto),
                            self.mode == ShellMode::Auto
                        ),
                        menu_button(
                            "Robot".into(),
                            Message::SwitchMode(ShellMode::Robot),
                            self.mode == ShellMode::Robot
                        ),
                        menu_button(
                            "Server".into(),
                            Message::SwitchMode(ShellMode::Server),
                            self.mode == ShellMode::Server
                        ),
                        menu_button(
                            "Home".into(),
                            Message::SwitchMode(ShellMode::SmartHome),
                            self.mode == ShellMode::SmartHome
                        ),
                    ]
                    .width(Length::Fixed(160.0))
                    .padding(5),
                )
                .style(move |_| container::Style {
                    background: Some(Color::from_rgba(bg.r, bg.g, bg.b, 0.92).into()),
                    border: iced::Border {
                        color: Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.1),
                        width: 1.0,
                        radius: 12.0.into(),
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: iced::Vector::new(0.0, 8.0),
                        blur_radius: 16.0,
                    },
                    ..Default::default()
                });

                final_view = final_view.push(
                    container(menu)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(iced::Padding {
                            top: 40.0,
                            left: 60.0,
                            ..Default::default()
                        })
                        .align_x(iced::alignment::Horizontal::Left)
                        .align_y(iced::alignment::Vertical::Top),
                );
            }

            if self.show_wifi_menu {
                let mut wifi_content = iced::widget::column![
                    iced::widget::text("Wi-Fi Networks")
                        .size(12)
                        .style(move |_| t::Style {
                            color: Some(Color::from_rgba(
                                text_color.r,
                                text_color.g,
                                text_color.b,
                                0.6
                            ))
                        }),
                    iced::widget::Space::new().height(5.0),
                    container(iced::widget::Space::new().height(0.5))
                        .width(Length::Fill)
                        .style(move |_| container::Style {
                            background: Some(
                                Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.2)
                                    .into()
                            ),
                            ..Default::default()
                        }),
                    iced::widget::Space::new().height(5.0),
                ];

                for network in &self.networks {
                    wifi_content = wifi_content.push(menu_button(
                        network.clone(),
                        Message::MenubarAction(MenubarMessage::ToggleWifiMenu),
                        network == "PeakOS_5G",
                    ));
                }

                let menu = container(wifi_content.width(Length::Fixed(200.0)).padding(10)).style(
                    move |_| container::Style {
                        background: Some(Color::from_rgba(bg.r, bg.g, bg.b, 0.92).into()),
                        border: Border {
                            color: Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.1),
                            width: 1.0,
                            radius: 12.0.into(),
                        },
                        shadow: Shadow {
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                            offset: Vector::new(0.0, 8.0),
                            blur_radius: 16.0,
                        },
                        ..Default::default()
                    },
                );

                final_view = final_view.push(
                    container(menu)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(iced::Padding {
                            top: 40.0,
                            right: 10.0,
                            ..Default::default()
                        })
                        .align_x(iced::alignment::Horizontal::Right)
                        .align_y(iced::alignment::Vertical::Top),
                );
            }
        }

        if let Some(app_id) = self.context_menu_app {
            let is_pinned = self.pinned_apps.contains(&app_id);
            let mut menu_col = iced::widget::Column::new().spacing(4).padding(4);

            if !is_pinned {
                menu_col = menu_col.push(
                    button(text("Keep in Dock"))
                        .on_press(Message::DockInteraction(dock::DockMessage::Pin(app_id)))
                        .width(Length::Fill)
                        .padding(8)
                        .style(move |_theme, _status| button::Style {
                            background: Some(Color::TRANSPARENT.into()),
                            text_color: if is_light { Color::BLACK } else { Color::WHITE },
                            ..Default::default()
                        }),
                );
            } else {
                menu_col = menu_col.push(
                    button(text("Remove from Dock"))
                        .on_press(Message::DockInteraction(dock::DockMessage::Unpin(app_id)))
                        .width(Length::Fill)
                        .padding(8)
                        .style(move |_theme, _status| button::Style {
                            background: Some(Color::TRANSPARENT.into()),
                            text_color: if is_light { Color::BLACK } else { Color::WHITE },
                            ..Default::default()
                        }),
                );
            }

            menu_col = menu_col.push(
                button(text("Quit"))
                    .on_press(Message::DockInteraction(dock::DockMessage::Quit(app_id)))
                    .width(Length::Fill)
                    .padding(8)
                    .style(move |_theme, _status| button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        text_color: Color::from_rgb(0.8, 0.2, 0.2),
                        ..Default::default()
                    }),
            );

            let context_menu_element: Element<'_, Message> = container(menu_col)
                .width(Length::Fixed(160.0))
                .style(move |_| container::Style {
                    background: Some(
                        if is_light {
                            Color::from_rgba8(250, 248, 245, 0.95)
                        } else {
                            Color::from_rgba8(25, 24, 24, 0.95)
                        }
                        .into(),
                    ),
                    border: Border {
                        color: if is_light {
                            Color::from_rgba(0.0, 0.0, 0.0, 0.1)
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.1)
                        },
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    shadow: Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: Vector::new(0.0, 4.0),
                        blur_radius: 12.0,
                    },
                    ..Default::default()
                })
                .into();

            final_view = final_view.push(
                iced::widget::mouse_area(
                    container(
                        iced::widget::mouse_area(container(context_menu_element).padding(2))
                            .on_press(Message::Tick),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(iced::Padding {
                        bottom: 70.0,
                        ..Default::default()
                    })
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Bottom),
                )
                .on_press(Message::DockInteraction(
                    dock::DockMessage::CloseContextMenu,
                )),
            );
        }

        // --- GLOBAL DRAG OVERLAY ---
        if self.window_manager.dragging.is_some() || self.dragging_app.is_some() {
            Stack::new()
                .push(final_view)
                .push(
                    iced::widget::mouse_area(
                        container(iced::widget::Space::new().height(Length::Fill))
                            .width(Length::Fill)
                            .height(Length::Fill),
                    )
                    .on_release(Message::Inspector(
                        crate::components::inspector::InspectorMessage::MouseReleased,
                    )),
                )
                .into()
        } else {
            final_view.into()
        }
    }
}
