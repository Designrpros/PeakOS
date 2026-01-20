use crate::colors::PeakColors;
use crate::ThemeTone;
use iced::Color;
use peak_core::registry::ShellMode;

/// Available themes in PeakUI
///
/// Each theme provides a complete visual identity with semantic colors
/// that can be applied to any OS mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PeakTheme {
    /// macOS-inspired clean design (current Desktop theme)
    Cupertino,
    /// Vibrant gaming aesthetic (Xbox/PlayStation style)
    Gaming,
    /// High contrast for 10-foot TV viewing
    MediaCenter,
    /// Warm, cozy ambient theme (Fireplace mode)
    Ambient,
    /// Monochrome, data-focused (Robot/Server mode)
    Terminal,
    /// High contrast dashboard (Automotive mode)
    Automotive,
    /// Modern control interface (SmartHome mode)
    Smart,
    /// Google Material Design
    Material,
    /// Microsoft Fluent Design
    Fluent,
    /// High contrast for accessibility
    HighContrast,
}

impl PeakTheme {
    /// Get the Peak Colors for this theme
    pub fn colors(self, tone: ThemeTone) -> PeakColors {
        match self {
            PeakTheme::Cupertino => Self::cupertino_colors(tone),
            PeakTheme::Gaming => Self::gaming_colors(tone),
            PeakTheme::MediaCenter => Self::media_center_colors(tone),
            PeakTheme::Ambient => Self::ambient_colors(tone),
            PeakTheme::Terminal => Self::terminal_colors(tone),
            PeakTheme::Automotive => Self::automotive_colors(tone),
            PeakTheme::Smart => Self::smart_colors(tone),
            PeakTheme::Material => Self::material_colors(tone),
            PeakTheme::Fluent => Self::fluent_colors(tone),
            PeakTheme::HighContrast => Self::high_contrast_colors(tone),
        }
    }

    /// Get the recommended default theme for a shell mode
    pub fn default_for_mode(mode: ShellMode) -> Self {
        match mode {
            ShellMode::Desktop => PeakTheme::Cupertino,
            ShellMode::Mobile => PeakTheme::Cupertino,
            ShellMode::Console => PeakTheme::Gaming,
            ShellMode::TV => PeakTheme::MediaCenter,
            ShellMode::Robot => PeakTheme::Terminal,
            ShellMode::Auto => PeakTheme::Automotive,
            ShellMode::Fireplace => PeakTheme::Ambient,
            ShellMode::SmartHome => PeakTheme::Smart,
            ShellMode::Kiosk => PeakTheme::HighContrast,
            ShellMode::Server => PeakTheme::Terminal,
        }
    }

    /// Get display name for UI
    pub fn display_name(self) -> &'static str {
        match self {
            PeakTheme::Cupertino => "Cupertino",
            PeakTheme::Gaming => "Gaming",
            PeakTheme::MediaCenter => "Media Center",
            PeakTheme::Ambient => "Ambient",
            PeakTheme::Terminal => "Terminal",
            PeakTheme::Automotive => "Automotive",
            PeakTheme::Smart => "Smart",
            PeakTheme::Material => "Material",
            PeakTheme::Fluent => "Fluent",
            PeakTheme::HighContrast => "High Contrast",
        }
    }

    /// Get all available themes
    pub fn all() -> &'static [PeakTheme] {
        &[
            PeakTheme::Cupertino,
            PeakTheme::Gaming,
            PeakTheme::MediaCenter,
            PeakTheme::Ambient,
            PeakTheme::Terminal,
            PeakTheme::Automotive,
            PeakTheme::Smart,
            PeakTheme::Material,
            PeakTheme::Fluent,
            PeakTheme::HighContrast,
        ]
    }

    // === Theme Color Implementations ===

    fn cupertino_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::from_rgb8(0, 122, 255),
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(200, 230, 255),
                on_primary_container: Color::from_rgb8(0, 50, 100),

                secondary: Color::from_rgb8(90, 200, 250),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(220, 240, 255),
                on_secondary_container: Color::from_rgb8(0, 50, 80),

                accent: Color::from_rgb8(255, 59, 48),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(52, 199, 89),
                warning: Color::from_rgb8(255, 149, 0),
                danger: Color::from_rgb8(255, 59, 48),
                info: Color::from_rgb8(0, 122, 255),

                surface: Color::WHITE,
                on_surface: Color::from_rgb8(25, 25, 25),
                surface_variant: Color::from_rgb8(242, 242, 247),
                on_surface_variant: Color::from_rgb8(60, 60, 67),

                background: Color::from_rgb8(242, 242, 247),
                on_background: Color::from_rgb8(25, 25, 25),

                border: Color::from_rgba8(0, 0, 0, 25.0),
                divider: Color::from_rgba8(0, 0, 0, 20.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.4),

                text_primary: Color::from_rgb8(25, 25, 25),
                text_secondary: Color::from_rgba8(25, 25, 25, 153.0),
                text_tertiary: Color::from_rgba8(25, 25, 25, 97.0),
                text_disabled: Color::from_rgba8(25, 25, 25, 66.0),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::from_rgb8(10, 132, 255),
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(0, 64, 128),
                on_primary_container: Color::from_rgb8(180, 220, 255),

                secondary: Color::from_rgb8(100, 210, 255),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(0, 80, 120),
                on_secondary_container: Color::from_rgb8(200, 235, 255),

                accent: Color::from_rgb8(255, 69, 58),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(48, 209, 88),
                warning: Color::from_rgb8(255, 159, 10),
                danger: Color::from_rgb8(255, 69, 58),
                info: Color::from_rgb8(10, 132, 255),

                surface: Color::from_rgb8(28, 28, 30),
                on_surface: Color::from_rgb8(242, 242, 247),
                surface_variant: Color::from_rgb8(44, 44, 46),
                on_surface_variant: Color::from_rgb8(199, 199, 204),

                background: Color::from_rgb8(16, 16, 18),
                on_background: Color::from_rgb8(242, 242, 247),

                border: Color::from_rgba8(255, 255, 255, 25.0),
                divider: Color::from_rgba8(255, 255, 255, 25.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.6),

                text_primary: Color::from_rgb8(242, 242, 247),
                text_secondary: Color::from_rgba8(242, 242, 247, 153.0),
                text_tertiary: Color::from_rgba8(242, 242, 247, 97.0),
                text_disabled: Color::from_rgba8(242, 242, 247, 66.0),
            },
        }
    }

    fn gaming_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::from_rgb8(0, 255, 127),
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(200, 255, 220),
                on_primary_container: Color::from_rgb8(0, 80, 40),

                secondary: Color::from_rgb8(138, 43, 226),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(220, 200, 240),
                on_secondary_container: Color::from_rgb8(60, 0, 100),

                accent: Color::from_rgb8(255, 20, 147),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(0, 255, 127),
                warning: Color::from_rgb8(255, 215, 0),
                danger: Color::from_rgb8(255, 69, 0),
                info: Color::from_rgb8(0, 191, 255),

                surface: Color::from_rgb8(40, 40, 50),
                on_surface: Color::from_rgb8(240, 240, 255),
                surface_variant: Color::from_rgb8(60, 60, 70),
                on_surface_variant: Color::from_rgb8(220, 220, 230),

                background: Color::from_rgb8(20, 20, 30),
                on_background: Color::from_rgb8(240, 240, 255),

                border: Color::from_rgba8(0, 255, 127, 80.0),
                divider: Color::from_rgba8(138, 43, 226, 60.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.7),

                text_primary: Color::from_rgb8(240, 240, 255),
                text_secondary: Color::from_rgba8(200, 200, 220, 200.0),
                text_tertiary: Color::from_rgba8(160, 160, 180, 150.0),
                text_disabled: Color::from_rgba8(120, 120, 140, 100.0),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::from_rgb8(0, 255, 127),
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(0, 120, 60),
                on_primary_container: Color::from_rgb8(200, 255, 220),

                secondary: Color::from_rgb8(138, 43, 226),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(80, 20, 130),
                on_secondary_container: Color::from_rgb8(220, 200, 240),

                accent: Color::from_rgb8(255, 20, 147),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(0, 255, 127),
                warning: Color::from_rgb8(255, 215, 0),
                danger: Color::from_rgb8(255, 69, 0),
                info: Color::from_rgb8(0, 191, 255),

                surface: Color::from_rgb8(30, 30, 40),
                on_surface: Color::from_rgb8(240, 240, 255),
                surface_variant: Color::from_rgb8(50, 50, 60),
                on_surface_variant: Color::from_rgb8(220, 220, 230),

                background: Color::from_rgb8(10, 10, 20),
                on_background: Color::from_rgb8(240, 240, 255),

                border: Color::from_rgba8(0, 255, 127, 100.0),
                divider: Color::from_rgba8(138, 43, 226, 80.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.8),

                text_primary: Color::from_rgb8(240, 240, 255),
                text_secondary: Color::from_rgba8(200, 200, 220, 200.0),
                text_tertiary: Color::from_rgba8(160, 160, 180, 150.0),
                text_disabled: Color::from_rgba8(120, 120, 140, 100.0),
            },
        }
    }

    fn media_center_colors(tone: ThemeTone) -> PeakColors {
        // TV mode is always dark for cinema experience
        let _ = tone;
        PeakColors {
            primary: Color::from_rgb8(255, 255, 255),
            on_primary: Color::BLACK,
            primary_container: Color::from_rgb8(60, 60, 80),
            on_primary_container: Color::WHITE,

            secondary: Color::from_rgb8(200, 200, 220),
            on_secondary: Color::BLACK,
            secondary_container: Color::from_rgb8(40, 40, 60),
            on_secondary_container: Color::from_rgb8(220, 220, 240),

            accent: Color::from_rgb8(255, 179, 0),
            on_accent: Color::BLACK,

            success: Color::from_rgb8(76, 217, 100),
            warning: Color::from_rgb8(255, 179, 0),
            danger: Color::from_rgb8(255, 69, 58),
            info: Color::from_rgb8(90, 200, 250),

            surface: Color::from_rgb8(25, 25, 35),
            on_surface: Color::WHITE,
            surface_variant: Color::from_rgb8(40, 40, 50),
            on_surface_variant: Color::from_rgb8(220, 220, 230),

            background: Color::from_rgb8(15, 15, 25),
            on_background: Color::WHITE,

            border: Color::from_rgba8(255, 255, 255, 50.0),
            divider: Color::from_rgba8(255, 255, 255, 30.0),
            overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.8),

            text_primary: Color::WHITE,
            text_secondary: Color::from_rgba8(255, 255, 255, 180.0),
            text_tertiary: Color::from_rgba8(255, 255, 255, 120.0),
            text_disabled: Color::from_rgba8(255, 255, 255, 80.0),
        }
    }

    fn ambient_colors(_tone: ThemeTone) -> PeakColors {
        // Fireplace is always dark with warm tones
        PeakColors {
            primary: Color::from_rgb8(255, 150, 50),
            on_primary: Color::from_rgb8(40, 20, 0),
            primary_container: Color::from_rgb8(120, 60, 20),
            on_primary_container: Color::from_rgb8(255, 220, 180),

            secondary: Color::from_rgb8(255, 120, 40),
            on_secondary: Color::from_rgb8(60, 30, 0),
            secondary_container: Color::from_rgb8(100, 50, 20),
            on_secondary_container: Color::from_rgb8(255, 200, 150),

            accent: Color::from_rgb8(255, 180, 80),
            on_accent: Color::from_rgb8(40, 20, 0),

            success: Color::from_rgb8(200, 180, 100),
            warning: Color::from_rgb8(255, 150, 50),
            danger: Color::from_rgb8(255, 100, 60),
            info: Color::from_rgb8(180, 150, 120),

            surface: Color::from_rgb8(40, 25, 15),
            on_surface: Color::from_rgb8(255, 230, 200),
            surface_variant: Color::from_rgb8(60, 35, 20),
            on_surface_variant: Color::from_rgb8(240, 210, 180),

            background: Color::from_rgb8(20, 10, 5),
            on_background: Color::from_rgb8(255, 230, 200),

            border: Color::from_rgba8(255, 150, 50, 50.0),
            divider: Color::from_rgba8(255, 150, 50, 30.0),
            overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.7),

            text_primary: Color::from_rgb8(255, 230, 200),
            text_secondary: Color::from_rgba8(255, 200, 150, 180.0),
            text_tertiary: Color::from_rgba8(255, 180, 120, 120.0),
            text_disabled: Color::from_rgba8(200, 150, 100, 80.0),
        }
    }

    fn terminal_colors(_tone: ThemeTone) -> PeakColors {
        // Terminal is always dark monochrome
        PeakColors {
            primary: Color::from_rgb8(0, 255, 65),
            on_primary: Color::BLACK,
            primary_container: Color::from_rgb8(0, 100, 30),
            on_primary_container: Color::from_rgb8(200, 255, 200),

            secondary: Color::from_rgb8(0, 200, 50),
            on_secondary: Color::BLACK,
            secondary_container: Color::from_rgb8(0, 80, 25),
            on_secondary_container: Color::from_rgb8(180, 255, 180),

            accent: Color::from_rgb8(0, 255, 255),
            on_accent: Color::BLACK,

            success: Color::from_rgb8(0, 255, 65),
            warning: Color::from_rgb8(255, 255, 0),
            danger: Color::from_rgb8(255, 85, 85),
            info: Color::from_rgb8(85, 255, 255),

            surface: Color::from_rgb8(10, 10, 10),
            on_surface: Color::from_rgb8(0, 255, 65),
            surface_variant: Color::from_rgb8(20, 20, 20),
            on_surface_variant: Color::from_rgb8(0, 220, 55),

            background: Color::from_rgb8(5, 5, 5),
            on_background: Color::from_rgb8(0, 255, 65),

            border: Color::from_rgba8(0, 255, 65, 80.0),
            divider: Color::from_rgba8(0, 255, 65, 50.0),
            overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.9),

            text_primary: Color::from_rgb8(0, 255, 65),
            text_secondary: Color::from_rgba8(0, 255, 65, 180.0),
            text_tertiary: Color::from_rgba8(0, 255, 65, 120.0),
            text_disabled: Color::from_rgba8(0, 255, 65, 80.0),
        }
    }

    fn automotive_colors(_tone: ThemeTone) -> PeakColors {
        // Automotive dashboard - always dark, high contrast
        PeakColors {
            primary: Color::from_rgb8(255, 0, 0),
            on_primary: Color::WHITE,
            primary_container: Color::from_rgb8(120, 0, 0),
            on_primary_container: Color::from_rgb8(255, 200, 200),

            secondary: Color::from_rgb8(0, 200, 255),
            on_secondary: Color::BLACK,
            secondary_container: Color::from_rgb8(0, 80, 120),
            on_secondary_container: Color::from_rgb8(200, 240, 255),

            accent: Color::from_rgb8(255, 215, 0),
            on_accent: Color::BLACK,

            success: Color::from_rgb8(0, 255, 100),
            warning: Color::from_rgb8(255, 215, 0),
            danger: Color::from_rgb8(255, 0, 0),
            info: Color::from_rgb8(0, 200, 255),

            surface: Color::from_rgb8(20, 20, 20),
            on_surface: Color::WHITE,
            surface_variant: Color::from_rgb8(40, 40, 40),
            on_surface_variant: Color::from_rgb8(220, 220, 220),

            background: Color::from_rgb8(10, 10, 10),
            on_background: Color::WHITE,

            border: Color::from_rgba8(255, 0, 0, 80.0),
            divider: Color::from_rgba8(255, 255, 255, 40.0),
            overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.85),

            text_primary: Color::WHITE,
            text_secondary: Color::from_rgba8(255, 255, 255, 200.0),
            text_tertiary: Color::from_rgba8(255, 255, 255, 140.0),
            text_disabled: Color::from_rgba8(255, 255, 255, 80.0),
        }
    }

    fn smart_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::from_rgb8(100, 210, 255),
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(220, 245, 255),
                on_primary_container: Color::from_rgb8(0, 60, 90),

                secondary: Color::from_rgb8(80, 180, 230),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(200, 235, 250),
                on_secondary_container: Color::from_rgb8(0, 50, 80),

                accent: Color::from_rgb8(120, 220, 255),
                on_accent: Color::BLACK,

                success: Color::from_rgb8(76, 217, 100),
                warning: Color::from_rgb8(255, 179, 0),
                danger: Color::from_rgb8(255, 59, 48),
                info: Color::from_rgb8(100, 210, 255),

                surface: Color::WHITE,
                on_surface: Color::from_rgb8(40, 60, 80),
                surface_variant: Color::from_rgb8(245, 248, 252),
                on_surface_variant: Color::from_rgb8(70, 90, 110),

                background: Color::from_rgb8(250, 252, 255),
                on_background: Color::from_rgb8(40, 60, 80),

                border: Color::from_rgba8(100, 210, 255, 40.0),
                divider: Color::from_rgba8(0, 0, 0, 15.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.3),

                text_primary: Color::from_rgb8(40, 60, 80),
                text_secondary: Color::from_rgba8(40, 60, 80, 170.0),
                text_tertiary: Color::from_rgba8(40, 60, 80, 120.0),
                text_disabled: Color::from_rgba8(40, 60, 80, 80.0),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::from_rgb8(100, 210, 255),
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(0, 80, 120),
                on_primary_container: Color::from_rgb8(220, 245, 255),

                secondary: Color::from_rgb8(120, 200, 240),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(0, 60, 100),
                on_secondary_container: Color::from_rgb8(200, 235, 250),

                accent: Color::from_rgb8(140, 230, 255),
                on_accent: Color::BLACK,

                success: Color::from_rgb8(76, 217, 100),
                warning: Color::from_rgb8(255, 179, 0),
                danger: Color::from_rgb8(255, 69, 58),
                info: Color::from_rgb8(100, 210, 255),

                surface: Color::from_rgb8(25, 30, 35),
                on_surface: Color::from_rgb8(230, 235, 240),
                surface_variant: Color::from_rgb8(40, 45, 50),
                on_surface_variant: Color::from_rgb8(200, 210, 220),

                background: Color::from_rgb8(15, 20, 25),
                on_background: Color::from_rgb8(230, 235, 240),

                border: Color::from_rgba8(100, 210, 255, 60.0),
                divider: Color::from_rgba8(255, 255, 255, 25.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.6),

                text_primary: Color::from_rgb8(230, 235, 240),
                text_secondary: Color::from_rgba8(230, 235, 240, 180.0),
                text_tertiary: Color::from_rgba8(230, 235, 240, 120.0),
                text_disabled: Color::from_rgba8(230, 235, 240, 80.0),
            },
        }
    }

    fn material_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::from_rgb8(103, 80, 164),
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(225, 218, 238),
                on_primary_container: Color::from_rgb8(50, 30, 90),

                secondary: Color::from_rgb8(3, 169, 244),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(200, 235, 250),
                on_secondary_container: Color::from_rgb8(0, 60, 90),

                accent: Color::from_rgb8(255, 64, 129),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(76, 175, 80),
                warning: Color::from_rgb8(255, 152, 0),
                danger: Color::from_rgb8(244, 67, 54),
                info: Color::from_rgb8(33, 150, 243),

                surface: Color::WHITE,
                on_surface: Color::from_rgba8(0, 0, 0, 222.0),
                surface_variant: Color::from_rgb8(245, 245, 245),
                on_surface_variant: Color::from_rgba8(0, 0, 0, 180.0),

                background: Color::from_rgb8(250, 250, 250),
                on_background: Color::from_rgba8(0, 0, 0, 222.0),

                border: Color::from_rgba8(0, 0, 0, 30.0),
                divider: Color::from_rgba8(0, 0, 0, 30.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.4),

                text_primary: Color::from_rgba8(0, 0, 0, 222.0),
                text_secondary: Color::from_rgba8(0, 0, 0, 138.0),
                text_tertiary: Color::from_rgba8(0, 0, 0, 97.0),
                text_disabled: Color::from_rgba8(0, 0, 0, 97.0),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::from_rgb8(187, 134, 252),
                on_primary: Color::from_rgb8(50, 0, 90),
                primary_container: Color::from_rgb8(80, 50, 120),
                on_primary_container: Color::from_rgb8(225, 218, 238),

                secondary: Color::from_rgb8(100, 210, 255),
                on_secondary: Color::from_rgb8(0, 50, 80),
                secondary_container: Color::from_rgb8(0, 70, 110),
                on_secondary_container: Color::from_rgb8(200, 235, 250),

                accent: Color::from_rgb8(255, 110, 160),
                on_accent: Color::from_rgb8(100, 0, 50),

                success: Color::from_rgb8(129, 199, 132),
                warning: Color::from_rgb8(255, 183, 77),
                danger: Color::from_rgb8(229, 115, 115),
                info: Color::from_rgb8(100, 181, 246),

                surface: Color::from_rgb8(30, 30, 30),
                on_surface: Color::WHITE,
                surface_variant: Color::from_rgb8(45, 45, 45),
                on_surface_variant: Color::from_rgb8(200, 200, 200),

                background: Color::from_rgb8(18, 18, 18),
                on_background: Color::WHITE,

                border: Color::from_rgba8(255, 255, 255, 30.0),
                divider: Color::from_rgba8(255, 255, 255, 30.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.6),

                text_primary: Color::WHITE,
                text_secondary: Color::from_rgba8(255, 255, 255, 180.0),
                text_tertiary: Color::from_rgba8(255, 255, 255, 120.0),
                text_disabled: Color::from_rgba8(255, 255, 255, 97.0),
            },
        }
    }

    fn fluent_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::from_rgb8(0, 120, 212),
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(210, 236, 255),
                on_primary_container: Color::from_rgb8(0, 50, 90),

                secondary: Color::from_rgb8(102, 102, 102),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(230, 230, 230),
                on_secondary_container: Color::from_rgb8(40, 40, 40),

                accent: Color::from_rgb8(16, 110, 190),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(16, 124, 16),
                warning: Color::from_rgb8(255, 185, 0),
                danger: Color::from_rgb8(232, 17, 35),
                info: Color::from_rgb8(0, 120, 212),

                surface: Color::from_rgb8(243, 243, 243),
                on_surface: Color::from_rgb8(26, 26, 26),
                surface_variant: Color::from_rgb8(250, 250, 250),
                on_surface_variant: Color::from_rgb8(70, 70, 70),

                background: Color::from_rgb8(255, 255, 255),
                on_background: Color::from_rgb8(26, 26, 26),

                border: Color::from_rgba8(0, 0, 0, 38.0),
                divider: Color::from_rgba8(0, 0, 0, 26.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.4),

                text_primary: Color::from_rgb8(26, 26, 26),
                text_secondary: Color::from_rgba8(0, 0, 0, 144.0),
                text_tertiary: Color::from_rgba8(0, 0, 0, 100.0),
                text_disabled: Color::from_rgba8(0, 0, 0, 87.0),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::from_rgb8(96, 205, 255),
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(0, 70, 120),
                on_primary_container: Color::from_rgb8(210, 236, 255),

                secondary: Color::from_rgb8(155, 155, 155),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(60, 60, 60),
                on_secondary_container: Color::from_rgb8(220, 220, 220),

                accent: Color::from_rgb8(96, 205, 255),
                on_accent: Color::BLACK,

                success: Color::from_rgb8(108, 203, 95),
                warning: Color::from_rgb8(252, 225, 0),
                danger: Color::from_rgb8(255, 67, 67),
                info: Color::from_rgb8(96, 205, 255),

                surface: Color::from_rgb8(31, 31, 31),
                on_surface: Color::from_rgb8(243, 243, 243),
                surface_variant: Color::from_rgb8(45, 45, 45),
                on_surface_variant: Color::from_rgb8(200, 200, 200),

                background: Color::from_rgb8(22, 22, 22),
                on_background: Color::from_rgb8(243, 243, 243),

                border: Color::from_rgba8(255, 255, 255, 40.0),
                divider: Color::from_rgba8(255, 255, 255, 30.0),
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.6),

                text_primary: Color::from_rgb8(243, 243, 243),
                text_secondary: Color::from_rgba8(255, 255, 255, 180.0),
                text_tertiary: Color::from_rgba8(255, 255, 255, 130.0),
                text_disabled: Color::from_rgba8(255, 255, 255, 100.0),
            },
        }
    }

    fn high_contrast_colors(tone: ThemeTone) -> PeakColors {
        match tone {
            ThemeTone::Light => PeakColors {
                primary: Color::BLACK,
                on_primary: Color::WHITE,
                primary_container: Color::from_rgb8(220, 220, 220),
                on_primary_container: Color::BLACK,

                secondary: Color::from_rgb8(60, 60, 60),
                on_secondary: Color::WHITE,
                secondary_container: Color::from_rgb8(200, 200, 200),
                on_secondary_container: Color::BLACK,

                accent: Color::from_rgb8(0, 0, 200),
                on_accent: Color::WHITE,

                success: Color::from_rgb8(0, 150, 0),
                warning: Color::from_rgb8(200, 150, 0),
                danger: Color::from_rgb8(200, 0, 0),
                info: Color::from_rgb8(0, 0, 200),

                surface: Color::WHITE,
                on_surface: Color::BLACK,
                surface_variant: Color::from_rgb8(240, 240, 240),
                on_surface_variant: Color::BLACK,

                background: Color::WHITE,
                on_background: Color::BLACK,

                border: Color::BLACK,
                divider: Color::BLACK,
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.5),

                text_primary: Color::BLACK,
                text_secondary: Color::from_rgb8(60, 60, 60),
                text_tertiary: Color::from_rgb8(100, 100, 100),
                text_disabled: Color::from_rgb8(150, 150, 150),
            },
            ThemeTone::Dark => PeakColors {
                primary: Color::WHITE,
                on_primary: Color::BLACK,
                primary_container: Color::from_rgb8(60, 60, 60),
                on_primary_container: Color::WHITE,

                secondary: Color::from_rgb8(200, 200, 200),
                on_secondary: Color::BLACK,
                secondary_container: Color::from_rgb8(80, 80, 80),
                on_secondary_container: Color::WHITE,

                accent: Color::from_rgb8(100, 200, 255),
                on_accent: Color::BLACK,

                success: Color::from_rgb8(100, 255, 100),
                warning: Color::from_rgb8(255, 220, 100),
                danger: Color::from_rgb8(255, 100, 100),
                info: Color::from_rgb8(100, 200, 255),

                surface: Color::BLACK,
                on_surface: Color::WHITE,
                surface_variant: Color::from_rgb8(30, 30, 30),
                on_surface_variant: Color::WHITE,

                background: Color::BLACK,
                on_background: Color::WHITE,

                border: Color::WHITE,
                divider: Color::WHITE,
                overlay: Color::from_rgba(0.0, 0.0, 0.0, 0.7),

                text_primary: Color::WHITE,
                text_secondary: Color::from_rgb8(200, 200, 200),
                text_tertiary: Color::from_rgb8(150, 150, 150),
                text_disabled: Color::from_rgb8(100, 100, 100),
            },
        }
    }
}
