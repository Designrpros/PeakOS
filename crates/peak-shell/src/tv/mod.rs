// TV Shell - Apple TV-style media interface
// Focus-based app rail + control center overlay

pub mod app_rail;
pub mod control_center;

pub use app_rail::{view as app_rail_view, AppRailMessage};
pub use control_center::{view as control_center_view, ControlCenterMessage};
