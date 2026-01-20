pub mod alert;
pub mod navigation;
pub mod segmented_picker;
pub mod window_chrome;

pub mod atoms;
pub mod catalog;
pub mod containers;
pub mod controls;
pub mod core;
pub mod forms;
pub mod inputs;
pub mod layout;
pub mod modifiers;
pub mod nav_split_view;
pub mod scroll_view;

pub mod prelude {
    pub use crate::atoms::{Divider, Icon, Image, Text};
    pub use crate::containers::{Card, Section};
    pub use crate::controls::{Button, ButtonStyle, Slider, Stepper, Toggle};
    pub use crate::core::{responsive, Context, DeviceType, View};
    pub use crate::forms::{Form, FormStyle};
    pub use crate::inputs::TextField;
    pub use crate::layout::{HStack, ResponsiveGrid, VStack, ZStack};
    pub use crate::modifiers::{Intent, Size, Variant};
    pub use crate::nav_split_view::NavigationSplitView;
    pub use crate::scroll_view::ScrollView;
}
