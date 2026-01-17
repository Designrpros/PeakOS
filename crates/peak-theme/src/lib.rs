use iced::Color;
use peak_core::registry::ShellMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeTone {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeTokens {
    pub accent: Color,
    pub background: Color,
    pub text: Color,
    pub glass_bg: Color,
    pub glass_border: Color,
    pub glass_opacity: f32,
    pub card_bg: Color,
    pub radius: f32,
    pub shadow_color: Color,
    pub divider: Color,
}

impl ThemeTokens {
    pub fn get(mode: ShellMode, tone: ThemeTone) -> Self {
        match (mode, tone) {
            (ShellMode::Peak, ThemeTone::Light) => Self {
                accent: Color::from_rgb8(0, 122, 255),
                background: Color::from_rgb8(247, 245, 242),
                text: Color::from_rgb8(35, 30, 30),
                glass_bg: Color::from_rgba8(247, 245, 242, 0.8),
                glass_border: Color::from_rgba8(35, 30, 30, 0.1),
                glass_opacity: 0.8,
                card_bg: Color::from_rgba8(255, 255, 255, 0.5),
                radius: 12.0,
                shadow_color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                divider: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            },
            (ShellMode::Peak, ThemeTone::Dark) => Self {
                accent: Color::from_rgb8(10, 132, 255),
                background: Color::from_rgb8(15, 14, 14),
                text: Color::from_rgb8(235, 230, 225),
                glass_bg: Color::from_rgba8(15, 14, 14, 0.8),
                glass_border: Color::from_rgba8(235, 230, 225, 0.1),
                glass_opacity: 0.8,
                card_bg: Color::from_rgba8(30, 30, 30, 0.5),
                radius: 12.0,
                shadow_color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                divider: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            },
            (ShellMode::Poolside, _) => Self {
                accent: Color::from_rgb8(255, 0, 127),
                background: Color::from_rgb8(255, 153, 204),
                text: Color::from_rgb8(50, 50, 50),
                glass_bg: Color::from_rgba8(255, 153, 204, 0.7),
                glass_border: Color::from_rgba8(255, 255, 255, 0.3),
                glass_opacity: 0.7,
                card_bg: Color::from_rgba8(255, 255, 255, 0.4),
                radius: 16.0,
                shadow_color: Color::from_rgba(1.0, 0.0, 0.5, 0.2),
                divider: Color::from_rgba(1.0, 1.0, 1.0, 0.3),
            },
        }
    }
}

impl Default for ThemeTokens {
    fn default() -> Self {
        Self::get(ShellMode::Peak, ThemeTone::Light)
    }
}
