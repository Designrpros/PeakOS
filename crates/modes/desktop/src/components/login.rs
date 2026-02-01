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

        // UI Definition
        let content = vstack![
            Text::<B>::new(self.user_name.clone())
                .size(32.0)
                .color(text_color)
                .bold(),
            Space::new(Length::Fill, Length::Fixed(30.0)),
            TextInput::new(
                password_value,
                "Enter Password",
                Message::UpdateLoginPassword,
            )
            .on_submit(Message::SubmitLogin)
            .password()
            .variant(Variant::Soft),
            Space::new(Length::Fill, Length::Fixed(30.0)),
            Button::new(Text::<B>::new("Login").size(16.0).align(Alignment::Center))
                .on_press(Message::SubmitLogin)
                .variant(Variant::Solid)
                .intent(Intent::Primary)
                .width(Length::Shrink)
        ]
        .spacing(20.0)
        .padding(40.0)
        .width(Length::Fixed(400.0))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        // Wallpaper
        let wallpaper_path =
            peak_core::utils::assets::get_asset_path("wallpapers/mountain_sunset_warm.jpg");
        let wallpaper = Image::new(wallpaper_path.to_string_lossy().to_string())
            .width(Length::Fill)
            .height(Length::Fill);

        // Overlay
        let overlay = Container::new(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill);

        // Logo (Top Left)
        let logo_path = peak_core::utils::assets::get_asset_path(&format!(
            "icons/menubar/{}",
            if self.is_light {
                "peak_logo.png"
            } else {
                "peak_logo_dark.png"
            }
        ));
        let logo = Container::new(
            Image::new(logo_path.to_string_lossy().to_string())
                .width(Length::Fixed(100.0))
                .height(Length::Fixed(50.0)),
        )
        .padding(20.0);

        // Toggle (Top Right)
        let theme_btn = Button::new(
            Text::<B>::new(if self.is_light { "moon" } else { "sun" })
                .size(20.0)
                .align(Alignment::Center),
        )
        .on_press(Message::ToggleTheme)
        .variant(Variant::Ghost)
        .intent(Intent::Neutral)
        .width(Length::Shrink);

        let top_right = Container::new(theme_btn).padding(20.0);

        let top_bar = hstack![logo, Space::new(Length::Fill, Length::Shrink), top_right]
            .width(Length::Fill)
            .align_y(Alignment::Center);

        // Ensure Top Bar is at the top
        let top_layer = vstack![top_bar, Space::new(Length::Fill, Length::Fill)]
            .width(Length::Fill)
            .height(Length::Fill);

        // Centered Card
        let centered_layer = vstack![
            Space::new(Length::Fill, Length::Fill),
            content,
            Space::new(Length::Fill, Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        zstack![wallpaper, overlay, centered_layer, top_layer]
            .width(Length::Fill)
            .height(Length::Fill)
            .view(context)
    }
}
