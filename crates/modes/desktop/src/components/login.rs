use crate::app::state::AppState;
use crate::app::Message;
use iced::{Alignment, Color, Length};
use peak_ui::prelude::*;

pub struct LoginView<'a> {
    state: &'a AppState,
    is_light: bool,
    user_name: String,
}

impl<'a> LoginView<'a> {
    pub fn new(state: &'a AppState, is_light: bool, user_name: String) -> Self {
        Self {
            state,
            is_light,
            user_name,
        }
    }
}

impl<'a, B> View<Message, B> for LoginView<'a>
where
    B: peak_ui::core::Backend,
{
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let text_color = if self.is_light {
            Color::BLACK
        } else {
            Color::WHITE
        };

        let password_value = if let AppState::Login(ref p) = self.state {
            p.clone()
        } else {
            String::new()
        };

        // Reimplementing logic from view.rs
        let content = B::vstack(
            vec![
                B::text(
                    self.user_name.clone(),
                    32.0,
                    Some(text_color),
                    true,
                    false,
                    None,
                    None,
                    Length::Shrink,
                    Alignment::Center,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(30.0)),
                B::text_input(
                    password_value,
                    "Enter Password".to_string(),
                    Message::UpdateLoginPassword,
                    Some(Message::SubmitLogin),
                    None,
                    true,
                    Variant::Soft,
                    context,
                ),
                B::space(Length::Fill, Length::Fixed(30.0)),
                B::button(
                    B::text(
                        "Login".into(),
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
                    Some(Message::SubmitLogin),
                    Variant::Solid,
                    Intent::Primary,
                    false, // is_compact
                    context,
                ),
            ],
            20.0,
            iced::Padding::new(40.0),
            Length::Fixed(400.0),
            Length::Shrink,
            Alignment::Center,
            Alignment::Center,
            1.0,
        );

        // Wallpaper
        let wallpaper_path =
            peak_core::utils::assets::get_asset_path("wallpapers/mountain_sunset_warm.jpg");
        let wallpaper = B::image(
            wallpaper_path.to_string_lossy().to_string(),
            Length::Fill,
            Length::Fill,
            0.0,
        );

        // Overlay

        let overlay: B::AnyView<Message> = B::container(
            // Explicit Type
            B::space(Length::Fill, Length::Fill),
            iced::Padding::default(),
            Length::Fill,
            Length::Fill,
            context,
        );

        // Logo (Top Left)
        let logo_path = peak_core::utils::assets::get_asset_path(&format!(
            "icons/menubar/{}",
            if self.is_light {
                "peak_logo.png"
            } else {
                "peak_logo_dark.png"
            }
        ));
        let logo = B::container(
            B::image(
                logo_path.to_string_lossy().to_string(),
                Length::Fixed(100.0),
                Length::Fixed(50.0),
                0.0,
            ),
            iced::Padding::new(20.0),
            Length::Shrink,
            Length::Shrink,
            context,
        );

        // Toggle (Top Right)
        let theme_btn = B::button(
            B::text(
                if self.is_light { "moon" } else { "sun" }.into(),
                20.0,
                None,
                false,
                false,
                None,
                None,
                Length::Shrink,
                Alignment::Center,
                context,
            ),
            Some(Message::ToggleTheme),
            Variant::Ghost,
            Intent::Neutral,
            false, // is_compact
            context,
        );
        let top_right = B::container(
            theme_btn,
            iced::Padding::new(20.0),
            Length::Shrink,
            Length::Shrink,
            context,
        );

        let top_bar = B::hstack(
            vec![logo, B::space(Length::Fill, Length::Shrink), top_right],
            0.0,
            iced::Padding::new(0.0),
            Length::Fill,
            Length::Shrink,
            Alignment::Start,
            Alignment::Start,
            1.0,
        );

        // Ensure Top Bar is at the top using VStack with Spacer
        let top_layer = B::vstack(
            vec![top_bar, B::space(Length::Fill, Length::Fill)],
            0.0,
            iced::Padding::new(0.0),
            Length::Fill,
            Length::Fill,
            Alignment::Start,
            Alignment::Start,
            1.0,
        );

        // Centered Card
        let centered_layer = B::vstack(
            vec![
                B::space(Length::Fill, Length::Fill),
                content,
                B::space(Length::Fill, Length::Fill),
            ],
            0.0,
            iced::Padding::new(0.0),
            Length::Fill,
            Length::Fill,
            Alignment::Center,
            Alignment::Center,
            1.0,
        );

        B::zstack(
            vec![wallpaper, overlay, centered_layer, top_layer],
            Length::Fill,
            Length::Fill,
            Alignment::Center,
        )
    }
}
