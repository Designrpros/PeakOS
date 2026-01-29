pub use peak_core::apps::settings::{SettingsApp, SettingsMessage, SettingsTab, ThemeMode};
use peak_ui::prelude::*;

// --- Pure PeakUI View Implementation ---

trait SettingsHelpers {
    fn view_tab_content<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage>;
    fn card<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
        content: B::AnyView<SettingsMessage>,
    ) -> B::AnyView<SettingsMessage>;
    fn labeled_row<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
        label: &str,
        widget: B::AnyView<SettingsMessage>,
    ) -> B::AnyView<SettingsMessage>;
    fn view_general<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage>;
    fn view_appearance<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage>;
    fn view_display<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage>;
    fn view_wifi<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage>;
    fn view_bluetooth<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage>;
    fn view_sound<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage>;
    fn view_intelligence<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage>;
    fn view_modes<B: peak_ui::core::Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage>;
}

impl<B> peak_ui::core::View<SettingsMessage, B> for DesktopSettingsApp
where
    B: peak_ui::core::Backend,
    peak_ui::prelude::NavigationSplitView<SettingsMessage, B>:
        peak_ui::core::View<SettingsMessage, B>,
{
    fn view(&self, context: &peak_ui::core::Context) -> B::AnyView<SettingsMessage> {
        let app = self.app.clone();

        // Sidebar Content
        let sidebar = peak_ui::prelude::Sidebar::<SettingsMessage, B>::new("Settings")
            .with_search(app.search_query.clone(), SettingsMessage::SearchChanged)
            .item(
                "Network",
                "wifi_full",
                SettingsMessage::TabChanged(SettingsTab::WiFi),
                app.current_tab == SettingsTab::WiFi,
            )
            .item(
                "Bluetooth",
                "bluetooth",
                SettingsMessage::TabChanged(SettingsTab::Bluetooth),
                app.current_tab == SettingsTab::Bluetooth,
            )
            .item(
                "General",
                "settings",
                SettingsMessage::TabChanged(SettingsTab::General),
                app.current_tab == SettingsTab::General,
            )
            .item(
                "Appearance",
                "palette",
                SettingsMessage::TabChanged(SettingsTab::Appearance),
                app.current_tab == SettingsTab::Appearance,
            )
            .item(
                "Display",
                "image",
                SettingsMessage::TabChanged(SettingsTab::Display),
                app.current_tab == SettingsTab::Display,
            )
            .item(
                "Sound",
                "media",
                SettingsMessage::TabChanged(SettingsTab::Sound),
                app.current_tab == SettingsTab::Sound,
            )
            .item(
                "Focus",
                "moon",
                SettingsMessage::TabChanged(SettingsTab::Focus),
                app.current_tab == SettingsTab::Focus,
            )
            .item(
                "Intelligence",
                "sparkles",
                SettingsMessage::TabChanged(SettingsTab::Intelligence),
                app.current_tab == SettingsTab::Intelligence,
            )
            .item(
                "Modes",
                "trigger",
                SettingsMessage::TabChanged(SettingsTab::Modes),
                app.current_tab == SettingsTab::Modes,
            );

        // Main Content View Closure
        let app_for_closure = app.clone();
        let content_view = peak_ui::core::ProxyView::new(move |ctx| {
            let app = app_for_closure.clone();
            let tab_title = B::text(
                format!("{:?}", app.current_tab),
                28.0, // Larger premium title
                None,
                true, // Bold
                false,
                None,
                None,
                Length::Shrink,
                Alignment::Start,
                ctx,
            );

            let content = app.view_tab_content::<B>(ctx);

            let vstack = B::vstack(
                vec![tab_title, content],
                32.0, // Generous spacing
                iced::Padding::from(0.0),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            );

            // Add generous padding to the scrollable content
            B::container(
                B::scroll_view(vstack, Length::Fill, Length::Fill, None, ctx),
                iced::Padding::from([24, 32]), // Standard premium padding
                Length::Fill,
                Length::Fill,
                ctx,
            )
        });

        let is_sidebar = app.current_tab == SettingsTab::Sidebar;

        let split_view = peak_ui::prelude::NavigationSplitView::new_generic(sidebar, content_view)
            .force_sidebar_on_slim(is_sidebar)
            .sidebar_width(260.0) // Slightly wider sidebar for premium feel
            .on_back(SettingsMessage::TabChanged(SettingsTab::Sidebar));

        peak_ui::core::View::<SettingsMessage, B>::view(&split_view, context)
    }

    fn describe(&self, _context: &peak_ui::core::Context) -> peak_ui::core::SemanticNode {
        // AI Integration: Describe the current state
        let mut children = vec![];
        let app = &self.app;

        // Describe SideBar items (Navigation)
        let sidebar_node = peak_ui::core::SemanticNode {
            role: "navigation".into(),
            label: Some("Sidebar".into()),
            children: vec![
                peak_ui::core::SemanticNode {
                    role: "button".into(),
                    label: Some("Network".into()),
                    ..Default::default()
                },
                peak_ui::core::SemanticNode {
                    role: "button".into(),
                    label: Some("Bluetooth".into()),
                    ..Default::default()
                },
                peak_ui::core::SemanticNode {
                    role: "button".into(),
                    label: Some("General".into()),
                    ..Default::default()
                },
                peak_ui::core::SemanticNode {
                    role: "button".into(),
                    label: Some("Appearance".into()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        children.push(sidebar_node);

        // Describe Current Tab Content
        children.push(peak_ui::core::SemanticNode {
            role: "main".into(),
            label: Some(format!("{:?} Settings", app.current_tab)),
            content: Some(match app.current_tab {
                SettingsTab::WiFi => if app.wifi_enabled {
                    "WiFi On"
                } else {
                    "WiFi Off"
                }
                .into(),
                SettingsTab::Bluetooth => if app.bluetooth_enabled {
                    "Bluetooth On"
                } else {
                    "Bluetooth Off"
                }
                .into(),
                SettingsTab::Sound => format!("Volume: {:.0}%", app.volume * 100.0),
                _ => "".into(),
            }),
            ..Default::default()
        });

        peak_ui::core::SemanticNode {
            role: "window".into(),
            label: Some("Settings".into()),
            children,
            ..Default::default()
        }
    }
}

impl SettingsHelpers for SettingsApp {
    fn view_tab_content<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage> {
        match self.current_tab {
            SettingsTab::Sidebar => B::space(Length::Shrink, Length::Shrink),
            SettingsTab::General => self.view_general::<B>(context),
            SettingsTab::Appearance => self.view_appearance::<B>(context),
            SettingsTab::Display => self.view_display::<B>(context),
            SettingsTab::WiFi => self.view_wifi::<B>(context),
            SettingsTab::Bluetooth => self.view_bluetooth::<B>(context),
            SettingsTab::Sound => self.view_sound::<B>(context),
            SettingsTab::Intelligence => self.view_intelligence::<B>(context),
            SettingsTab::Modes => self.view_modes::<B>(context),
            _ => B::text(
                "".into(),
                16.0,
                None,
                false,
                false,
                None,
                None,
                Length::Shrink,
                Alignment::Start,
                context,
            ),
        }
    }

    fn card<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
        content: B::AnyView<SettingsMessage>,
    ) -> B::AnyView<SettingsMessage> {
        let _radius = context.radius(12.0);
        let _border_color = context.theme.colors.divider;
        let _bg_color = context.theme.colors.surface;

        B::container(
            content,
            iced::Padding::from(16.0),
            Length::Fill,
            Length::Shrink,
            context,
        )
        // Note: Generic Backend might not support complex borders yet,
        // but we'll use container as the base.
        // In a real PeakUI impl, B::container would have a style/variant.
    }

    fn labeled_row<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
        label: &str,
        widget: B::AnyView<SettingsMessage>,
    ) -> B::AnyView<SettingsMessage> {
        B::hstack(
            vec![
                B::text(
                    label.into(),
                    14.0,
                    None,
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::space(Length::Fill, Length::Shrink),
                widget,
            ],
            0.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Center,
            Alignment::Center,
            1.0,
        )
    }
    fn view_general<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage> {
        let about_card = self.card::<B>(
            context,
            B::vstack(
                vec![
                    self.labeled_row::<B>(
                        context,
                        "Version",
                        B::text(
                            "1.0.0-alpha.5".into(),
                            14.0,
                            None,
                            false,
                            false,
                            None,
                            None,
                            Length::Shrink,
                            Alignment::End,
                            context,
                        ),
                    ),
                    B::space(Length::Shrink, Length::Fixed(8.0)),
                    self.labeled_row::<B>(
                        context,
                        "Kernel",
                        B::text(
                            "PeakOS 6.1.0-generic".into(),
                            14.0,
                            None,
                            false,
                            false,
                            None,
                            None,
                            Length::Shrink,
                            Alignment::End,
                            context,
                        ),
                    ),
                ],
                8.0,
                iced::Padding::default(),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            ),
        );

        B::vstack(
            vec![
                B::text(
                    "General".into(),
                    14.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                about_card,
            ],
            12.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Start,
            Alignment::Start,
            1.0,
        )
    }

    fn view_appearance<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage> {
        let theme_card = self.card::<B>(
            context,
            B::vstack(
                vec![
                    self.labeled_row::<B>(
                        context,
                        "Dark Mode",
                        B::toggle(
                            "".into(),
                            self.theme_mode == ThemeMode::Dark,
                            |b| {
                                if b {
                                    SettingsMessage::ThemeChanged(ThemeMode::Dark)
                                } else {
                                    SettingsMessage::ThemeChanged(ThemeMode::Light)
                                }
                            },
                            context,
                        ),
                    ),
                    B::space(Length::Shrink, Length::Fixed(8.0)),
                    B::text(
                        "Automatically switch between light and dark themes based on system settings or schedule.".into(),
                        12.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Fill,
                        Alignment::Start,
                        context,
                    ),
                ],
                0.0,
                iced::Padding::default(),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            ),
        );

        B::vstack(
            vec![
                B::text(
                    "Theme".into(),
                    14.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                theme_card,
            ],
            12.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Start,
            Alignment::Start,
            1.0,
        )
    }

    fn view_display<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage> {
        B::text(
            "Display Settings".into(),
            16.0,
            None,
            false,
            false,
            None,
            None,
            Length::Shrink,
            Alignment::Start,
            context,
        )
    }

    fn view_wifi<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage> {
        let wifi_card = self.card::<B>(
            context,
            B::vstack(
                vec![
                    self.labeled_row::<B>(
                        context,
                        "Wi-Fi",
                        B::toggle(
                            "".into(),
                            self.wifi_enabled,
                            |b| SettingsMessage::ToggleWiFi(b),
                            context,
                        ),
                    ),
                    B::space(Length::Shrink, Length::Fixed(8.0)),
                    B::text(
                        if self.wifi_enabled {
                            "Connected to 'Peak_5G'"
                        } else {
                            "Turn on Wi-Fi to see available networks"
                        }
                        .into(),
                        12.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Fill,
                        Alignment::Start,
                        context,
                    ),
                ],
                0.0,
                iced::Padding::default(),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            ),
        );

        B::vstack(
            vec![
                B::text(
                    "Network".into(),
                    14.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                wifi_card,
            ],
            12.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Start,
            Alignment::Start,
            1.0,
        )
    }

    fn view_bluetooth<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage> {
        let bt_card = self.card::<B>(
            context,
            B::vstack(
                vec![
                    self.labeled_row::<B>(
                        context,
                        "Bluetooth",
                        B::toggle(
                            "".into(),
                            self.bluetooth_enabled,
                            |b| SettingsMessage::ToggleBluetooth(b),
                            context,
                        ),
                    ),
                    B::space(Length::Shrink, Length::Fixed(8.0)),
                    B::text(
                        if self.bluetooth_enabled {
                            "Visible as 'Peak_Workstation'"
                        } else {
                            "Bluetooth is disabled"
                        }
                        .into(),
                        12.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Fill,
                        Alignment::Start,
                        context,
                    ),
                ],
                0.0,
                iced::Padding::default(),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            ),
        );

        B::vstack(
            vec![
                B::text(
                    "Devices".into(),
                    14.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                bt_card,
            ],
            12.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Start,
            Alignment::Start,
            1.0,
        )
    }

    fn view_sound<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage> {
        let volume_card = self.card::<B>(
            context,
            B::vstack(
                vec![
                    self.labeled_row::<B>(
                        context,
                        "Output Volume",
                        B::slider(
                            0.0..=1.0,
                            self.volume,
                            |v| SettingsMessage::VolumeChanged(v),
                            context,
                        ),
                    ),
                    B::space(Length::Shrink, Length::Fixed(8.0)),
                    self.labeled_row::<B>(
                        context,
                        "Mute",
                        B::toggle(
                            "".into(),
                            self.volume == 0.0,
                            |b| {
                                if b {
                                    SettingsMessage::VolumeChanged(0.0)
                                } else {
                                    SettingsMessage::VolumeChanged(0.5)
                                }
                            },
                            context,
                        ),
                    ),
                ],
                8.0,
                iced::Padding::default(),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            ),
        );

        B::vstack(
            vec![
                B::text(
                    "Audio".into(),
                    14.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                volume_card,
            ],
            12.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Start,
            Alignment::Start,
            1.0,
        )
    }

    fn view_intelligence<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage> {
        let app = self;

        // --- Local Intelligence Card ---
        let local_card = self.card::<B>(
            context,
            B::vstack(
                app.recommended_models
                    .iter()
                    .map(|m| {
                        let status_text = if m.is_active {
                            "Active"
                        } else if let Some(prog) = m.download_progress {
                            &format!("Downloading {:.0}%", prog * 100.0)
                        } else if m.is_downloaded {
                            "Downloaded"
                        } else {
                            &m.size_estimate
                        };

                        B::hstack(
                            vec![
                                B::vstack(
                                    vec![
                                        B::text(
                                            m.name.clone(),
                                            14.0,
                                            None,
                                            true,
                                            false,
                                            None,
                                            None,
                                            Length::Shrink,
                                            Alignment::Start,
                                            context,
                                        ),
                                        B::text(
                                            m.description.clone(),
                                            12.0,
                                            None,
                                            false,
                                            false,
                                            None,
                                            None,
                                            Length::Shrink,
                                            Alignment::Start,
                                            context,
                                        ),
                                    ],
                                    2.0,
                                    iced::Padding::default(),
                                    Length::Shrink,
                                    Length::Shrink,
                                    Alignment::Center,
                                    Alignment::Center,
                                    1.0,
                                ),
                                B::space(Length::Fill, Length::Shrink),
                                B::button(
                                    B::text(
                                        status_text.into(),
                                        13.0,
                                        None,
                                        false,
                                        false,
                                        None,
                                        None,
                                        Length::Shrink,
                                        Alignment::Center,
                                        context,
                                    ),
                                    if m.is_active {
                                        None
                                    } else if m.is_downloaded {
                                        Some(SettingsMessage::ModelActivate(m.id.clone()))
                                    } else {
                                        Some(SettingsMessage::ModelDownload(m.id.clone()))
                                    },
                                    if m.is_active {
                                        peak_ui::modifiers::Variant::Soft
                                    } else {
                                        peak_ui::modifiers::Variant::Ghost
                                    },
                                    if m.is_active {
                                        peak_ui::modifiers::Intent::Success
                                    } else {
                                        peak_ui::modifiers::Intent::Neutral
                                    },
                                    false, // is_compact
                                    context,
                                ),
                            ],
                            0.0,
                            iced::Padding::default(),
                            Length::Fill,
                            Length::Shrink,
                            Alignment::Center,
                            Alignment::Center,
                            1.0,
                        )
                    })
                    .collect(),
                12.0,
                iced::Padding::default(),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            ),
        );

        // --- Cloud Intelligence Card ---
        let cloud_card = self.card::<B>(
            context,
            B::vstack(
                vec![
                    self.labeled_row::<B>(
                        context,
                        "OpenRouter Access",
                        B::toggle(
                            "".into(),
                            app.cloud_intelligence_enabled,
                            |b| SettingsMessage::ToggleCloudIntelligence(b),
                            context,
                        ),
                    ),
                    B::space(Length::Shrink, Length::Fixed(8.0)),
                    B::text("Connect to high-performance cloud models (Claude 3.5, GPT-4o) via OpenRouter.".into(), 12.0, None, false, false, None, None, Length::Fill, Alignment::Start, context),
                    // B::input_field placeholder if Backend supports it?
                    // For now let's assume B::text as a placeholder for the API key if no input tool exists
                    B::text(if app.openrouter_key.is_empty() { "API Key: Not Set".into() } else { "API Key: ••••••••••••".into() }, 13.0, None, false, false, None, None, Length::Shrink, Alignment::Start, context),
                ],
                4.0,
                iced::Padding::default(),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            ),
        );

        // --- Preferences Card ---
        let prefs_card = self.card::<B>(
            context,
            B::vstack(
                vec![
                    self.labeled_row::<B>(
                        context,
                        "Subtitles & Captions",
                        B::toggle(
                            "".into(),
                            app.captions_enabled,
                            |b| SettingsMessage::ToggleCaptions(b),
                            context,
                        ),
                    ),
                    B::space(Length::Shrink, Length::Fixed(4.0)),
                    self.labeled_row::<B>(
                        context,
                        "Voice Feedback",
                        B::toggle(
                            "".into(),
                            app.voice_enabled,
                            |b| SettingsMessage::ToggleVoice(b),
                            context,
                        ),
                    ),
                ],
                8.0,
                iced::Padding::default(),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            ),
        );

        B::vstack(
            vec![
                B::text(
                    "Local Intelligence".into(),
                    14.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                local_card,
                B::space(Length::Shrink, Length::Fixed(16.0)),
                B::text(
                    "Cloud Intelligence".into(),
                    14.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                cloud_card,
                B::space(Length::Shrink, Length::Fixed(16.0)),
                B::text(
                    "Preferences".into(),
                    14.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                prefs_card,
            ],
            12.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Start,
            Alignment::Start,
            1.0,
        )
    }

    fn view_modes<B: Backend>(
        &self,
        context: &peak_ui::core::Context,
    ) -> B::AnyView<SettingsMessage> {
        let modes = [
            peak_core::registry::ShellMode::Desktop,
            peak_core::registry::ShellMode::Mobile,
            peak_core::registry::ShellMode::TV,
            peak_core::registry::ShellMode::Console,
            peak_core::registry::ShellMode::Auto,
        ];

        let styles = [
            peak_core::registry::ShellStyle::Cupertino,
            peak_core::registry::ShellStyle::Redmond,
            peak_core::registry::ShellStyle::AI,
            peak_core::registry::ShellStyle::TV,
        ];

        let mode_card = self.card::<B>(
            context,
            B::vstack(
                modes
                    .into_iter()
                    .map(|m| {
                        B::button(
                            B::hstack(
                                vec![
                                    B::text(
                                        format!("{}", m),
                                        14.0,
                                        None,
                                        false,
                                        false,
                                        None,
                                        None,
                                        Length::Shrink,
                                        Alignment::Start,
                                        context,
                                    ),
                                    B::space(Length::Fill, Length::Shrink),
                                    if self.current_mode == m {
                                        B::text(
                                            "Active".into(),
                                            12.0,
                                            Some(context.theme.colors.primary.into()),
                                            true,
                                            false,
                                            None,
                                            None,
                                            Length::Shrink,
                                            Alignment::End,
                                            context,
                                        )
                                    } else {
                                        B::space(Length::Shrink, Length::Shrink)
                                    },
                                ],
                                0.0,
                                iced::Padding::default(),
                                Length::Fill,
                                Length::Shrink,
                                Alignment::Center,
                                Alignment::Center,
                                1.0,
                            ),
                            Some(SettingsMessage::ModeChanged(m)),
                            if self.current_mode == m {
                                Variant::Soft
                            } else {
                                Variant::Ghost
                            },
                            Intent::Neutral,
                            false, // is_compact
                            context,
                        )
                    })
                    .collect(),
                4.0,
                iced::Padding::default(),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            ),
        );

        let style_card = self.card::<B>(
            context,
            B::vstack(
                styles
                    .into_iter()
                    .map(|s| {
                        B::button(
                            B::hstack(
                                vec![
                                    B::text(
                                        format!("{}", s),
                                        14.0,
                                        None,
                                        false,
                                        false,
                                        None,
                                        None,
                                        Length::Shrink,
                                        Alignment::Start,
                                        context,
                                    ),
                                    B::space(Length::Fill, Length::Shrink),
                                    if self.current_shell_style == s {
                                        B::text(
                                            "Active".into(),
                                            12.0,
                                            Some(context.theme.colors.primary.into()),
                                            true,
                                            false,
                                            None,
                                            None,
                                            Length::Shrink,
                                            Alignment::End,
                                            context,
                                        )
                                    } else {
                                        B::space(Length::Shrink, Length::Shrink)
                                    },
                                ],
                                0.0,
                                iced::Padding::default(),
                                Length::Fill,
                                Length::Shrink,
                                Alignment::Center,
                                Alignment::Center,
                                1.0,
                            ),
                            Some(SettingsMessage::ShellStyleChanged(s)),
                            if self.current_shell_style == s {
                                Variant::Soft
                            } else {
                                Variant::Ghost
                            },
                            Intent::Neutral,
                            false, // is_compact
                            context,
                        )
                    })
                    .collect(),
                4.0,
                iced::Padding::default(),
                Length::Fill,
                Length::Shrink,
                Alignment::Start,
                Alignment::Start,
                1.0,
            ),
        );

        B::vstack(
            vec![
                B::text(
                    "Experience".into(),
                    14.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                mode_card,
                B::space(Length::Shrink, Length::Fixed(16.0)),
                B::text(
                    "Interface Style".into(),
                    14.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                style_card,
            ],
            12.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Start,
            Alignment::Start,
            1.0,
        )
    }
}

pub struct DesktopSettingsApp {
    pub app: SettingsApp,
}

impl DesktopSettingsApp {
    pub fn new() -> Self {
        Self {
            app: SettingsApp::new(),
        }
    }
}

impl Default for DesktopSettingsApp {
    fn default() -> Self {
        Self::new()
    }
}

use peak_core::app_traits::{PeakApp, ShellContext};

impl PeakApp for DesktopSettingsApp {
    type Message = SettingsMessage;

    fn title(&self) -> String {
        self.app.title()
    }

    fn update(
        &mut self,
        message: Self::Message,
        context: &dyn ShellContext,
    ) -> iced::Task<Self::Message> {
        self.app.update(message, context)
    }

    fn view(&self, theme: &peak_core::theme::Theme) -> iced::Element<'static, Self::Message> {
        let mode = peak_ui::core::ShellMode::Desktop;
        let tone = if *theme == peak_core::theme::Theme::Light {
            peak_ui_theme::ThemeTone::Light
        } else {
            peak_ui_theme::ThemeTone::Dark
        };
        let tokens = peak_ui::core::ThemeTokens::get(mode, tone);
        let context = peak_ui::core::Context::new(
            mode,
            tokens,
            iced::Size::new(1920.0, 1080.0),
            Localization::default(),
        );

        peak_ui::core::View::<SettingsMessage, peak_ui::core::IcedBackend>::view(self, &context)
            .into()
    }
}
