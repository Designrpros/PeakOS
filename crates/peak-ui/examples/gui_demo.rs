use iced::Size;
use peak_core::registry::ShellMode;
use peak_theme::ThemeTokens;
use peak_ui::core::{Context, IcedBackend};
use peak_ui::prelude::*;

fn main() {
    let tokens = ThemeTokens::get(ShellMode::Desktop, peak_theme::ThemeTone::Dark);
    let _context = Context::new(ShellMode::Desktop, tokens, Size::new(800.0, 600.0));

    // Define a UI using the SAME components as the TUI app
    let _ui: VStack<(), IcedBackend> = VStack::new()
        .spacing(10.0)
        .push(Text::new("PeakOS Desktop Link").title1())
        .push(Divider::new())
        .push(
            HStack::new()
                .spacing(10.0)
                .push(Icon::new("settings"))
                .push(Text::new("Settings")),
        )
        .push(
            HStack::new()
                .spacing(10.0)
                .push(Icon::new("terminal"))
                .push(Text::new("Terminal")),
        );

    // This would render in a real GUI window
    println!("GUI UI structure created successfully!");
    println!("Components: VStack with 2 HStack children");
    println!("Same semantic structure as TUI version!");
}
