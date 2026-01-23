pub mod alert;
pub mod navigation;
pub mod reference;
pub mod segmented_picker;
pub mod toolbar;
pub mod window_chrome;

pub mod atoms;
pub mod catalog;
pub mod console;
pub mod containers;
pub mod controls;
pub mod core;
pub mod forms;
pub mod gestures;
pub mod inputs;
pub mod layout;
pub mod modifiers;
pub mod motion;
pub mod nav_split_view;
pub mod scroll_view;

pub mod prelude {
    pub use crate::atoms::{Divider, Icon, Image, Rectangle, Space, Text};
    pub use crate::catalog::{Catalog, CatalogItem, CatalogMessage};
    pub use crate::console::Console;
    pub use crate::containers::{Card, GlassCard, Section};
    pub use crate::controls::{Button, ButtonStyle, Slider, Stepper, Toggle};
    pub use crate::core::{
        responsive, AIBackend, Context, DeviceType, IcedBackend, ProxyView, SemanticNode,
        ShellMode, TermBackend, ThemeTokens, View,
    };
    pub use crate::forms::{Form, FormStyle};
    pub use crate::gestures::{Gesture, GestureDetector};
    pub use crate::inputs::{TextField, TextInput};
    pub use crate::layout::{HStack, ResponsiveGrid, VStack, ZStack};
    pub use crate::modifiers::{ControlSize, Intent, Variant};
    pub use crate::motion::{AnimationConfig, Interpolatable, MotionState, Spring};
    pub use crate::nav_split_view::NavigationSplitView;
    pub use crate::navigation::Sidebar;
    pub use crate::scroll_view::ScrollView;
    pub use crate::segmented_picker::SegmentedPicker;
    pub use crate::toolbar::{ToolbarGroup, ToolbarItem};
    pub use peak_theme::ThemeTone;

    // Re-export core UI types so showcase doesn't need direct iced imports
    pub use iced::widget::{column, container, row, stack};
    pub use iced::{
        application, run, Alignment, Background, Border, Color, Element, Font, Length, Padding,
        Renderer, Result, Shadow, Size, Task, Theme, Vector,
    };
    pub mod font {
        pub use iced::font::*;
    }
}
