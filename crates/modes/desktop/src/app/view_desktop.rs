// Desktop view rendering

use super::{Message, PeakNative};
use crate::pages::Page;
use iced::widget::{button, container, text, text as t, Column, Stack};
use iced::{Border, Color, Element, Length, Shadow, Vector};
use peak_core::registry::ShellMode;
use peak_shell::{
    dock,
    menubar::{self, MenubarMessage},
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
                } else {
                    if !standard_pinned.contains(&id) {
                        standard_running.push(id);
                    }
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
            match (self.mode, is_light) {
                (ShellMode::Peak, true) => peak_core::utils::assets::get_asset_path(
                    "wallpapers/mountain_classic_light.jpg",
                ),
                (ShellMode::Peak, false) => {
                    peak_core::utils::assets::get_asset_path("wallpapers/mountain_classic.jpg")
                }
                (ShellMode::Poolside, true) => {
                    peak_core::utils::assets::get_asset_path("wallpapers/poolsuite_luxury.jpg")
                }
                (ShellMode::Poolside, false) => peak_core::utils::assets::get_asset_path(
                    "wallpapers/poolsuite_luxury_night.jpg",
                ),
            }
        };

        use peak_ui::window_chrome;

        // Dynamic z-order Workspace Rendering
        let mut workspace_stack = Stack::new().push(self.desktop.view().map(Message::Desktop));

        for &app_id in &self.window_manager.z_order {
            if let Some(state) = self.window_manager.window_states.get(&app_id) {
                // Workspace Filtering
                if !state.is_sticky
                    && (state.reality != self.mode || state.desktop_idx != self.current_desktop)
                {
                    continue;
                }

                let content: Element<'_, Message> =
                    if let Some(app) = self.registry.running_apps.get(&app_id) {
                        app.view(&self.theme)
                    } else {
                        container(iced::widget::text("UNSUPPORTED")).into()
                    };

                let title = if let Some(all_app) = self.registry.running_apps.get(&app_id) {
                    all_app.title()
                } else {
                    match app_id {
                        peak_core::registry::AppId::Terminal => "System Console".to_string(),
                        peak_core::registry::AppId::Library => "The Arcade".to_string(),
                        peak_core::registry::AppId::Turntable => "The Jukebox".to_string(),
                        peak_core::registry::AppId::Settings => "Core Sync".to_string(),
                        peak_core::registry::AppId::FileManager => "File System".to_string(),
                        peak_core::registry::AppId::Store => "App Store".to_string(),
                        peak_core::registry::AppId::Editor => "Simple Text".to_string(),
                        peak_core::registry::AppId::Browser => "Netscape Navigator".to_string(),
                        _ => "Application".to_string(),
                    }
                };
                let title = title.as_str();

                let on_close = match app_id {
                    peak_core::registry::AppId::Terminal => Message::ToggleTerminal,
                    peak_core::registry::AppId::Library => Message::ToggleArcade,
                    peak_core::registry::AppId::Turntable => Message::ToggleJukebox,
                    peak_core::registry::AppId::Settings => Message::ToggleSettings,
                    peak_core::registry::AppId::FileManager => Message::ToggleExplorer,
                    peak_core::registry::AppId::Store => Message::ToggleStore,
                    peak_core::registry::AppId::Editor => Message::ToggleEditor,
                    peak_core::registry::AppId::Browser => {
                        Message::LaunchBrowser("about:blank".into())
                    }
                    _ => Message::Exit,
                };

                let on_close = match app_id {
                    peak_core::registry::AppId::Browser => Message::CloseBrowser,
                    _ => on_close,
                };

                let mut win_x = state.x;
                let mut win_y = state.y.max(40.0); // Safe guard for menubar

                if self.is_desktop_revealed {
                    let screen_center_x = self.window_manager.screen_size.width / 2.0;
                    win_y = -state.height + 60.0;
                    win_x = screen_center_x - (state.width / 2.0);

                    if let Some(pos) = self
                        .window_manager
                        .z_order
                        .iter()
                        .position(|&id| id == app_id)
                    {
                        let offset = pos as f32 * 4.0;
                        win_x += offset;
                        win_y += offset;
                    }
                }

                workspace_stack = workspace_stack.push(
                    container(
                        container(window_chrome::view(
                            title,
                            content,
                            on_close,
                            Some(Message::Maximize(app_id)),
                        ))
                        .width(state.width)
                        .height(state.height),
                    )
                    .padding(iced::Padding {
                        top: win_y,
                        left: win_x,
                        ..Default::default()
                    }),
                );
            }
        }

        // Dock Logic Reuse (If we wanted to show dock in Desktop mode as well, e.g. for inspection)
        // But for now, we assume if we are Refactored, we DON'T show them here.
        // However, existing code logic for Pinned Apps calculation is deep in here.
        // ...
        // Simplification: We remove Dock/Bar from THIS view.

        let workspace =
            crate::components::desktop_container::view(&wallpaper_path, workspace_stack.into());

        let workspace_and_inspector = iced::widget::row![
            container(workspace)
                .width(Length::Fill)
                .height(Length::Fill),
            if self.inspector.is_visible {
                self.inspector.view(self.tokens).map(Message::Inspector)
            } else {
                container(iced::widget::Space::with_width(0)).into()
            }
        ];

        let mut final_view = Stack::new().push(workspace_and_inspector);

        // Menubar overlay (top) -- REMOVED for separate process
        // RESTORED for macOS/Windows (Integrated Mode)
        #[cfg(not(target_os = "linux"))]
        let mut final_view = final_view.push(
            container(menubar::view(self.tokens).map(Message::MenubarAction))
                .width(Length::Fill)
                .height(Length::Shrink)
                .align_y(iced::alignment::Vertical::Top),
        );

        // Dock overlay (bottom center) -- REMOVED for separate process
        // RESTORED for macOS/Windows (Integrated Mode)
        #[cfg(not(target_os = "linux"))]
        let mut final_view = {
            if self.dock_visible {
                // Duplicate Dock Data Logic for Integrated Mode
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
                    } else {
                        if !standard_pinned.contains(&id) {
                            standard_running.push(id);
                        }
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

                final_view.push(
                    container(dock_element)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center)
                        .align_y(iced::alignment::Vertical::Bottom)
                        .padding(iced::Padding {
                            bottom: 10.0,
                            ..Default::default()
                        }),
                )
            } else {
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
                                background: Some(self.tokens.background.into()),
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
            let text_color = tokens.text;
            let bg = tokens.glass_bg;

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
                            let mut c = tokens.accent;
                            c.a = 0.2;
                            c
                        } else if is_hovered {
                            let mut c = tokens.text;
                            c.a = 0.1;
                            c
                        } else {
                            Color::TRANSPARENT
                        };

                        button::Style {
                            background: Some(final_bg.into()),
                            text_color: if active { tokens.accent } else { tokens.text },
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
                        iced::widget::Space::with_height(5.0),
                        container(iced::widget::Space::with_height(0.5))
                            .width(Length::Fill)
                            .style(move |_| container::Style {
                                background: Some(
                                    Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.2)
                                        .into()
                                ),
                                ..Default::default()
                            }),
                        iced::widget::Space::with_height(5.0),
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
                            "Peak".into(),
                            Message::SwitchMode(ShellMode::Peak),
                            self.mode == ShellMode::Peak
                        ),
                        menu_button(
                            "Riviera".into(),
                            Message::SwitchMode(ShellMode::Poolside),
                            self.mode == ShellMode::Poolside
                        ),
                    ]
                    .width(Length::Fixed(140.0))
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
                    iced::widget::Space::with_height(5.0),
                    container(iced::widget::Space::with_height(0.5))
                        .width(Length::Fill)
                        .style(move |_| container::Style {
                            background: Some(
                                Color::from_rgba(text_color.r, text_color.g, text_color.b, 0.2)
                                    .into()
                            ),
                            ..Default::default()
                        }),
                    iced::widget::Space::with_height(5.0),
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
            let mut menu_col = Column::new().spacing(4).padding(4);

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
                        container(iced::widget::vertical_space())
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
