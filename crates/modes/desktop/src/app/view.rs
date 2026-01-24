// Main view rendering

use super::{AppState, Message, PeakNative};
use iced::widget::{button, container, text, text as t, Stack};
use iced::{Color, Element, Length};

impl PeakNative {
    pub fn view(&self) -> Element<'_, Message> {
        // FIXED: Subprocesses (Bar/Dock) must always render their specific views,
        // ignoring the AppState (which defaults to Setup on fresh boot).
        if self.launch_mode != crate::app::LaunchMode::Desktop {
            return self.view_desktop();
        }

        let content = match &self.state {
            AppState::Setup(wizard_state) => self.view_setup(wizard_state),
            AppState::Login(_) => self.view_login_new(),
            AppState::Desktop => self.view_desktop(),
        };

        if let Some((title, body)) = &self.alert {
            iced::widget::stack![
                content,
                iced::widget::container(peak_ui::alert::SystemAlert::view(
                    title.clone(),
                    body.clone(),
                    Message::CloseAlert,
                    matches!(self.theme, peak_core::Theme::Light)
                ))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)
                .style(|_theme| {
                    iced::widget::container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.0, 0.0, 0.0, 0.5,
                        ))),
                        ..Default::default()
                    }
                })
            ]
            .into()
        } else {
            content
        }
    }

    fn view_login_new(&self) -> Element<'_, Message> {
        let user_name = self
            .user
            .as_ref()
            .map(|u| u.full_name.clone())
            .unwrap_or("User".to_string());

        let is_light = matches!(self.theme, peak_core::theme::Theme::Light);
        let text_color = if is_light { Color::BLACK } else { Color::WHITE };

        // Theme Toggle SVG Handle
        let toggle_icon = if is_light {
            // Moon (Black for Light Mode)
            iced::widget::svg::Handle::from_memory(br#"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                </svg>
            "#.to_vec())
        } else {
            // Sun (White for Dark Mode)
            iced::widget::svg::Handle::from_memory(br#"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/>
                    <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
                    <line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/>
                    <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
                </svg>
            "#.to_vec())
        };

        let theme_btn = button(
            iced::widget::svg(toggle_icon)
                .width(Length::Fixed(24.0))
                .height(Length::Fixed(24.0)),
        )
        .on_press(Message::ToggleTheme)
        .padding(10)
        .style(move |_, status| {
            let bg_alpha = if status == iced::widget::button::Status::Hovered {
                0.2
            } else {
                0.0
            };
            iced::widget::button::Style {
                background: Some(Color::from_rgba(0.5, 0.5, 0.5, bg_alpha).into()),
                ..Default::default()
            }
        });

        let top_right = container(theme_btn)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right)
            .align_y(iced::alignment::Vertical::Top)
            .padding(20);

        // 1. Logo (Top Left)
        let top_left = container(
            iced::widget::image(iced::widget::image::Handle::from_path(
                peak_core::utils::assets::get_asset_path(&format!(
                    "icons/menubar/{}",
                    if is_light {
                        "peak_logo.png"
                    } else {
                        "peak_logo_dark.png"
                    }
                )),
            ))
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(50.0)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::alignment::Horizontal::Left)
        .align_y(iced::alignment::Vertical::Top)
        .padding(20);

        // 2. Main Login Content (No Avatar)
        let content = iced::widget::column![
            // Name
            text(user_name)
                .size(32)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .style(move |_| t::Style {
                    color: Some(text_color)
                }),
            iced::widget::Space::with_height(30.0),
            // Password Input
            iced::widget::text_input(
                "Enter Password",
                if let AppState::Login(ref p) = self.state {
                    p
                } else {
                    ""
                }
            )
            .on_input(Message::UpdateLoginPassword)
            .on_submit(Message::SubmitLogin)
            .secure(true)
            .width(Length::Fixed(280.0))
            .padding(15)
            .style(move |_, status| peak_core::styles::style_soft_input(status, is_light)),
            iced::widget::Space::with_height(30.0),
            // Login Button
            button(
                container(text("Login").size(16))
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center)
            )
            .on_press(Message::SubmitLogin)
            .padding([12, 50])
            .width(Length::Fixed(280.0)) // Match input width
            .style(move |_, status| peak_core::styles::style_pill_button(status, is_light))
        ]
        .align_x(iced::Alignment::Center)
        .spacing(0);

        // WRAP IN GLASS CARD
        let card = container(content)
            .padding(60) // Increased padding for cleaner look without avatar
            .style(move |_| peak_core::styles::style_glass_card(is_light));

        let centered_content = container(card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        // Wallpaper + Blur Overlay
        let wallpaper_path =
            peak_core::utils::assets::get_asset_path("wallpapers/mountain_sunset_warm.jpg");

        Stack::new()
            .push(
                container(
                    iced::widget::image(iced::widget::image::Handle::from_path(wallpaper_path))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(iced::ContentFit::Cover),
                )
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .push(
                container(iced::widget::Space::new(Length::Fill, Length::Fill))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(move |_theme: &iced::Theme| container::Style {
                        background: Some(
                            if is_light {
                                Color::from_rgba(0.9, 0.9, 0.95, 0.3)
                            } else {
                                Color::from_rgba(0.0, 0.0, 0.0, 0.5)
                            }
                            .into(),
                        ),
                        ..Default::default()
                    }),
            )
            .push(centered_content)
            .push(top_right)
            .push(top_left)
            .into()
    }

    fn view_setup<'a>(&'a self, state: &'a peak_apps::wizard::WizardState) -> Element<'a, Message> {
        let is_light = matches!(self.theme, peak_core::theme::Theme::Light);

        // --- Steps ---

        let content = match state.current_step {
            peak_apps::wizard::WizardStep::Welcome => iced::widget::column![
                iced::widget::image(iced::widget::image::Handle::from_path(
                    peak_core::utils::assets::get_asset_path(&format!(
                        "icons/menubar/{}",
                        if is_light {
                            "peak_logo.png"
                        } else {
                            "peak_logo_dark.png"
                        }
                    ))
                ))
                .width(Length::Fixed(180.0))
                .height(Length::Fixed(90.0)),
                text("Welcome to Peak.")
                    .size(36)
                    .font(iced::Font {
                        family: iced::font::Family::SansSerif,
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .style(move |_| t::Style {
                        color: Some(if is_light {
                            Color::from_rgb8(26, 26, 26)
                        } else {
                            Color::WHITE
                        })
                    }),
                text("Let's set up your new home.")
                    .size(18)
                    .style(move |_| t::Style {
                        color: Some(Color::from_rgb8(102, 102, 102)) // #666
                    }),
                iced::widget::Space::with_height(40.0),
                iced::widget::button(text("Get Started").size(16))
                    .on_press(Message::Wizard(peak_apps::wizard::WizardMessage::NextStep))
                    .padding([15, 60])
                    .style(move |_, status| peak_core::styles::style_pill_button(status, is_light))
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
            peak_apps::wizard::WizardStep::Identity => {
                iced::widget::column![
                    text("Who are you?")
                        .size(24)
                        .font(iced::Font {
                            family: iced::font::Family::SansSerif,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(move |_| t::Style {
                            color: Some(if is_light {
                                Color::from_rgb8(26, 26, 26)
                            } else {
                                Color::WHITE
                            })
                        }),
                    iced::widget::Space::with_height(20.0),
                    // Group 1: Identity
                    iced::widget::column![
                        text("Full Name").size(12).style(move |_| {
                            t::Style {
                                color: Some(Color::from_rgb8(120, 120, 120)),
                            }
                        }),
                        iced::widget::text_input("John Appleseed", &state.full_name_input)
                            .on_input(|s| Message::Wizard(
                                peak_apps::wizard::WizardMessage::UpdateFullName(s)
                            ))
                            .padding(12)
                            .style(move |_, status| peak_core::styles::style_soft_input(
                                status, is_light
                            )),
                        iced::widget::Space::with_height(10.0),
                        text("Account Name").size(12).style(move |_| {
                            t::Style {
                                color: Some(Color::from_rgb8(120, 120, 120)),
                            }
                        }),
                        iced::widget::text_input("john", &state.username_input)
                            .on_input(|s| Message::Wizard(
                                peak_apps::wizard::WizardMessage::UpdateUsername(s)
                            ))
                            .padding(12)
                            .style(move |_, status| peak_core::styles::style_soft_input(
                                status, is_light
                            )),
                    ]
                    .spacing(5),
                    iced::widget::Space::with_height(20.0),
                    // Error Message
                    if let Some(error) = &state.error_message {
                        text(error.clone()).size(14).style(move |_| t::Style {
                            color: Some(Color::from_rgb8(255, 59, 48)), // System Red
                        })
                    } else {
                        text(" ").size(14)
                    },
                    iced::widget::Space::with_height(20.0),
                    // Buttons
                    iced::widget::row![
                        iced::widget::button(text("Back").size(16))
                            .on_press(Message::Wizard(peak_apps::wizard::WizardMessage::PrevStep))
                            .style(move |_, status| peak_core::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Next").size(16))
                            .on_press(Message::Wizard(peak_apps::wizard::WizardMessage::NextStep))
                            .padding([12, 40])
                            .style(move |_, status| peak_core::styles::style_pill_button(
                                status, is_light
                            ))
                    ]
                    .spacing(20)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::alignment::Horizontal::Center)
            }
            peak_apps::wizard::WizardStep::Security => {
                iced::widget::column![
                    text("Secure your account")
                        .size(24)
                        .font(iced::Font {
                            family: iced::font::Family::SansSerif,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(move |_| t::Style {
                            color: Some(if is_light {
                                Color::from_rgb8(26, 26, 26)
                            } else {
                                Color::WHITE
                            })
                        }),
                    iced::widget::Space::with_height(20.0),
                    // Group 2: Security
                    iced::widget::column![
                        text("Password").size(12).style(move |_| t::Style {
                            color: Some(Color::from_rgb8(120, 120, 120))
                        }),
                        iced::widget::text_input("Required", &state.password_input)
                            .on_input(|s| Message::Wizard(
                                peak_apps::wizard::WizardMessage::UpdatePassword(s)
                            ))
                            .padding(12)
                            .secure(true)
                            .style(move |_, status| peak_core::styles::style_soft_input(
                                status, is_light
                            )),
                        iced::widget::Space::with_height(10.0),
                        text("Verify").size(12).style(move |_| t::Style {
                            color: Some(Color::from_rgb8(120, 120, 120))
                        }),
                        iced::widget::text_input("Required", &state.password_confirm_input)
                            .on_input(|s| Message::Wizard(
                                peak_apps::wizard::WizardMessage::UpdatePasswordConfirm(s)
                            ))
                            .padding(12)
                            .secure(true)
                            .style(move |_, status| peak_core::styles::style_soft_input(
                                status, is_light
                            )),
                        iced::widget::Space::with_height(10.0),
                        text("Hint").size(12).style(move |_| t::Style {
                            color: Some(Color::from_rgb8(120, 120, 120))
                        }),
                        iced::widget::text_input("Optional", &state.password_hint_input)
                            .on_input(|s| Message::Wizard(
                                peak_apps::wizard::WizardMessage::UpdatePasswordHint(s)
                            ))
                            .padding(12)
                            .style(move |_, status| peak_core::styles::style_soft_input(
                                status, is_light
                            )),
                    ]
                    .spacing(5),
                    iced::widget::Space::with_height(20.0),
                    // Error Message
                    if let Some(error) = &state.error_message {
                        text(error.clone()).size(14).style(move |_| t::Style {
                            color: Some(Color::from_rgb8(255, 59, 48)), // System Red
                        })
                    } else {
                        text(" ").size(14)
                    },
                    iced::widget::Space::with_height(20.0),
                    // Buttons
                    iced::widget::row![
                        iced::widget::button(text("Back").size(16))
                            .on_press(Message::Wizard(peak_apps::wizard::WizardMessage::PrevStep))
                            .style(move |_, status| peak_core::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Next").size(16))
                            .on_press(Message::Wizard(peak_apps::wizard::WizardMessage::NextStep))
                            .padding([12, 40])
                            .style(move |_, status| peak_core::styles::style_pill_button(
                                status, is_light
                            ))
                    ]
                    .spacing(20)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::alignment::Horizontal::Center)
            }
            peak_apps::wizard::WizardStep::WifiConnect => {
                iced::widget::column![
                    text("Get Connected")
                        .size(24)
                        .font(iced::Font {
                            family: iced::font::Family::SansSerif,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(move |_| t::Style {
                            color: Some(if is_light {
                                Color::from_rgb8(26, 26, 26)
                            } else {
                                Color::WHITE
                            })
                        }),
                    text("Select a network to continue.")
                        .size(14)
                        .style(move |_| t::Style {
                            color: Some(Color::from_rgb8(102, 102, 102))
                        }),
                    iced::widget::Space::with_height(20.0),
                    // Valid Network List (Table View)
                    container(iced::widget::scrollable(iced::widget::column(
                        peak_core::integrations::network::get_available_networks()
                            .into_iter()
                            .enumerate()
                            .map(|(i, net)| {
                                let is_sel = Some(net.clone()) == state.selected_network;
                                let content = iced::widget::button(
                                    iced::widget::row![
                                        iced::widget::svg(peak_core::icons::get_status_icon(
                                            "wifi",
                                            if is_sel {
                                                if is_light {
                                                    "#000000"
                                                } else {
                                                    "#FFFFFF"
                                                }
                                            } else {
                                                "#888888"
                                            }
                                        ))
                                        .width(Length::Fixed(16.0))
                                        .height(Length::Fixed(16.0)),
                                        text(net.clone()).size(14),
                                        iced::widget::horizontal_space(),
                                        // Lock icon placeholder
                                        // iced::widget::svg(peak_core::icons::lock(...)),
                                        if is_sel {
                                            text("✓").size(14)
                                        } else {
                                            text("").size(14)
                                        }
                                    ]
                                    .spacing(10)
                                    .align_y(iced::Alignment::Center),
                                )
                                .on_press(Message::Wizard(
                                    peak_apps::wizard::WizardMessage::SelectNetwork(net),
                                ))
                                .width(Length::Fill)
                                .padding(12)
                                .style(move |_, status| {
                                    if is_sel {
                                        iced::widget::button::Style {
                                            background: Some(if is_light {
                                                Color::BLACK.into()
                                            } else {
                                                Color::WHITE.into()
                                            }),
                                            text_color: if is_light {
                                                Color::WHITE
                                            } else {
                                                Color::BLACK
                                            },
                                            ..Default::default()
                                        }
                                    } else if status == iced::widget::button::Status::Hovered {
                                        iced::widget::button::Style {
                                            background: Some(if is_light {
                                                Color::from_rgba(0.0, 0.0, 0.0, 0.05).into()
                                            } else {
                                                Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()
                                            }),
                                            text_color: if is_light {
                                                Color::BLACK
                                            } else {
                                                Color::WHITE
                                            },
                                            ..Default::default()
                                        }
                                    } else {
                                        iced::widget::button::Style {
                                            background: Some(Color::TRANSPARENT.into()),
                                            text_color: if is_light {
                                                Color::BLACK
                                            } else {
                                                Color::WHITE
                                            },
                                            ..Default::default()
                                        }
                                    }
                                });

                                if i > 0 {
                                    iced::widget::column![
                                        iced::widget::Rule::horizontal(1).style(move |_| {
                                            iced::widget::rule::Style {
                                                color: Color::from_rgba(0.5, 0.5, 0.5, 0.2),
                                                width: 1,
                                                radius: 0.0.into(),
                                                fill_mode: iced::widget::rule::FillMode::Full,
                                            }
                                        }),
                                        content
                                    ]
                                    .into()
                                } else {
                                    content.into()
                                }
                            })
                            .collect::<Vec<_>>()
                    )))
                    .height(Length::Fixed(200.0))
                    .style(move |_| container::Style {
                        background: Some(Color::WHITE.into()),
                        border: iced::Border {
                            radius: 12.0.into(),
                            width: 1.0,
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        },
                        ..Default::default()
                    }),
                    iced::widget::Space::with_height(20.0),
                    // Wifi Password & Actions
                    if state.selected_network.is_some() {
                        Element::new(
                            iced::widget::column![
                            Element::new(
                                text::<iced::Theme, iced::Renderer>("Password")
                                    .size(12)
                                    .style(move |_theme: &iced::Theme| t::Style {
                                        color: Some(Color::from_rgb8(120, 120, 120))
                                    })
                            ),
                            Element::new(
                                iced::widget::text_input::<Message, iced::Theme, iced::Renderer>(
                                    "Wifi Password",
                                    &state.wifi_password_input
                                )
                                .on_input(|s| Message::Wizard(
                                    peak_apps::wizard::WizardMessage::UpdateWifiPassword(s)
                                ))
                                .padding(12)
                                .secure(true)
                                .style(
                                    move |_theme: &iced::Theme, status| {
                                        peak_core::styles::style_soft_input(status, is_light)
                                    }
                                )
                            ),
                            Element::new(
                                iced::widget::button::<Message, iced::Theme, iced::Renderer>(
                                    container(text("Join Network").size(14))
                                        .width(Length::Fill)
                                        .align_x(iced::alignment::Horizontal::Center)
                                )
                                .on_press(Message::Wizard(
                                    peak_apps::wizard::WizardMessage::NextStep
                                )) // Proceed to theme after connecting
                                .padding(12)
                                .width(Length::Fill)
                                .style(
                                    move |_theme: &iced::Theme, status| {
                                        peak_core::styles::style_pill_button(status, is_light)
                                    }
                                )
                            ),
                        ]
                            .spacing(10),
                        )
                    } else {
                        Element::new(
                            iced::widget::text::<iced::Theme, iced::Renderer>(
                                "Select a network to connect.",
                            )
                            .size(12)
                            .style(move |_theme: &iced::Theme| t::Style {
                                color: Some(Color::from_rgba(0.5, 0.5, 0.5, 0.7)),
                            }),
                        )
                    },
                    iced::widget::Space::with_height(30.0),
                    iced::widget::row![
                        iced::widget::button(text("Back").size(14))
                            .on_press(Message::Wizard(peak_apps::wizard::WizardMessage::PrevStep))
                            .style(move |_, status| peak_core::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Next").size(14))
                            .on_press(Message::Wizard(peak_apps::wizard::WizardMessage::NextStep))
                            .padding([12, 40])
                            .style(move |_, status| peak_core::styles::style_pill_button(
                                status, is_light
                            )),
                    ]
                    .spacing(20)
                    .align_y(iced::Alignment::Center)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::Alignment::Center)
            }
            peak_apps::wizard::WizardStep::ThemeSelection => {
                iced::widget::column![
                    text("Personalize")
                        .size(24)
                        .font(iced::Font {
                            family: iced::font::Family::SansSerif,
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(move |_| t::Style {
                            color: Some(if is_light {
                                Color::from_rgb8(26, 26, 26)
                            } else {
                                Color::WHITE
                            })
                        }),
                    text("Choose your experience.")
                        .size(14)
                        .style(move |_| t::Style {
                            color: Some(Color::from_rgb8(102, 102, 102))
                        }),
                    iced::widget::Space::with_height(20.0),
                    // Mode Section
                    text("Mode").size(12).style(move |_| t::Style {
                        color: Some(Color::from_rgb8(120, 120, 120))
                    }),
                    iced::widget::row![
                        // Desktop
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(peak_core::icons::get_status_icon(
                                    "image",
                                    if state.selected_mode.as_deref() == Some("desktop") {
                                        "#007AFF"
                                    } else {
                                        "#888888"
                                    }
                                ))
                                .width(Length::Fixed(24.0))
                                .height(Length::Fixed(24.0)),
                                text("Desktop").size(12)
                            ]
                            .align_x(iced::Alignment::Center)
                            .spacing(5)
                        )
                        .on_press(Message::Wizard(
                            peak_apps::wizard::WizardMessage::SelectMode("desktop".to_string())
                        ))
                        .width(Length::Fixed(80.0))
                        .padding(10)
                        .style(move |_, _| {
                            let is_sel = state.selected_mode.as_deref() == Some("desktop");
                            iced::widget::button::Style {
                                background: if is_sel {
                                    Some(Color::from_rgba(0.0, 0.48, 1.0, 0.1).into())
                                } else {
                                    None
                                },
                                border: iced::Border {
                                    radius: 12.0.into(),
                                    width: if is_sel { 2.0 } else { 1.0 },
                                    color: if is_sel {
                                        Color::from_rgba(0.0, 0.48, 1.0, 1.0)
                                    } else {
                                        Color::from_rgba(0.5, 0.5, 0.5, 0.2)
                                    },
                                },
                                ..Default::default()
                            }
                        }),
                        // Mobile
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(peak_core::icons::get_status_icon(
                                    "smartphone",
                                    if state.selected_mode.as_deref() == Some("mobile") {
                                        "#007AFF"
                                    } else {
                                        "#888888"
                                    }
                                ))
                                .width(Length::Fixed(24.0))
                                .height(Length::Fixed(24.0)),
                                text("Mobile").size(12)
                            ]
                            .align_x(iced::Alignment::Center)
                            .spacing(5)
                        )
                        .on_press(Message::Wizard(
                            peak_apps::wizard::WizardMessage::SelectMode("mobile".to_string())
                        ))
                        .width(Length::Fixed(80.0))
                        .padding(10)
                        .style(move |_, _| {
                            let is_sel = state.selected_mode.as_deref() == Some("mobile");
                            iced::widget::button::Style {
                                background: if is_sel {
                                    Some(Color::from_rgba(0.0, 0.48, 1.0, 0.1).into())
                                } else {
                                    None
                                },
                                border: iced::Border {
                                    radius: 12.0.into(),
                                    width: if is_sel { 2.0 } else { 1.0 },
                                    color: if is_sel {
                                        Color::from_rgba(0.0, 0.48, 1.0, 1.0)
                                    } else {
                                        Color::from_rgba(0.5, 0.5, 0.5, 0.2)
                                    },
                                },
                                ..Default::default()
                            }
                        }),
                        // Console
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(peak_core::icons::get_status_icon(
                                    "monitor",
                                    if state.selected_mode.as_deref() == Some("console") {
                                        "#007AFF"
                                    } else {
                                        "#888888"
                                    }
                                ))
                                .width(Length::Fixed(24.0))
                                .height(Length::Fixed(24.0)),
                                text("Console").size(12)
                            ]
                            .align_x(iced::Alignment::Center)
                            .spacing(5)
                        )
                        .on_press(Message::Wizard(
                            peak_apps::wizard::WizardMessage::SelectMode("console".to_string())
                        ))
                        .width(Length::Fixed(80.0))
                        .padding(10)
                        .style(move |_, _| {
                            let is_sel = state.selected_mode.as_deref() == Some("console");
                            iced::widget::button::Style {
                                background: if is_sel {
                                    Some(Color::from_rgba(0.0, 0.48, 1.0, 0.1).into())
                                } else {
                                    None
                                },
                                border: iced::Border {
                                    radius: 12.0.into(),
                                    width: if is_sel { 2.0 } else { 1.0 },
                                    color: if is_sel {
                                        Color::from_rgba(0.0, 0.48, 1.0, 1.0)
                                    } else {
                                        Color::from_rgba(0.5, 0.5, 0.5, 0.2)
                                    },
                                },
                                ..Default::default()
                            }
                        }),
                    ]
                    .spacing(10),
                    iced::widget::Space::with_height(20.0),
                    // Theme Section
                    text("Theme").size(12).style(move |_| t::Style {
                        color: Some(Color::from_rgba(0.5, 0.5, 0.5, 0.8))
                    }),
                    iced::widget::row![
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(peak_core::icons::get_status_icon(
                                    "sun",
                                    if is_light { "#007AFF" } else { "#888888" }
                                ))
                                .width(Length::Fixed(24.0))
                                .height(Length::Fixed(24.0)),
                                text("Light").size(12)
                            ]
                            .align_x(iced::Alignment::Center)
                            .spacing(5)
                        )
                        .on_press(Message::ToggleTheme)
                        .width(Length::Fixed(80.0))
                        .padding(10)
                        .style(move |_, _| iced::widget::button::Style {
                            background: if is_light {
                                Some(Color::from_rgba(0.0, 0.48, 1.0, 0.1).into())
                            } else {
                                Some(Color::from_rgba(0.5, 0.5, 0.5, 0.1).into())
                            },
                            border: iced::Border {
                                radius: 12.0.into(),
                                width: if is_light { 2.0 } else { 0.0 },
                                color: Color::from_rgba(0.0, 0.48, 1.0, 1.0)
                            },
                            ..Default::default()
                        }),
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(peak_core::icons::get_status_icon(
                                    "moon",
                                    if !is_light { "#007AFF" } else { "#888888" }
                                ))
                                .width(Length::Fixed(24.0))
                                .height(Length::Fixed(24.0)),
                                text("Dark").size(12)
                            ]
                            .align_x(iced::Alignment::Center)
                            .spacing(5)
                        )
                        .on_press(Message::ToggleTheme)
                        .width(Length::Fixed(80.0))
                        .padding(10)
                        .style(move |_, _| iced::widget::button::Style {
                            background: if !is_light {
                                Some(Color::from_rgba(0.0, 0.48, 1.0, 0.1).into())
                            } else {
                                Some(Color::from_rgba(0.5, 0.5, 0.5, 0.1).into())
                            },
                            border: iced::Border {
                                radius: 12.0.into(),
                                width: if !is_light { 2.0 } else { 0.0 },
                                color: Color::from_rgba(0.0, 0.48, 1.0, 1.0)
                            },
                            ..Default::default()
                        }),
                    ]
                    .spacing(10),
                    iced::widget::Space::with_height(20.0),
                    // Shell Style Section
                    text("Shell Style").size(12).style(move |_| t::Style {
                        color: Some(Color::from_rgba(0.5, 0.5, 0.5, 0.8))
                    }),
                    iced::widget::row![
                        // Cupertino
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(peak_core::icons::get_status_icon(
                                    "layout",
                                    if state.selected_theme.as_deref() == Some("cupertino") {
                                        "#007AFF"
                                    } else {
                                        "#888888"
                                    }
                                ))
                                .width(Length::Fixed(24.0))
                                .height(Length::Fixed(24.0)),
                                text("Cupertino").size(12)
                            ]
                            .align_x(iced::Alignment::Center)
                            .spacing(5)
                        )
                        .on_press(Message::Wizard(
                            peak_apps::wizard::WizardMessage::SelectTheme("cupertino".to_string())
                        ))
                        .width(Length::Fixed(80.0))
                        .padding(10)
                        .style(move |_, _| {
                            let is_sel = state.selected_theme.as_deref() == Some("cupertino");
                            iced::widget::button::Style {
                                background: if is_sel {
                                    Some(Color::from_rgba(0.0, 0.48, 1.0, 0.1).into())
                                } else {
                                    None
                                },
                                border: iced::Border {
                                    radius: 12.0.into(),
                                    width: if is_sel { 2.0 } else { 1.0 },
                                    color: if is_sel {
                                        Color::from_rgba(0.0, 0.48, 1.0, 1.0)
                                    } else {
                                        Color::from_rgba(0.5, 0.5, 0.5, 0.2)
                                    },
                                },
                                ..Default::default()
                            }
                        }),
                        // Redmond
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(peak_core::icons::get_status_icon(
                                    "grid",
                                    if state.selected_theme.as_deref() == Some("redmond") {
                                        "#007AFF"
                                    } else {
                                        "#888888"
                                    }
                                ))
                                .width(Length::Fixed(24.0))
                                .height(Length::Fixed(24.0)),
                                text("Redmond").size(12)
                            ]
                            .align_x(iced::Alignment::Center)
                            .spacing(5)
                        )
                        .on_press(Message::Wizard(
                            peak_apps::wizard::WizardMessage::SelectTheme("redmond".to_string())
                        ))
                        .width(Length::Fixed(80.0))
                        .padding(10)
                        .style(move |_, _| {
                            let is_sel = state.selected_theme.as_deref() == Some("redmond");
                            iced::widget::button::Style {
                                background: if is_sel {
                                    Some(Color::from_rgba(0.0, 0.48, 1.0, 0.1).into())
                                } else {
                                    None
                                },
                                border: iced::Border {
                                    radius: 12.0.into(),
                                    width: if is_sel { 2.0 } else { 1.0 },
                                    color: if is_sel {
                                        Color::from_rgba(0.0, 0.48, 1.0, 1.0)
                                    } else {
                                        Color::from_rgba(0.5, 0.5, 0.5, 0.2)
                                    },
                                },
                                ..Default::default()
                            }
                        }),
                        // AI
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(peak_core::icons::get_status_icon(
                                    "cpu",
                                    if state.selected_theme.as_deref() == Some("ai") {
                                        "#007AFF"
                                    } else {
                                        "#888888"
                                    }
                                ))
                                .width(Length::Fixed(24.0))
                                .height(Length::Fixed(24.0)),
                                text("Peak AI").size(12)
                            ]
                            .align_x(iced::Alignment::Center)
                            .spacing(5)
                        )
                        .on_press(Message::Wizard(
                            peak_apps::wizard::WizardMessage::SelectTheme("ai".to_string())
                        ))
                        .width(Length::Fixed(80.0))
                        .padding(10)
                        .style(move |_, _| {
                            let is_sel = state.selected_theme.as_deref() == Some("ai");
                            iced::widget::button::Style {
                                background: if is_sel {
                                    Some(Color::from_rgba(0.0, 0.48, 1.0, 0.1).into())
                                } else {
                                    None
                                },
                                border: iced::Border {
                                    radius: 12.0.into(),
                                    width: if is_sel { 2.0 } else { 1.0 },
                                    color: if is_sel {
                                        Color::from_rgba(0.0, 0.48, 1.0, 1.0)
                                    } else {
                                        Color::from_rgba(0.5, 0.5, 0.5, 0.2)
                                    },
                                },
                                ..Default::default()
                            }
                        }),
                    ]
                    .spacing(10),
                    iced::widget::Space::with_height(20.0),
                    // Time Picker (Visual Only)
                    text("Time Zone").size(12).style(move |_| t::Style {
                        color: Some(Color::from_rgba(0.5, 0.5, 0.5, 0.8))
                    }),
                    container(
                        iced::widget::row![
                            iced::widget::text("UTC-08:00 Pacific TIme (US & Canada)").size(13),
                            iced::widget::horizontal_space(),
                            iced::widget::svg(peak_core::icons::get_status_icon(
                                "clock", "#888888"
                            ))
                            .width(Length::Fixed(16.0))
                            .height(Length::Fixed(16.0))
                        ]
                        .align_y(iced::Alignment::Center)
                        .padding(10)
                    )
                    .width(Length::Fill)
                    .style(move |_| container::Style {
                        background: Some(if is_light {
                            Color::WHITE.into()
                        } else {
                            Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()
                        }),
                        border: iced::Border {
                            radius: 8.0.into(),
                            width: 1.0,
                            color: Color::from_rgba(0.5, 0.5, 0.5, 0.2),
                        },
                        ..Default::default()
                    }),
                    iced::widget::row![
                        iced::widget::button(text("Back").size(14))
                            .on_press(Message::Wizard(peak_apps::wizard::WizardMessage::PrevStep))
                            .style(move |_, status| peak_core::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Finish").size(14))
                            .on_press(Message::Wizard(
                                peak_apps::wizard::WizardMessage::CompleteSetup
                            ))
                            .padding([12, 40])
                            .style(move |_, status| peak_core::styles::style_pill_button(
                                status, is_light
                            )),
                    ]
                    .spacing(20)
                    .align_y(iced::Alignment::Center)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::Alignment::Center)
            }
            peak_apps::wizard::WizardStep::Complete => iced::widget::column![
                iced::widget::svg(peak_core::icons::get_avatar_handle(
                    "peak",
                    if is_light { "#000000" } else { "#FFFFFF" }
                ))
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0)),
                text("Account Created").size(24).font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                }),
                iced::widget::Space::with_height(20.0),
                iced::widget::button(text("Enter PeakOS").size(16))
                    .on_press(Message::SubmitLogin)
                    .padding([15, 60])
                    .style(move |_, status| peak_core::styles::style_pill_button(status, is_light))
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
        };

        // 1. Theme Toggle (Top Right)
        let theme_btn = iced::widget::button(
            iced::widget::svg(peak_core::icons::get_status_icon(
                if is_light { "moon" } else { "sun" },
                if is_light { "#000000" } else { "#FFFFFF" },
            ))
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0)),
        )
        .on_press(Message::ToggleTheme)
        .padding(10)
        .style(move |_, _| iced::widget::button::Style {
            background: None,
            text_color: if is_light { Color::BLACK } else { Color::WHITE },
            ..Default::default()
        });

        let top_right = container(theme_btn)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right)
            .align_y(iced::alignment::Vertical::Top)
            .padding(20);

        // 2. Logo (Top Left)
        let top_left = container(
            iced::widget::image(iced::widget::image::Handle::from_path(
                peak_core::utils::assets::get_asset_path(&format!(
                    "icons/menubar/{}",
                    if is_light {
                        "peak_logo.png"
                    } else {
                        "peak_logo_dark.png"
                    }
                )),
            ))
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(50.0)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::alignment::Horizontal::Left)
        .align_y(iced::alignment::Vertical::Top)
        .padding(20);

        let card = container(content)
            .padding(40)
            .style(move |_| peak_core::styles::style_glass_card(is_light));

        // Wallpaper + Blur Overlay
        let wallpaper_path =
            peak_core::utils::assets::get_asset_path("wallpapers/mountain_sunset_warm.jpg");

        Stack::new()
            .push(
                container(
                    iced::widget::image(iced::widget::image::Handle::from_path(wallpaper_path))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(iced::ContentFit::Cover),
                )
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .push(
                container(iced::widget::Space::new(Length::Fill, Length::Fill))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(move |_theme: &iced::Theme| container::Style {
                        background: Some(
                            if is_light {
                                Color::from_rgba(0.9, 0.9, 0.95, 0.3)
                            } else {
                                Color::from_rgba(0.0, 0.0, 0.0, 0.5)
                            }
                            .into(),
                        ),
                        ..Default::default()
                    }),
            )
            .push(
                container(card)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
            )
            .push(top_right)
            .push(top_left)
            .into()
    }
}
