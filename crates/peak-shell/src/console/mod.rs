// Console Shell - PS5-style gaming interface
// Large game tiles + category filtering

pub mod category_bar;
pub mod game_rail;

pub use category_bar::{view as category_bar_view, CategoryBarMessage};
pub use game_rail::{view as game_rail_view, GameRailMessage};
