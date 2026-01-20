#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Solid, // Full background color
    Soft,    // Light background, dark text
    Outline, // Border only
    Ghost,   // No background until hover
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Intent {
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
    Neutral,
}
