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
        match mode {
            ShellMode::Desktop => match tone {
                ThemeTone::Light => Self {
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
                ThemeTone::Dark => Self {
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
            },
            ShellMode::Mobile => match tone {
                ThemeTone::Light => Self {
                    accent: Color::from_rgb8(255, 45, 85),
                    background: Color::from_rgb8(255, 255, 255),
                    text: Color::BLACK,
                    glass_bg: Color::from_rgba(1.0, 1.0, 1.0, 0.9),
                    glass_border: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    glass_opacity: 0.9,
                    card_bg: Color::from_rgba(0.95, 0.95, 0.95, 1.0),
                    radius: 20.0,
                    shadow_color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    divider: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                },
                ThemeTone::Dark => Self {
                    accent: Color::from_rgb8(255, 55, 95),
                    background: Color::BLACK,
                    text: Color::WHITE,
                    glass_bg: Color::from_rgba(0.1, 0.1, 0.1, 0.9),
                    glass_border: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                    glass_opacity: 0.9,
                    card_bg: Color::from_rgba(0.15, 0.15, 0.15, 1.0),
                    radius: 20.0,
                    shadow_color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                    divider: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                },
            },
            ShellMode::Console => Self {
                accent: Color::from_rgb8(50, 255, 50),
                background: Color::from_rgb8(10, 10, 10),
                text: Color::from_rgb8(240, 240, 240),
                glass_bg: Color::from_rgba(0.0, 0.0, 0.0, 0.6),
                glass_border: Color::from_rgba(0.2, 1.0, 0.2, 0.3),
                glass_opacity: 0.6,
                card_bg: Color::from_rgba(0.1, 0.1, 0.1, 0.8),
                radius: 4.0,
                shadow_color: Color::from_rgba(0.0, 1.0, 0.0, 0.1),
                divider: Color::from_rgba(0.0, 1.0, 0.0, 0.2),
            },
            ShellMode::Fireplace => Self {
                accent: Color::from_rgb8(255, 150, 50),
                background: Color::from_rgb8(20, 10, 5),
                text: Color::from_rgb8(255, 230, 200),
                glass_bg: Color::from_rgba(0.1, 0.05, 0.0, 0.4),
                glass_border: Color::from_rgba(1.0, 0.6, 0.0, 0.1),
                glass_opacity: 0.4,
                card_bg: Color::from_rgba(1.0, 0.6, 0.0, 0.05),
                radius: 30.0,
                shadow_color: Color::from_rgba(1.0, 0.4, 0.0, 0.1),
                divider: Color::from_rgba(1.0, 0.6, 0.0, 0.1),
            },
            ShellMode::Robot => Self {
                accent: Color::from_rgb8(255, 200, 0),
                background: Color::from_rgb8(30, 30, 35),
                text: Color::from_rgb8(200, 200, 210),
                glass_bg: Color::from_rgba(0.1, 0.1, 0.15, 0.95),
                glass_border: Color::from_rgba(1.0, 0.8, 0.0, 0.5),
                glass_opacity: 0.95,
                card_bg: Color::from_rgba(0.2, 0.2, 0.25, 1.0),
                radius: 0.0,
                shadow_color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                divider: Color::from_rgba(1.0, 0.8, 0.0, 0.1),
            },
            ShellMode::Server => Self {
                accent: Color::from_rgb8(0, 255, 65),
                background: Color::from_rgb8(5, 5, 5),
                text: Color::from_rgb8(0, 255, 65),
                glass_bg: Color::from_rgba(0.0, 0.1, 0.0, 0.9),
                glass_border: Color::from_rgba(0.0, 1.0, 0.0, 0.2),
                glass_opacity: 0.9,
                card_bg: Color::from_rgba(0.0, 0.0, 0.0, 1.0),
                radius: 2.0,
                shadow_color: Color::TRANSPARENT,
                divider: Color::from_rgba(0.0, 1.0, 0.0, 0.3),
            },
            ShellMode::SmartHome => Self {
                accent: Color::from_rgb8(100, 210, 255),
                background: Color::from_rgb8(250, 252, 255),
                text: Color::from_rgb8(40, 60, 80),
                glass_bg: Color::from_rgba(1.0, 1.0, 1.0, 0.9),
                glass_border: Color::from_rgba(0.5, 0.8, 1.0, 0.2),
                glass_opacity: 0.9,
                card_bg: Color::WHITE,
                radius: 24.0,
                shadow_color: Color::from_rgba(0.0, 0.3, 0.6, 0.05),
                divider: Color::from_rgba(0.0, 0.0, 0.0, 0.03),
            },
            ShellMode::TV => Self {
                accent: Color::from_rgb8(255, 255, 255),
                background: Color::from_rgb8(15, 15, 25),
                text: Color::WHITE,
                glass_bg: Color::from_rgba(0.0, 0.0, 0.0, 0.8),
                glass_border: Color::from_rgba(1.0, 1.0, 1.0, 0.2),
                glass_opacity: 0.8,
                card_bg: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                radius: 16.0,
                shadow_color: Color::from_rgba(0.0, 0.0, 0.0, 0.8),
                divider: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            },
            ShellMode::Auto => Self {
                accent: Color::from_rgb8(255, 0, 0),
                background: Color::from_rgb8(10, 10, 10),
                text: Color::WHITE,
                glass_bg: Color::from_rgba(0.1, 0.1, 0.1, 1.0),
                glass_border: Color::from_rgba(1.0, 0.0, 0.0, 0.3),
                glass_opacity: 1.0,
                card_bg: Color::from_rgba(0.2, 0.2, 0.2, 1.0),
                radius: 40.0,
                shadow_color: Color::BLACK,
                divider: Color::from_rgba(1.0, 0.0, 0.0, 0.2),
            },
            ShellMode::Kiosk => Self {
                accent: Color::from_rgb8(0, 0, 0),
                background: Color::WHITE,
                text: Color::BLACK,
                glass_bg: Color::WHITE,
                glass_border: Color::BLACK,
                glass_opacity: 1.0,
                card_bg: Color::WHITE,
                radius: 0.0,
                shadow_color: Color::TRANSPARENT,
                divider: Color::BLACK,
            },
        }
    }
}

impl Default for ThemeTokens {
    fn default() -> Self {
        Self::get(ShellMode::Desktop, ThemeTone::Light)
    }
}
