use iced::Element;
use peak_core::registry::ShellMode;
use peak_theme::{ThemeTokens, ThemeTone};
use peak_ui::prelude::*;

pub fn main() -> iced::Result {
    iced::run("PeakUI Hello World", update, view)
}

fn update(_state: &mut (), _message: ()) {
    // No-op
}

fn view(_state: &()) -> Element<'_, ()> {
    responsive(
        ShellMode::Desktop,
        ThemeTokens::get(ShellMode::Desktop, ThemeTone::Light),
        |context| {
            VStack::new()
                .spacing(20.0)
                .padding(40.0)
                .push(Text::new("Hello from PeakUI").large_title())
                .push(Text::new("This is a declarative UI experiment.").title1())
                .push(
                    HStack::new()
                        .spacing(10.0)
                        .push(Text::new("Cross-platform").secondary())
                        .push(Text::new("•").secondary())
                        .push(Text::new("Adaptive").secondary())
                        .push(Text::new("•").secondary())
                        .push(Text::new("Rust-native").secondary()),
                )
                .view(&context)
        },
    )
}
