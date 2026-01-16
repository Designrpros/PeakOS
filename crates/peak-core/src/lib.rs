pub mod app_traits;
pub mod icons;
pub mod integrations;
pub mod models;
pub mod registry;
pub mod styles;
pub mod systems;
pub mod theme;
pub mod utils;

// Re-export key types for convenience
pub use registry::{AppId, AppInfo, ShellMode};
pub use theme::Theme;
