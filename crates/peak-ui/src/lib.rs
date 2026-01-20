pub mod alert;
pub mod navigation;
pub mod segmented_picker;
pub mod window_chrome;

pub mod atoms;
pub mod containers;
pub mod controls;
pub mod core;
pub mod forms;
pub mod layout;
pub mod nav_split_view;
pub mod scroll_view;

pub mod prelude {
    pub use crate::atoms::Text;
    pub use crate::containers::{Card, Section};
    pub use crate::controls::{Button, ButtonStyle, Slider, Toggle};
    pub use crate::core::{responsive, Context, DeviceType, View};
    pub use crate::forms::{Form, FormStyle};
    pub use crate::layout::{HStack, ResponsiveGrid, VStack, ZStack};
    pub use crate::nav_split_view::NavigationSplitView;
    pub use crate::scroll_view::ScrollView;
}
