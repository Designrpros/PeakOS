// Desktop view rendering

use super::{Message, PeakNative};
use crate::components::{
    desktop_container, dock,
    menubar::{self, MenubarMessage},
};
use crate::pages::Page;
use crate::registry::ShellMode;
use iced::widget::{button, container, text, text as t, Column, Stack};
use iced::{Border, Color, Element, Length, Shadow, Vector};

// Helper function for app grid
// Helper function for app grid
fn view_app_grid(is_light: bool) -> Element<'static, Message> {
    let apps = vec![
        crate::registry::AppId::Terminal,
        crate::registry::AppId::Browser,
        crate::registry::AppId::Library,
        crate::registry::AppId::Store,
        crate::registry::AppId::Turntable,
        crate::registry::AppId::Settings,
        crate::registry::AppId::FileManager,
        crate::registry::AppId::Cortex,
        crate::registry::AppId::Editor,
        crate::registry::AppId::Antigravity,
    ];

    let grid = crate::components::app_grid::view(&apps, is_light).map(Message::DockInteraction);

    container(grid)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

impl PeakNative {
    pub fn view_desktop(&self) -> Element<'_, Message> {
        let is_light = self.settings.theme_mode == crate::apps::settings::ThemeMode::Light;
        let wallpaper_path = match (self.mode, is_light) {
            (ShellMode::Peak, true) => {
                crate::utils::assets::get_asset_path("wallpapers/mountain_classic_light.jpg")
            }
            (ShellMode::Peak, false) => {
                crate::utils::assets::get_asset_path("wallpapers/mountain_classic.jpg")
            }
            (ShellMode::Poolside, true) => {
                crate::utils::assets::get_asset_path("wallpapers/poolsuite_luxury.jpg")
            }
            (ShellMode::Poolside, false) => {
                crate::utils::assets::get_asset_path("wallpapers/poolsuite_luxury_night.jpg")
            }
        };

        use crate::components::window_chrome;

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

                let content: Element<'_, Message> = match app_id {
                    crate::registry::AppId::Terminal => {
                        self.terminal.view(is_light).map(Message::Terminal)
                    }
                    crate::registry::AppId::Library => {
                        self.library.view(&self.games).map(Message::Library)
                    }
                    crate::registry::AppId::Turntable => self
                        .jukebox
                        .view(&self.games, is_light)
                        .map(Message::Jukebox),
                    crate::registry::AppId::Settings => {
                        self.settings.view(is_light).map(Message::Settings)
                    }
                    crate::registry::AppId::FileManager => {
                        self.explorer.view(is_light).map(Message::Explorer)
                    }
                    crate::registry::AppId::Store => self.store.view(is_light).map(Message::Store),
                    crate::registry::AppId::Editor => {
                        self.editor.view(is_light).map(Message::Editor)
                    }
                    crate::registry::AppId::Browser => {
                        // Empty transparent container, as the real browser is overlaying here
                        container(iced::widget::Space::new(Length::Fill, Length::Fill)).into()
                    }
                    _ => container(iced::widget::text("UNSUPPORTED")).into(),
                };

                let title = match app_id {
                    crate::registry::AppId::Terminal => "System Console",
                    crate::registry::AppId::Library => "The Arcade",
                    crate::registry::AppId::Turntable => "The Jukebox",
                    crate::registry::AppId::Settings => "Core Sync",
                    crate::registry::AppId::FileManager => "File System",
                    crate::registry::AppId::Store => "App Store",
                    crate::registry::AppId::Editor => "Simple Text",
                    crate::registry::AppId::Browser => "Netscape Navigator",
                    _ => "Application",
                };

                let on_close = match app_id {
                    crate::registry::AppId::Terminal => Message::ToggleTerminal,
                    crate::registry::AppId::Library => Message::ToggleArcade,
                    crate::registry::AppId::Turntable => Message::ToggleJukebox,
                    crate::registry::AppId::Settings => Message::ToggleSettings,
                    crate::registry::AppId::FileManager => Message::ToggleExplorer,
                    crate::registry::AppId::Store => Message::ToggleStore,
                    crate::registry::AppId::Editor => Message::ToggleEditor,
                    crate::registry::AppId::Browser => Message::LaunchBrowser("about:blank".into()), // TODO: Better close logic for browser
                    _ => Message::Exit,
                };

                // Browser closing logic - actually we just want to remove it from view
                let on_close = if app_id == crate::registry::AppId::Browser {
                    // We need a specific CloseBrowser message ideally, but for now reuse toggle/close
                    // The browser process is kept alive for now per task 6
                    // But we should probably actually hide it.
                    Message::LaunchBrowser("about:blank".into()) // Placeholder
                } else {
                    on_close
                };

                // Correct on_close for browser:
                let on_close = match app_id {
                    crate::registry::AppId::Browser => Message::CloseBrowser,
                    _ => on_close,
                };

                let mut win_x = state.x;
                let mut win_y = state.y;

                if self.is_desktop_revealed {
                    // Centralize windows into a stack at the top edge of the screen
                    let screen_center_x = self.window_manager.screen_size.width / 2.0;

                    // Windows should be mostly hidden at the top, just a small sliver visible
                    win_y = -state.height + 60.0; // Increased from 20 to 60 for better visibility
                    win_x = screen_center_x - (state.width / 2.0);

                    // Add a slight stack offset to make multiple windows "slightly visible" as a stack
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
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(iced::Padding {
                        top: win_y,
                        left: win_x,
                        ..Default::default()
                    })
                    .align_x(iced::alignment::Horizontal::Left)
                    .align_y(iced::alignment::Vertical::Top),
                );
            }
        }

        let main_content = container(workspace_stack)
            .width(Length::Fill)
            .height(Length::Fill);

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

        for &id in &self.running_apps {
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

        // Final list for dock categorization
        let pinned = standard_pinned;
        let running = standard_running;
        let mut repos = repo_pinned;
        for r in repo_running {
            if !repos.contains(&r) {
                repos.push(r);
            }
        }

        let dock_element = dock::view(
            &pinned,
            &running,
            &repos,
            self.dragging_app,
            self.context_menu_app,
            is_light,
            self.mode,
        )
        .map(Message::DockInteraction);

        // if self.dragging_app.is_some() {
        //     dock_element = iced::widget::opacity(dock_element, 0.5).into();
        // }

        let global_ui = Stack::new()
            .push(
                container(main_content)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(
                container(if self.dock_visible {
                    Element::from(dock_element)
                } else {
                    iced::widget::Space::with_height(1.0).into()
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .padding(20)
                .align_y(iced::alignment::Vertical::Bottom),
            );

        let main_ui = desktop_container::view(&wallpaper_path, global_ui.into());

        let mut main_stack = iced::widget::Stack::new().push(main_ui);

        if self.show_omnibar {
            main_stack = main_stack.push(
                container(self.omnibar.view(is_light).map(Message::Omnibar))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(100.0),
            );
        }

        if self.show_app_grid {
            main_stack = main_stack
                .push(
                    container(
                        // Backdrop
                        button(text(""))
                            .style(move |_, _| button::Style {
                                background: Some(if is_light {
                                    iced::Color::from_rgb8(242, 242, 247).into()
                                // Solid Light
                                } else {
                                    iced::Color::from_rgb8(20, 20, 20).into() // Solid Dark
                                }),
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
                    container(view_app_grid(is_light))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill),
                );
        }

        if self.show_switcher {
            main_stack = main_stack
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
                    container(self.switcher.view(is_light).map(Message::Switcher))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill),
                );
        }

        if self.show_spaces_selector {
            main_stack = main_stack.push(
                container(
                    crate::components::spaces_strip::view(self.mode, self.current_desktop)
                        .map(Message::SwitchSpace),
                )
                .width(Length::Fill)
                .padding(iced::Padding {
                    top: 50.0,
                    ..Default::default()
                }) // 40px menubar + 10px breathing
                .center_x(Length::Fill)
                .align_y(iced::alignment::Vertical::Top),
            );
        }

        if self.show_system_menu || self.show_reality_menu || self.show_wifi_menu {
            let (text_color, bg) = match self.mode {
                ShellMode::Peak => (
                    Color::from_rgb8(35, 30, 30),    // Warm Deep Grey
                    Color::from_rgb8(247, 245, 242), // Warm Stone Paper
                ),
                ShellMode::Poolside => (
                    Color::from_rgb8(50, 50, 50),    // Retro Dark Grey Text
                    Color::from_rgb8(255, 153, 204), // Retro Pink Bar
                ),
            };

            let menu_button = |label: String, msg: Message, active: bool| -> Element<'_, Message> {
                button(
                    container(text(label).size(13))
                        .width(Length::Fill)
                        .padding([5, 10]),
                )
                .on_press(msg)
                .style(move |_, status| {
                    let is_hovered = status == iced::widget::button::Status::Hovered;
                    button::Style {
                        background: if active {
                            // Warm gray for active state
                            if is_light {
                                Some(Color::from_rgb8(220, 220, 220).into()) // Warm light gray
                            } else {
                                Some(Color::from_rgb8(60, 60, 60).into()) // Warm dark gray
                            }
                        } else if is_hovered {
                            Some(Color::from_rgba(1.0, 1.0, 1.0, 0.1).into())
                        } else {
                            None
                        },
                        text_color: if active {
                            if is_light {
                                Color::BLACK
                            } else {
                                Color::WHITE
                            }
                        } else {
                            text_color
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
                        menu_button("Shut Down...".into(), Message::Exit, false), // Uses exit(0) for now
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

                main_stack = main_stack.push(
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

                main_stack = main_stack.push(
                    container(menu)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(iced::Padding {
                            top: 40.0,
                            left: 60.0, // Positioned under the mode name
                            ..Default::default()
                        })
                        .align_x(iced::alignment::Horizontal::Left)
                        .align_y(iced::alignment::Vertical::Top),
                );
            }

            if self.show_wifi_menu {
                // Header
                let mut content = iced::widget::column![
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

                // Network List
                for network in &self.networks {
                    content = content.push(
                        menu_button(
                            network.clone(),
                            Message::MenubarAction(MenubarMessage::ToggleWifiMenu),
                            network == "PeakOS_5G",
                        ), // Mock active
                    );
                }

                let menu =
                    container(content.width(Length::Fixed(200.0)).padding(10)).style(move |_| {
                        container::Style {
                            background: Some(Color::from_rgba(bg.r, bg.g, bg.b, 0.92).into()),
                            border: iced::Border {
                                color: Color::from_rgba(
                                    text_color.r,
                                    text_color.g,
                                    text_color.b,
                                    0.1,
                                ),
                                width: 1.0,
                                radius: 12.0.into(),
                            },
                            shadow: iced::Shadow {
                                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                                offset: iced::Vector::new(0.0, 8.0),
                                blur_radius: 16.0,
                            },
                            ..Default::default()
                        }
                    });

                main_stack = main_stack.push(
                    container(menu)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(iced::Padding {
                            top: 40.0,
                            right: 10.0, // Right aligned
                            ..Default::default()
                        })
                        .align_x(iced::alignment::Horizontal::Right)
                        .align_y(iced::alignment::Vertical::Top),
                );
            }
        }

        if self.inspector.is_visible {
            main_stack = main_stack.push(
                container(self.inspector.view(is_light).map(Message::Inspector))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Right)
                    .align_y(iced::alignment::Vertical::Top)
                    .padding(iced::Padding {
                        top: 0.0,
                        right: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                    }),
            );
        }

        // Always render Menubar on top of everything
        main_stack = main_stack.push(
            container(menubar::view(self.mode, is_light).map(Message::MenubarAction))
                .width(Length::Fill)
                .height(Length::Shrink) // Only take required height (don't block screen)
                .align_y(iced::alignment::Vertical::Top),
        );

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
                        text_color: Color::from_rgb(0.8, 0.2, 0.2), // Red for quit
                        ..Default::default()
                    }),
            );

            let _is_dragging = self.dragging_app.is_some();
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

            main_stack = main_stack.push(
                iced::widget::mouse_area(
                    container(
                        iced::widget::mouse_area(container(context_menu_element).padding(2))
                            .on_press(Message::Tick), // Prevent click-through for menu itself
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

        main_stack.into()
    }
}
