// Trait definitions for PeakOS apps
// This module defines the common interface that all built-in apps implement

use iced::{Color, Element, Subscription, Task};

/// Theme information passed to apps for consistent styling
#[derive(Debug, Clone, Copy)]
pub struct AppTheme {
    pub is_light: bool,
    pub text_color: Color,
    pub bg_color: Color,
    pub border_color: Color,
    #[allow(dead_code)]
    pub accent_color: Color,
}

impl AppTheme {
    /// Create a light theme
    pub fn light() -> Self {
        Self {
            is_light: true,
            text_color: Color::from_rgb8(35, 30, 30),
            bg_color: Color::from_rgb8(247, 245, 242),
            border_color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            accent_color: Color::from_rgb8(0, 122, 255),
        }
    }

    /// Create a dark theme
    pub fn dark() -> Self {
        Self {
            is_light: false,
            text_color: Color::from_rgb8(235, 230, 225),
            bg_color: Color::from_rgb8(15, 14, 14),
            border_color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            accent_color: Color::from_rgb8(10, 132, 255),
        }
    }
}

/// Core trait that all PeakOS apps implement
/// This provides a standardized interface for app lifecycle management
#[allow(dead_code)]
pub trait PeakApp {
    /// The message type this app handles
    type Message: Clone + std::fmt::Debug;

    /// Update the app state based on a message
    /// Returns a Task that may produce more messages
    fn update(&mut self, message: Self::Message) -> Task<Self::Message>;

    /// Render the app UI
    /// Takes theme information for consistent styling
    fn view(&self, theme: &AppTheme) -> Element<'_, Self::Message>;

    /// Subscribe to external events (timers, async operations, etc.)
    /// Default implementation returns no subscriptions
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}
