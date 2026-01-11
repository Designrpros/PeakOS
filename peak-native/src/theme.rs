use iced::{Color, Theme as NativeTheme};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Cyber, // The "Game Mode" (Neon/Dark)
    Light, // The "Zen Mode" (Paper/Ink)
    Dark,  // Standard System Dark
}

impl Theme {
    pub fn palette(&self) -> Palette {
        match self {
            Theme::Cyber => Palette {
                // Keep Cyber as the "Special" Game Mode
                background: Color::from_rgb8(10, 10, 15),
                text: Color::from_rgb8(0, 255, 200),
                primary: Color::from_rgb8(255, 0, 128),
                surface: Color::from_rgb8(20, 20, 30),
                border: Color::from_rgba(0.0, 1.0, 0.78, 0.1),
                accent: Color::from_rgb8(255, 0, 128),
            },
            Theme::Light => Palette {
                // Warm Stone (Light): Background #F7F5F2, Text #231E1E
                background: Color::from_rgb8(247, 245, 242),
                text: Color::from_rgb8(35, 30, 30),
                primary: Color::from_rgb8(35, 30, 30),
                surface: Color::WHITE,
                border: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                accent: Color::from_rgb8(50, 50, 50),
            },
            Theme::Dark => Palette {
                // Obsidian (Dark): Deep warm blacks and subtle highlights
                background: Color::from_rgb8(15, 14, 14), // #0F0E0E
                text: Color::from_rgb8(235, 230, 225),
                primary: Color::from_rgb8(235, 230, 225),
                surface: Color::from_rgb8(22, 21, 21), // #161515
                border: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                accent: Color::from_rgb8(210, 205, 200),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub background: Color,
    pub text: Color,
    pub primary: Color,
    pub surface: Color,
    pub border: Color,
    pub accent: Color,
}

// Map the enum to Iced's internal styling
impl From<Theme> for NativeTheme {
    fn from(theme: Theme) -> Self {
        // You can return built-in themes or fully custom ones here
        match theme {
            Theme::Light => NativeTheme::Light,
            Theme::Dark => NativeTheme::Dark,
            Theme::Cyber => NativeTheme::Dark, // Pending custom definition
        }
    }
}
