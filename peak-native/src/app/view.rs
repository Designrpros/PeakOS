// Main view rendering

use super::{AppState, Message, PeakNative};
use iced::widget::{button, container, text, text as t, Stack};
use iced::{Color, Element, Length};

impl PeakNative {
    pub fn view(&self) -> Element<'_, Message> {
        let content = match &self.state {
            AppState::Setup(wizard_state) => self.view_setup(wizard_state),
            AppState::Login(_) => self.view_login_new(),
            AppState::Desktop => self.view_desktop(),
        };

        if let Some((title, body)) = &self.alert {
            iced::widget::stack![
                content,
                iced::widget::container(crate::components::alert::SystemAlert::view(
                    title,
                    body,
                    Message::CloseAlert,
                    matches!(self.theme, crate::theme::Theme::Light)
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

        let avatar_icon_key = self.user.as_ref().and_then(|u| u.avatar_icon.clone());

        let is_light = self.settings.theme_mode == crate::apps::settings::ThemeMode::Light;
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

        // 1. Avatar Circle
        let avatar_content: Element<'_, Message> = if let Some(key) = avatar_icon_key {
            let color = if is_light { "#000000" } else { "#FFFFFF" };
            let handle = crate::icons::get_avatar_handle(&key, color);

            container(
                iced::widget::svg(handle)
                    .width(Length::Fixed(70.0))
                    .height(Length::Fixed(70.0)),
            )
            .into()
        } else {
            container(
                text(user_name.chars().next().unwrap_or('?').to_string())
                    .size(60)
                    .style(move |_| t::Style {
                        color: Some(Color::BLACK),
                    }),
            )
            .into()
        };

        let avatar = container(avatar_content)
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(120.0))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(move |_| container::Style {
                // Subtle Glass effect for avatar
                background: Some(if is_light {
                    Color::from_rgba(0.0, 0.0, 0.0, 0.05).into()
                } else {
                    // "Monochrome Pop": White background for avatar in Dark Mode
                    Color::WHITE.into()
                }),
                border: iced::Border {
                    radius: 60.0.into(),
                    width: 1.0,
                    color: if is_light {
                        Color::from_rgba(0.0, 0.0, 0.0, 0.1)
                    } else {
                        Color::BLACK // Contrast border against white circle
                    },
                },
                ..Default::default()
            });

        let content = iced::widget::column![
            iced::widget::image(iced::widget::image::Handle::from_path(
                crate::utils::assets::get_asset_path(&format!(
                    "icons/menubar/{}",
                    if is_light {
                        "peak_logo.png"
                    } else {
                        "peak_logo_dark.png"
                    }
                ))
            ))
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(50.0)),
            iced::widget::Space::with_height(20.0),
            avatar,
            iced::widget::Space::with_height(20.0),
            // 2. Name
            text(user_name)
                .size(24)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .style(move |_| t::Style {
                    color: Some(text_color)
                }),
            iced::widget::Space::with_height(20.0),
            // 3. Password Input
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
            .style(move |_, status| crate::styles::style_soft_input(status, is_light)),
            iced::widget::Space::with_height(30.0),
            // 4. Login Button
            button(
                container(text("Login").size(16))
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center)
            )
            .on_press(Message::SubmitLogin)
            .padding([12, 50])
            .width(Length::Fixed(280.0)) // Match input width
            .style(move |_, status| crate::styles::style_pill_button(status, is_light))
        ]
        .align_x(iced::Alignment::Center)
        .spacing(0);

        // WRAP IN GLASS CARD
        let card = container(content)
            .padding(40)
            .style(move |_| crate::styles::style_glass_card(is_light));

        let centered_content = container(card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        // Vector Background Layer
        Stack::new()
            .push(self.vector_bg.view(is_light))
            .push(centered_content)
            .push(top_right)
            .into()
    }

    fn view_setup(&self, state: &crate::apps::wizard::WizardState) -> Element<'_, Message> {
        let is_light = self.settings.theme_mode == crate::apps::settings::ThemeMode::Light;

        // --- Steps ---

        let content = match state.current_step {
            crate::apps::wizard::WizardStep::Welcome => iced::widget::column![
                iced::widget::image(iced::widget::image::Handle::from_path(
                    crate::utils::assets::get_asset_path(&format!(
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
                    .on_press(Message::Wizard(
                        crate::apps::wizard::WizardMessage::NextStep
                    ))
                    .padding([15, 60])
                    .style(move |_, status| crate::styles::style_pill_button(status, is_light))
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
            crate::apps::wizard::WizardStep::Identity => {
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
                                crate::apps::wizard::WizardMessage::UpdateFullName(s)
                            ))
                            .padding(12)
                            .style(move |_, status| crate::styles::style_soft_input(
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
                                crate::apps::wizard::WizardMessage::UpdateUsername(s)
                            ))
                            .padding(12)
                            .style(move |_, status| crate::styles::style_soft_input(
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
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::PrevStep
                            ))
                            .style(move |_, status| crate::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Next").size(16))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::NextStep
                            ))
                            .padding([12, 40])
                            .style(move |_, status| crate::styles::style_pill_button(
                                status, is_light
                            ))
                    ]
                    .spacing(20)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::alignment::Horizontal::Center)
            }
            crate::apps::wizard::WizardStep::Security => {
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
                                crate::apps::wizard::WizardMessage::UpdatePassword(s)
                            ))
                            .padding(12)
                            .secure(true)
                            .style(move |_, status| crate::styles::style_soft_input(
                                status, is_light
                            )),
                        iced::widget::Space::with_height(10.0),
                        text("Verify").size(12).style(move |_| t::Style {
                            color: Some(Color::from_rgb8(120, 120, 120))
                        }),
                        iced::widget::text_input("Required", &state.password_confirm_input)
                            .on_input(|s| Message::Wizard(
                                crate::apps::wizard::WizardMessage::UpdatePasswordConfirm(s)
                            ))
                            .padding(12)
                            .secure(true)
                            .style(move |_, status| crate::styles::style_soft_input(
                                status, is_light
                            )),
                        iced::widget::Space::with_height(10.0),
                        text("Hint").size(12).style(move |_| t::Style {
                            color: Some(Color::from_rgb8(120, 120, 120))
                        }),
                        iced::widget::text_input("Optional", &state.password_hint_input)
                            .on_input(|s| Message::Wizard(
                                crate::apps::wizard::WizardMessage::UpdatePasswordHint(s)
                            ))
                            .padding(12)
                            .style(move |_, status| crate::styles::style_soft_input(
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
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::PrevStep
                            ))
                            .style(move |_, status| crate::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Next").size(16))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::NextStep
                            ))
                            .padding([12, 40])
                            .style(move |_, status| crate::styles::style_pill_button(
                                status, is_light
                            ))
                    ]
                    .spacing(20)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::alignment::Horizontal::Center)
            }
            crate::apps::wizard::WizardStep::WifiConnect => {
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
                        crate::integrations::network::get_available_networks()
                            .into_iter()
                            .enumerate()
                            .map(|(i, net)| {
                                let is_sel = Some(net.clone()) == state.selected_network;
                                let content = iced::widget::button(
                                    iced::widget::row![
                                        iced::widget::svg(crate::icons::get_status_icon(
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
                                        // iced::widget::svg(crate::icons::lock(...)),
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
                                    crate::apps::wizard::WizardMessage::SelectNetwork(net),
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
                    iced::widget::Space::with_height(30.0),
                    iced::widget::row![
                        iced::widget::button(text("Back").size(14))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::PrevStep
                            ))
                            .style(move |_, status| crate::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Next").size(14))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::NextStep
                            ))
                            .padding([12, 40])
                            .style(move |_, status| crate::styles::style_pill_button(
                                status, is_light
                            )),
                    ]
                    .spacing(20)
                    .align_y(iced::Alignment::Center)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::Alignment::Center)
            }
            crate::apps::wizard::WizardStep::ThemeSelection => {
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
                    text("Choose your look.").size(14).style(move |_| t::Style {
                        color: Some(Color::from_rgb8(102, 102, 102))
                    }),
                    iced::widget::Space::with_height(20.0),
                    // Avatar Section
                    text("Avatar").size(12).style(move |_| t::Style {
                        color: Some(Color::from_rgb8(120, 120, 120))
                    }),
                    iced::widget::row(
                        crate::icons::AVATAR_OPTIONS
                            .iter()
                            .map(|&key| {
                                let is_selected = state.selected_avatar.as_deref() == Some(key);
                                let color = match key {
                                    "robot" => Color::from_rgb8(0, 122, 255), // Blue
                                    "smile" => Color::from_rgb8(255, 204, 0), // Yellow
                                    "house" => Color::from_rgb8(52, 199, 89), // Green
                                    _ => Color::from_rgb8(142, 142, 147),     // Grey
                                };

                                iced::widget::button(
                                    container(iced::widget::Space::new(
                                        Length::Fixed(40.0),
                                        Length::Fixed(40.0),
                                    ))
                                    .style(move |_| {
                                        container::Style {
                                            background: Some(color.into()),
                                            border: iced::Border {
                                                radius: 20.0.into(),
                                                width: if is_selected { 2.0 } else { 0.0 },
                                                color: if is_light {
                                                    Color::BLACK
                                                } else {
                                                    Color::WHITE
                                                },
                                            },
                                            ..Default::default()
                                        }
                                    }),
                                )
                                .on_press(Message::Wizard(
                                    crate::apps::wizard::WizardMessage::SelectAvatar(
                                        key.to_string(),
                                    ),
                                ))
                                .padding(2)
                                .style(move |_, _status| {
                                    iced::widget::button::Style {
                                        background: None,
                                        border: iced::Border {
                                            radius: 24.0.into(), // Larger radius for hover ring if needed
                                            width: 0.0,
                                            color: Color::TRANSPARENT,
                                        },
                                        ..Default::default()
                                    }
                                })
                                .into()
                            })
                            .collect::<Vec<_>>()
                    )
                    .spacing(15),
                    iced::widget::Space::with_height(20.0),
                    // Theme Section
                    text("Theme").size(12).style(move |_| t::Style {
                        color: Some(Color::from_rgba(0.5, 0.5, 0.5, 0.8))
                    }),
                    iced::widget::row![
                        iced::widget::button(
                            iced::widget::column![
                                iced::widget::svg(crate::icons::get_status_icon(
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
                                iced::widget::svg(crate::icons::get_status_icon(
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
                    iced::widget::Space::with_height(40.0),
                    iced::widget::row![
                        iced::widget::button(text("Back").size(14))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::PrevStep
                            ))
                            .style(move |_, status| crate::styles::style_secondary_button(
                                status, is_light
                            )),
                        iced::widget::button(text("Finish").size(14))
                            .on_press(Message::Wizard(
                                crate::apps::wizard::WizardMessage::CompleteSetup
                            ))
                            .padding([12, 40])
                            .style(move |_, status| crate::styles::style_pill_button(
                                status, is_light
                            )),
                    ]
                    .spacing(20)
                    .align_y(iced::Alignment::Center)
                ]
                .width(Length::Fixed(360.0))
                .align_x(iced::Alignment::Center)
            }
            crate::apps::wizard::WizardStep::Complete => iced::widget::column![
                iced::widget::svg(crate::icons::get_avatar_handle(
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
                    .style(move |_, status| crate::styles::style_pill_button(status, is_light))
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
        };

        let card = container(content)
            .padding(40)
            .style(move |_| crate::styles::style_glass_card(is_light));

        Stack::new()
            .push(self.vector_bg.view(is_light))
            .push(
                container(card)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
            )
            .into()
    }
}
