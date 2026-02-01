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

impl<Message, B> View<Message, B> for WizardView
where
    Message: Clone + 'static + From<WizardMessage>,
    B: peak_ui::core::Backend,
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let content = self.render_step::<Message, B>(context);

        // Determine opacity based on theme
        let overlay_color = if matches!(context.theme.tone, peak_ui_theme::ThemeTone::Dark) {
            iced::Color::from_rgba(0.0, 0.0, 0.0, 0.6) // Darker overlay for dark theme
        } else {
            iced::Color::from_rgba(1.0, 1.0, 1.0, 0.4) // Light overlay for light theme
        };

        let wallpaper = Image::new("assets/wallpapers/mountain_sunset_warm.jpg")
            .width(Length::Fill)
            .height(Length::Fill);

        let overlay = Container::new(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .background(overlay_color);

        let header = Container::new(
            hstack![
                // Left: Peak Icon
                Image::new("assets/Peak.png")
                    .width(Length::Fixed(32.0))
                    .height(Length::Fixed(32.0)),
                Space::new(Length::Fill, Length::Shrink),
                // Right: Theme Switcher
                Button::new(
                    Text::<B>::new(
                        if matches!(context.theme.tone, peak_ui_theme::ThemeTone::Dark) {
                            "☀️"
                        } else {
                            "🌙"
                        },
                    )
                    .size(20.0),
                )
                .on_press(Message::from(WizardMessage::SelectTheme(
                    if matches!(context.theme.tone, peak_ui_theme::ThemeTone::Dark) {
                        "light".into()
                    } else {
                        "dark".into()
                    },
                )))
                .padding(8.0)
                .width(Length::Shrink)
            ]
            .width(Length::Fill)
            .align_y(Alignment::Center),
        )
        .padding(24.0)
        .width(Length::Fill)
        .height(Length::Shrink)
        .align_y(Alignment::Start);

        let centered_content = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        zstack![wallpaper, overlay, centered_content, header]
            .width(Length::Fill)
            .height(Length::Fill)
            .view(context)
    }
}

impl WizardView {
    fn render_step<Message, B>(&self, context: &Context) -> Box<dyn View<Message, B>>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        match self.state.current_step {
            WizardStep::Welcome => self.render_welcome::<Message, B>(context),
            WizardStep::Identity => self.render_identity::<Message, B>(context),
            WizardStep::Security => self.render_security::<Message, B>(context),
            WizardStep::WifiConnect => self.render_wifi::<Message, B>(context),
            WizardStep::ThemeSelection => self.render_theme_selection::<Message, B>(context),
            WizardStep::Complete => self.render_complete::<Message, B>(),
        }
    }

    fn render_welcome<Message, B>(&self, _context: &Context) -> Box<dyn View<Message, B>>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        Box::new(
            vstack![
                Text::<B>::new("Welcome to Peak.")
                    .size(36.0)
                    .bold()
                    .align(Alignment::Center),
                Text::<B>::new("Let's set up your new home.")
                    .size(18.0)
                    .secondary()
                    .align(Alignment::Center),
                Space::new(Length::Fill, Length::Fixed(40.0)),
                Button::new(
                    Text::<B>::new("Get Started")
                        .size(16.0)
                        .align(Alignment::Center)
                )
                .on_press(Message::from(WizardMessage::NextStep))
                .variant(Variant::Solid)
                .intent(Intent::Primary)
                .width(Length::Shrink),
            ]
            .spacing(10.0)
            .padding(20.0)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
        )
    }

    fn render_identity<Message, B>(&self, context: &Context) -> Box<dyn View<Message, B>>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let content = vstack![
            Text::<B>::new("Who's using this computer?")
                .size(24.0)
                .bold()
                .align(Alignment::Center),
            Text::<B>::new("This name will be visible to others on the network.")
                .size(14.0)
                .secondary()
                .align(Alignment::Center),
            Space::new(Length::Fill, Length::Fixed(40.0)),
            TextInput::new(self.state.full_name_input.clone(), "Full Name", |s| {
                WizardMessage::UpdateFullName(s).into()
            },)
            .variant(Variant::Soft),
            Space::new(Length::Fill, Length::Fixed(10.0)),
            TextInput::new(self.state.username_input.clone(), "Account Name", |s| {
                WizardMessage::UpdateUsername(s).into()
            },)
            .on_submit(Message::from(WizardMessage::NextStep))
            .variant(Variant::Soft),
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        self.render_layout::<Message, B>(content, context)
    }

    fn render_security<Message, B>(&self, context: &Context) -> Box<dyn View<Message, B>>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let content = vstack![
            Text::<B>::new("Create a password")
                .size(24.0)
                .bold()
                .align(Alignment::Center),
            Text::<B>::new("Make it memorable, but secure.")
                .size(14.0)
                .secondary()
                .align(Alignment::Center),
            Space::new(Length::Fill, Length::Fixed(40.0)),
            TextInput::new(self.state.password_input.clone(), "Password", |s| {
                WizardMessage::UpdatePassword(s).into()
            },)
            .password()
            .variant(Variant::Soft),
            Space::new(Length::Fill, Length::Fixed(10.0)),
            TextInput::new(self.state.password_confirm_input.clone(), "Verify", |s| {
                WizardMessage::UpdatePasswordConfirm(s).into()
            },)
            .password()
            .variant(Variant::Soft),
            Space::new(Length::Fill, Length::Fixed(10.0)),
            TextInput::new(
                self.state.password_hint_input.clone(),
                "Hint (Optional)",
                |s| WizardMessage::UpdatePasswordHint(s).into(),
            )
            .on_submit(Message::from(WizardMessage::NextStep))
            .variant(Variant::Soft),
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        self.render_layout::<Message, B>(content, context)
    }

    fn render_wifi<Message, B>(&self, context: &Context) -> Box<dyn View<Message, B>>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let content = vstack![
            Text::<B>::new("Connect to Network")
                .size(24.0)
                .bold()
                .align(Alignment::Center),
            Text::<B>::new("Choose a network to get connected.")
                .size(14.0)
                .secondary()
                .align(Alignment::Center),
            Space::new(Length::Fill, Length::Fixed(40.0)),
            TextInput::new(
                self.state.selected_network.clone().unwrap_or_default(),
                "Network Name (SSID)",
                |s| WizardMessage::SelectNetwork(s).into(),
            )
            .variant(Variant::Soft),
            Space::new(Length::Fill, Length::Fixed(10.0)),
            TextInput::new(
                self.state.wifi_password_input.clone(),
                "Network Password",
                |s| WizardMessage::UpdateWifiPassword(s).into(),
            )
            .password()
            .on_submit(Message::from(WizardMessage::NextStep))
            .variant(Variant::Soft),
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        self.render_layout::<Message, B>(content, context)
    }

    fn render_theme_selection<Message, B>(&self, context: &Context) -> Box<dyn View<Message, B>>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let content = vstack![
            Text::<B>::new("Choose your Style")
                .size(24.0)
                .bold()
                .align(Alignment::Center),
            Text::<B>::new("Select an experience that fits you.")
                .size(14.0)
                .secondary()
                .align(Alignment::Center),
            Space::new(Length::Fill, Length::Fixed(40.0)),
            Text::<B>::new("Experience Mode")
                .size(12.0)
                .secondary()
                .bold(),
            hstack![
                Button::new(
                    Text::<B>::new("Desktop")
                        .size(14.0)
                        .align(Alignment::Center)
                )
                .on_press(Message::from(WizardMessage::SelectMode("desktop".into())))
                .variant(if self.state.selected_mode.as_deref() == Some("desktop") {
                    Variant::Solid
                } else {
                    Variant::Soft
                })
                .intent(Intent::Primary)
                .width(Length::Fill),
                Button::new(Text::<B>::new("Tablet").size(14.0).align(Alignment::Center))
                    .on_press(Message::from(WizardMessage::SelectMode("tablet".into())))
                    .variant(if self.state.selected_mode.as_deref() == Some("tablet") {
                        Variant::Solid
                    } else {
                        Variant::Soft
                    })
                    .intent(Intent::Primary)
                    .width(Length::Fill),
            ]
            .spacing(10.0)
            .width(Length::Fill),
            Space::new(Length::Fill, Length::Fixed(20.0)),
            Text::<B>::new("Theme").size(12.0).secondary().bold(),
            hstack![
                Button::new(
                    Text::<B>::new("Cupertino")
                        .size(14.0)
                        .align(Alignment::Center)
                )
                .on_press(Message::from(WizardMessage::SelectTheme(
                    "cupertino".into()
                )))
                .variant(
                    if self.state.selected_theme.as_deref() == Some("cupertino") {
                        Variant::Solid
                    } else {
                        Variant::Soft
                    }
                )
                .intent(Intent::Primary)
                .width(Length::Fill),
                Button::new(Text::<B>::new("Peak").size(14.0).align(Alignment::Center))
                    .on_press(Message::from(WizardMessage::SelectTheme("peak".into())))
                    .variant(if self.state.selected_theme.as_deref() == Some("peak") {
                        Variant::Solid
                    } else {
                        Variant::Soft
                    })
                    .intent(Intent::Primary)
                    .width(Length::Fill),
            ]
            .spacing(10.0)
            .width(Length::Fill),
        ]
        .spacing(10.0)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        self.render_layout::<Message, B>(content, context)
    }

    fn render_layout<Message, B>(
        &self,
        content: impl View<Message, B> + 'static,
        context: &Context,
    ) -> Box<dyn View<Message, B>>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        let error_view = if let Some(err) = &self.state.error_message {
            Text::<B>::new(err.clone())
                .size(14.0)
                .color(context.theme.colors.danger)
                .align(Alignment::Center)
        } else {
            Text::<B>::new(" ").size(14.0) // Placeholder
        };

        let nav_row = hstack![
            Button::new(Text::<B>::new("Back").size(16.0).align(Alignment::Center))
                .on_press(Message::from(WizardMessage::PrevStep))
                .variant(Variant::Ghost)
                .intent(Intent::Neutral)
                .width(Length::Shrink),
            Space::new(Length::Fill, Length::Shrink),
            Button::new(
                Text::<B>::new("Continue")
                    .size(16.0)
                    .align(Alignment::Center)
            )
            .on_press(Message::from(WizardMessage::NextStep))
            .variant(Variant::Solid)
            .intent(Intent::Primary)
            .width(Length::Shrink),
        ]
        .width(Length::Fill)
        .align_y(Alignment::Center);

        Box::new(
            vstack![
                content,
                Space::new(Length::Fill, Length::Fixed(20.0)),
                error_view,
                Space::new(Length::Fill, Length::Fixed(20.0)),
                nav_row,
            ]
            .padding(40.0)
            .width(Length::Fixed(500.0)) // Fixed width card
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
        )
    }

    fn render_complete<Message, B>(&self) -> Box<dyn View<Message, B>>
    where
        Message: Clone + 'static + From<WizardMessage>,
        B: peak_ui::core::Backend,
    {
        Box::new(
            vstack![
                Text::<B>::new("Welcome Home.")
                    .size(32.0)
                    .bold()
                    .align(Alignment::Center),
                Text::<B>::new("Your PeakOS is ready.")
                    .size(16.0)
                    .secondary()
                    .align(Alignment::Center),
                Space::new(Length::Fill, Length::Fixed(40.0)),
                Button::new(
                    Text::<B>::new("Start using PeakOS")
                        .size(16.0)
                        .align(Alignment::Center),
                )
                .on_press(Message::from(WizardMessage::CompleteSetup))
                .variant(Variant::Solid)
                .intent(Intent::Primary)
                .width(Length::Shrink),
            ]
            .spacing(24.0)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
        )
    }
}
