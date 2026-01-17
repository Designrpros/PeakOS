use iced::Color;

pub struct ThemeTokens {
    pub accent: Color,
    pub background: Color,
    pub glass_opacity: f32,
    pub radius: f32,
}

impl Default for ThemeTokens {
    fn default() -> Self {
        Self {
            accent: Color::from_rgb8(0, 122, 255),
            background: Color::from_rgb8(240, 240, 240),
            glass_opacity: 0.5,
            radius: 12.0,
        }
    }
}
