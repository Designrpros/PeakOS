#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Home,
    Library,
    Cortex,
    Settings,
    Poolside,
}

pub mod cortex;
pub mod settings;
