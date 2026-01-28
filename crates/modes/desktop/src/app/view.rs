// Main view rendering

use super::{AppState, Message, PeakNative};
use iced::Element;

impl PeakNative {
    pub fn view(&self) -> Element<'_, Message> {
        // FIXED: Subprocesses (Bar/Dock) must always render their specific views,
        // ignoring the AppState (which defaults to Setup on fresh boot).
        if self.launch_mode != crate::app::LaunchMode::Desktop {
            return self.view_desktop();
        }

        let content = match &self.state {
            AppState::Setup(wizard_state) => self.view_setup(wizard_state),
            AppState::Login(_) => self.view_login(),
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

    fn view_login(&self) -> Element<'_, Message> {
        use crate::components::login::LoginView;
        use peak_ui::core::Context;
        use peak_ui_theme::{ThemeTokens, ThemeTone};

        let ui_mode = peak_ui::core::ShellMode::Desktop;
        let tone = if matches!(self.theme, peak_core::theme::Theme::Light) {
            ThemeTone::Light
        } else {
            ThemeTone::Dark
        };
        let ui_tokens = ThemeTokens::get(ui_mode, tone);

        let context = Context::new(
            ui_mode,
            ui_tokens,
            self.window_manager.screen_size,
            peak_ui::localization::Localization::default(),
        )
        .with_safe_area(iced::Padding::default());

        let user_name = self
            .user
            .as_ref()
            .map(|u| u.full_name.clone())
            .unwrap_or("User".to_string());
        let is_light = matches!(self.theme, peak_core::theme::Theme::Light);

        use peak_ui::core::IcedBackend;
        peak_ui::core::View::<Message, IcedBackend>::view(
            &LoginView::new(&self.state, is_light, user_name),
            &context,
        )
    }

    fn view_setup<'a>(&'a self, state: &'a peak_apps::wizard::WizardState) -> Element<'a, Message> {
        use crate::components::wizard::WizardView;
        use peak_ui::core::Context;
        use peak_ui_theme::{ThemeTokens, ThemeTone};

        let ui_mode = peak_ui::core::ShellMode::Desktop;
        let tone = if matches!(self.theme, peak_core::theme::Theme::Light) {
            ThemeTone::Light
        } else {
            ThemeTone::Dark
        };
        let ui_tokens = ThemeTokens::get(ui_mode, tone);

        let context = Context::new(
            ui_mode,
            ui_tokens,
            self.window_manager.screen_size,
            peak_ui::localization::Localization::default(),
        )
        .with_safe_area(iced::Padding::default());

        use peak_ui::core::IcedBackend;
        peak_ui::core::View::<Message, IcedBackend>::view(&WizardView::new(state.clone()), &context)
    }
}
