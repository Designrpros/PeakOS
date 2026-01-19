// Redmond Shell - Windows 10-style desktop
// Bottom taskbar with left-aligned Start menu

// pub mod start_menu;
pub mod taskbar;

// pub use start_menu::{view as start_menu_view, StartMenuMessage};
pub use taskbar::{view as taskbar_view, TaskbarMessage};
