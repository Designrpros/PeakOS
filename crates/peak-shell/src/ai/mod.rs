// AI Shell - OS-style desktop with integrated AI search
// High floating top dock + integrated AI input

// pub mod ai_input;
// pub mod top_dock;

// pub use ai_input::{view as ai_input_view, AiInputMessage};
// pub use top_dock::{view as top_dock_view, TopDockMessage};

use iced::Element;
use peak_theme::ThemeTokens;

pub fn view<'a>(_tokens: ThemeTokens) -> Element<'a, String> {
    iced::widget::container(iced::widget::text("AI Shell Placeholder")).into()
}
