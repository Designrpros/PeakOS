use iced::{Alignment, Length};
use peak_apps::wizard::{WizardMessage, WizardState, WizardStep};
use peak_ui::prelude::*;

pub struct WizardView {
    state: WizardState,
}

impl WizardView {
    pub fn new(state: WizardState) -> Self {
        Self { state }
    }
}

impl<Message> View<Message, peak_ui::core::IcedBackend> for WizardView
where
    Message: Clone + 'static + From<WizardMessage>,
{
    fn view(&self, context: &Context) -> Element<'static, Message> {
        let content = self.render_step::<Message>(context);

        // Determine opacity based on theme
        let overlay_color = if matches!(context.theme.tone, peak_ui_theme::ThemeTone::Dark) {
            iced::Color::from_rgba(0.0, 0.0, 0.0, 0.6) // Darker overlay for dark theme
        } else {
            iced::Color::from_rgba(1.0, 1.0, 1.0, 0.4) // Light overlay for light theme
        };

        // Render Background Stack using native Iced widgets for precise control
        iced::widget::stack![
            // 1. Wallpaper Layer
            iced::widget::image("assets/wallpapers/mountain_sunset_warm.jpg")
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(iced::ContentFit::Cover),
            // 2. Opacity Layer
            iced::widget::container(iced::widget::Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| iced::widget::container::Style {
                    background: Some(overlay_color.into()),
                    ..Default::default()
                }),
            // 3. Content Layer (Centered)
            iced::widget::container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center),
            // 4. Header Layer (Icon & Theme Switcher)
            iced::widget::container(
                iced::widget::row![
                    // Left: Peak Icon
                    iced::widget::image("assets/Peak.png")
                        .width(Length::Fixed(32.0))
                        .height(Length::Fixed(32.0)),
                    iced::widget::horizontal_space(),
                    // Right: Theme Switcher
                    iced::widget::button(
                        iced::widget::text(
                            if matches!(context.theme.tone, peak_ui_theme::ThemeTone::Dark) {
                                "☀️"
                            } else {
                                "🌙"
                            }
                        )
                        .size(20.0)
                    )
                    .on_press(Message::from(WizardMessage::SelectTheme(
                        if matches!(context.theme.tone, peak_ui_theme::ThemeTone::Dark) {
                            "light".into()
                        } else {
                            "dark".into()
                        }
                    )))
                    .padding(8.0)
                ]
                .width(Length::Fill)
                .align_y(iced::Alignment::Center)
            )
            .padding(24.0)
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_y(iced::alignment::Vertical::Top)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

impl WizardView {
    fn render_step<Message>(&self, context: &Context) -> Element<'static, Message>
    where
        Message: Clone + 'static + From<WizardMessage>,
    {
        use peak_ui::core::IcedBackend as B;
        match self.state.current_step {
            WizardStep::Welcome => self.render_welcome::<Message, B>(context).into(),
            WizardStep::Identity => self.render_identity::<Message, B>(context).into(),
            WizardStep::Security => self.render_security::<Message, B>(context).into(),
            WizardStep::WifiConnect => self.render_wifi::<Message, B>(context).into(),
            WizardStep::ThemeSelection => self.render_theme_selection::<Message, B>(context).into(),
            WizardStep::Complete => self.render_complete::<Message, B>(context).into(),
        }
    }

    fn render_welcome<Message, B>(&self, context: &Context) -> B::AnyView<Message>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        // ... Ported Welcome Step ...
        // Placeholder for now
        B::vstack(
            vec![
                B::text(
                    "Welcome to Peak.".into(),
                    36.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::text(
                    "Let's set up your new home.".into(),
                    18.0,
                    None,
                    false,
                    true,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(40.0)),
                B::button(
                    B::text(
                        "Get Started".into(),
                        16.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Shrink,
                        Alignment::Center,
                        context,
                    ),
                    Some(WizardMessage::NextStep.into()),
                    Variant::Solid,
                    Intent::Primary,
                    context,
                ),
            ],
            10.0,
            iced::Padding::new(20.0),
            Length::Fill,
            Length::Shrink,
            Alignment::Center,
            1.0,
        )
    }

    // Stub other render functions
    fn render_identity<Message, B>(&self, context: &Context) -> B::AnyView<Message>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let content = B::vstack(
            vec![
                B::text::<Message>(
                    "Who's using this computer?".into(),
                    24.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::text::<Message>(
                    "This name will be visible to others on the network.".into(),
                    14.0,
                    Some(context.theme.colors.text_secondary),
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(40.0)),
                B::text_input(
                    self.state.full_name_input.clone(),
                    "Full Name".into(),
                    |s| WizardMessage::UpdateFullName(s).into(),
                    None::<Message>,
                    None,
                    false,
                    Variant::Soft,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(10.0)),
                B::text_input(
                    self.state.username_input.clone(),
                    "Account Name".into(),
                    |s| WizardMessage::UpdateUsername(s).into(),
                    Some(WizardMessage::NextStep.into()),
                    None,
                    false,
                    Variant::Soft,
                    context,
                ),
            ],
            0.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Center,
            1.0,
        );

        self.render_layout::<Message, B>(content, context)
    }

    fn render_security<Message, B>(&self, context: &Context) -> B::AnyView<Message>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let content = B::vstack(
            vec![
                B::text::<Message>(
                    "Create a password".into(),
                    24.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::text::<Message>(
                    "Make it memorable, but secure.".into(),
                    14.0,
                    Some(context.theme.colors.text_secondary),
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(40.0)),
                B::text_input(
                    self.state.password_input.clone(),
                    "Password".into(),
                    |s| WizardMessage::UpdatePassword(s).into(),
                    None::<Message>,
                    None,
                    true,
                    Variant::Soft,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(10.0)),
                B::text_input(
                    self.state.password_confirm_input.clone(),
                    "Verify".into(),
                    |s| WizardMessage::UpdatePasswordConfirm(s).into(),
                    None::<Message>,
                    None,
                    true,
                    Variant::Soft,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(10.0)),
                B::text_input(
                    self.state.password_hint_input.clone(),
                    "Hint (Optional)".into(),
                    |s| WizardMessage::UpdatePasswordHint(s).into(),
                    Some(WizardMessage::NextStep.into()),
                    None,
                    false,
                    Variant::Soft,
                    context,
                ),
            ],
            0.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Center,
            1.0,
        );

        self.render_layout::<Message, B>(content, context)
    }
    fn render_wifi<Message, B>(&self, context: &Context) -> B::AnyView<Message>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let content = B::vstack(
            vec![
                B::text::<Message>(
                    "Connect to Network".into(),
                    24.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::text::<Message>(
                    "Choose a network to get connected.".into(),
                    14.0,
                    Some(context.theme.colors.text_secondary),
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(40.0)),
                // Quick placeholder for network list - ideally this would be a scrollable list
                B::text_input(
                    self.state.selected_network.clone().unwrap_or_default(),
                    "Network Name (SSID)".into(),
                    |s| WizardMessage::SelectNetwork(s).into(),
                    None::<Message>,
                    None,
                    false,
                    Variant::Soft,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(10.0)),
                B::text_input(
                    self.state.wifi_password_input.clone(),
                    "Network Password".into(),
                    |s| WizardMessage::UpdateWifiPassword(s).into(),
                    Some(WizardMessage::NextStep.into()),
                    None,
                    true,
                    Variant::Soft,
                    context,
                ),
            ],
            0.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Center,
            1.0,
        );

        self.render_layout::<Message, B>(content, context)
    }

    fn render_theme_selection<Message, B>(&self, context: &Context) -> B::AnyView<Message>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let content = B::vstack(
            vec![
                B::text::<Message>(
                    "Choose your Style".into(),
                    24.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::text::<Message>(
                    "Select an experience that fits you.".into(),
                    14.0,
                    Some(context.theme.colors.text_secondary),
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(40.0)),
                // Mode Selection (Desktop vs Mobile vs TV)
                B::text::<Message>(
                    "Experience Mode".into(),
                    12.0,
                    Some(context.theme.colors.text_tertiary),
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                B::hstack(
                    vec![
                        B::button(
                            B::text(
                                "Desktop".into(),
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
                            Some(WizardMessage::SelectMode("desktop".into()).into()),
                            if self.state.selected_mode.as_deref() == Some("desktop") {
                                Variant::Solid
                            } else {
                                Variant::Soft
                            },
                            Intent::Primary,
                            context,
                        ),
                        B::button(
                            B::text(
                                "Tablet".into(),
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
                            Some(WizardMessage::SelectMode("tablet".into()).into()),
                            if self.state.selected_mode.as_deref() == Some("tablet") {
                                Variant::Solid
                            } else {
                                Variant::Soft
                            },
                            Intent::Primary,
                            context,
                        ),
                    ],
                    10.0,
                    iced::Padding::default(),
                    Length::Fill,
                    Length::Shrink,
                    Alignment::Center,
                    1.0,
                ),
                B::space(Length::Fill, Length::Fixed(20.0)),
                // Theme Tone (Light vs Dark) - Actually handled by system but let's offer it?
                // Wait, state has `selected_theme` (cupertino, redmond, ai).
                B::text::<Message>(
                    "Theme".into(),
                    12.0,
                    Some(context.theme.colors.text_tertiary),
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Start,
                    context,
                ),
                B::hstack(
                    vec![
                        B::button(
                            B::text(
                                "Cupertino".into(),
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
                            Some(WizardMessage::SelectTheme("cupertino".into()).into()),
                            if self.state.selected_theme.as_deref() == Some("cupertino") {
                                Variant::Solid
                            } else {
                                Variant::Soft
                            },
                            Intent::Primary,
                            context,
                        ),
                        B::button(
                            B::text(
                                "Peak".into(),
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
                            Some(WizardMessage::SelectTheme("peak".into()).into()),
                            if self.state.selected_theme.as_deref() == Some("peak") {
                                Variant::Solid
                            } else {
                                Variant::Soft
                            },
                            Intent::Primary,
                            context,
                        ),
                    ],
                    10.0,
                    iced::Padding::default(),
                    Length::Fill,
                    Length::Shrink,
                    Alignment::Center,
                    1.0,
                ),
            ],
            10.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Center,
            1.0,
        );

        self.render_layout::<Message, B>(content, context)
    }
    fn render_layout<Message, B>(
        &self,
        content: B::AnyView<Message>,
        context: &Context,
    ) -> B::AnyView<Message>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let error_view = if let Some(err) = &self.state.error_message {
            B::text(
                err.clone(),
                14.0,
                Some(context.theme.colors.danger),
                false,
                false,
                None,
                None,
                Length::Shrink,
                Alignment::Center,
                context,
            )
        } else {
            B::space(Length::Fill, Length::Fixed(14.0)) // Placeholder for consistent height
        };

        let nav_row = B::hstack(
            vec![
                B::button(
                    B::text(
                        "Back".into(),
                        16.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Shrink,
                        Alignment::Center,
                        context,
                    ),
                    Some(WizardMessage::PrevStep.into()),
                    Variant::Ghost,
                    Intent::Neutral,
                    context,
                ),
                B::space(Length::Fill, Length::Shrink),
                B::button(
                    B::text(
                        "Continue".into(),
                        16.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Shrink,
                        Alignment::Center,
                        context,
                    ),
                    Some(WizardMessage::NextStep.into()),
                    Variant::Solid,
                    Intent::Primary,
                    context,
                ),
            ],
            0.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Center,
            1.0,
        );

        B::vstack(
            vec![
                content,
                B::space(Length::Fill, Length::Fixed(20.0)),
                error_view,
                B::space(Length::Fill, Length::Fixed(20.0)),
                nav_row,
            ],
            0.0,
            iced::Padding::new(40.0),
            Length::Fixed(500.0), // Fixed width card
            Length::Shrink,
            Alignment::Center,
            1.0,
        )
    }
    fn render_complete<Message, B>(&self, context: &Context) -> B::AnyView<Message>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let content = B::vstack(
            vec![
                B::text::<Message>(
                    "Welcome Home.".into(),
                    32.0,
                    None,
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::text::<Message>(
                    "Your PeakOS is ready.".into(),
                    16.0,
                    Some(context.theme.colors.text_secondary),
                    false,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(40.0)),
                B::button(
                    B::text(
                        "Start using PeakOS".into(),
                        16.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Shrink,
                        Alignment::Center,
                        context,
                    ),
                    Some(WizardMessage::CompleteSetup.into()),
                    Variant::Solid,
                    Intent::Primary,
                    context,
                ),
            ],
            24.0,
            iced::Padding::default(),
            Length::Fill,
            Length::Shrink,
            Alignment::Center,
            1.0,
        );

        // Custom layout for complete screen
        content
    }
}
