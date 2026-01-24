use crate::modifiers::Intent;
// Force rebuild to pick up peak-icons changes
use iced::{Alignment, Color, Length, Padding, Renderer, Shadow, Size, Theme, Vector};
pub use peak_core::registry::ShellMode;
pub use peak_theme::ThemeTokens;

#[derive(Clone, Debug)]
pub struct Context {
    pub theme: ThemeTokens,
    pub mode: ShellMode,
    pub device: DeviceType,
    pub size: Size,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceType {
    Desktop,
    Mobile,
    TV,
}

impl Context {
    pub fn new(mode: ShellMode, theme: ThemeTokens, size: Size) -> Self {
        let device = match mode {
            ShellMode::Desktop => DeviceType::Desktop,
            ShellMode::Mobile => DeviceType::Mobile,
            ShellMode::TV | ShellMode::Console | ShellMode::Fireplace => DeviceType::TV,
            _ => DeviceType::Desktop,
        };

        Self {
            theme,
            mode,
            device,
            size,
        }
    }

    pub fn is_slim(&self) -> bool {
        self.size.width < 600.0
    }

    pub fn is_wide(&self) -> bool {
        self.size.width > 1200.0
    }

    pub fn shadow(&self, color: Color, offset: impl Into<Vector>, blur_radius: f32) -> Shadow {
        if cfg!(target_arch = "wasm32") {
            Shadow::default()
        } else {
            Shadow {
                color,
                offset: offset.into(),
                blur_radius,
            }
        }
    }

    pub fn radius(&self, radius: f32) -> iced::border::Radius {
        if cfg!(target_arch = "wasm32") {
            0.0.into()
        } else {
            radius.into()
        }
    }
}

/// A Backend defines the output type and composition logic for a View.
pub trait Backend: Sized + Clone + 'static {
    type AnyView<Message: 'static>: 'static;

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        scale: f32,
    ) -> Self::AnyView<Message>;

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_y: Alignment,
        scale: f32,
    ) -> Self::AnyView<Message>;

    fn text<Message: 'static>(
        content: String,
        size: f32,
        color: Option<Color>,
        is_bold: bool,
        is_dim: bool,
        intent: Option<Intent>,
        font: Option<iced::Font>,
        alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn icon<Message: 'static>(
        name: String,
        size: f32,
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn divider<Message: 'static>(context: &Context) -> Self::AnyView<Message>;

    fn space<Message: 'static>(width: Length, height: Length) -> Self::AnyView<Message>;

    fn rectangle<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
        radius: f32,
        border_width: f32,
        border_color: Option<Color>,
    ) -> Self::AnyView<Message>;

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_press: Option<Message>,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn sidebar_item<Message: 'static>(
        title: String,
        icon: String,
        is_selected: bool,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        on_change: impl Fn(String) -> Message + 'static,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn slider<Message: Clone + 'static>(
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        on_change: impl Fn(f32) -> Message + 'static,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        on_toggle: impl Fn(bool) -> Message + 'static,
        context: &Context,
    ) -> Self::AnyView<Message>;

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        width: Length,
        height: Length,
    ) -> Self::AnyView<Message>;

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        columns: usize,
        spacing: f32,
    ) -> Self::AnyView<Message>;

    fn image<Message: 'static>(
        path: std::path::PathBuf,
        width: Length,
        height: Length,
        radius: f32,
    ) -> Self::AnyView<Message>;
}

/// The default Iced-based GUI backend.
#[derive(Clone, Copy, Debug, Default)]
pub struct IcedBackend;

impl Backend for IcedBackend {
    type AnyView<Message: 'static> = iced::Element<'static, Message, Theme, Renderer>;

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_x: Alignment,
        scale: f32,
    ) -> Self::AnyView<Message> {
        use iced::widget::{column, container};
        container(column(children).spacing(spacing * scale).align_x(align_x))
            .padding(Padding {
                top: padding.top * scale,
                right: padding.right * scale,
                bottom: padding.bottom * scale,
                left: padding.left * scale,
            })
            .width(width)
            .height(height)
            .into()
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        align_y: Alignment,
        scale: f32,
    ) -> Self::AnyView<Message> {
        use iced::widget::{container, row};
        container(row(children).spacing(spacing * scale).align_y(align_y))
            .padding(Padding {
                top: padding.top * scale,
                right: padding.right * scale,
                bottom: padding.bottom * scale,
                left: padding.left * scale,
            })
            .width(width)
            .height(height)
            .into()
    }

    fn text<Message: 'static>(
        content: String,
        size: f32,
        color: Option<Color>,
        is_bold: bool,
        is_dim: bool,
        intent: Option<Intent>,
        font: Option<iced::Font>,
        alignment: Alignment,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::text;
        let color = color.unwrap_or_else(|| {
            if let Some(i) = intent {
                match i {
                    Intent::Primary => context.theme.colors.primary,
                    Intent::Secondary => context.theme.colors.secondary,
                    Intent::Success => context.theme.colors.success,
                    Intent::Warning => context.theme.colors.warning,
                    Intent::Danger => context.theme.colors.danger,
                    Intent::Info => context.theme.colors.info,
                    Intent::Neutral => context.theme.colors.text_primary,
                }
            } else if is_dim {
                context.theme.colors.text_secondary
            } else {
                context.theme.colors.text_primary
            }
        });

        let mut font = font.unwrap_or_default();
        if is_bold {
            font.weight = iced::font::Weight::Bold;
        }

        text(content)
            .size(size * context.theme.scaling)
            .color(color)
            .font(font)
            .align_x(alignment)
            .into()
    }

    fn icon<Message: 'static>(
        name: String,
        size: f32,
        color: Option<Color>,
        context: &Context,
    ) -> Self::AnyView<Message> {
        let theme = context.theme;
        let final_color = color.unwrap_or(theme.colors.text_primary);

        let hex_color = format!(
            "#{:02X}{:02X}{:02X}",
            (final_color.r * 255.0) as u8,
            (final_color.g * 255.0) as u8,
            (final_color.b * 255.0) as u8
        );

        // 1. Try embedded icons first
        if let Some(svg_data) = peak_icons::get_icon(&name) {
            #[cfg(target_arch = "wasm32")]
            log::debug!("Icon '{}' found in EMBEDDED storage.", name);

            // AGGRESSIVE RECOLORING: Replace various black definitions with theme color
            let colored_svg = svg_data
                .replace("currentColor", &hex_color)
                .replace("fill=\"#000000\"", &format!("fill=\"{}\"", hex_color))
                .replace("fill=\"black\"", &format!("fill=\"{}\"", hex_color));

            return iced::widget::svg(iced::widget::svg::Handle::from_memory(
                colored_svg.into_bytes(),
            ))
            .width(size)
            .height(size)
            .into();
        }

        // 2. WASM FIX: Smart mapping to existing assets
        #[cfg(target_arch = "wasm32")]
        {
            // Determine if we need white or black icons based on brightness
            let luminance =
                0.2126 * final_color.r + 0.7152 * final_color.g + 0.0722 * final_color.b;
            let folder = if luminance > 0.5 { "white" } else { "black" };

            // Map abstract names to real files in your 'dist' folder
            let filename = match name.as_str() {
                // Sidebar mappings
                "sidebar" => "apps", // Maps 'sidebar' -> 'apps.svg'
                "book" => "library",
                "map" => "map",
                "users" => "smile",
                "layers" => "folder",
                "palette" => "palette",
                "maximize" => "settings",
                "type" => "document",
                "grid" => "apps",
                "monitor" => "cpu",
                "box" => "cube",

                // Toolbar / Common
                "search" => "search",
                "settings" => "settings",
                "wifi" => "wifi_full",
                "battery" => "battery",
                "terminal" => "terminal",
                "library" => "library",
                "store" => "store",
                _ => name.as_str(),
            };

            // Force path for showcase to ensure visibility
            let best_path = if [
                "search",
                "settings",
                "wifi_full",
                "battery",
                "library",
                "store",
                "terminal",
            ]
            .contains(&filename)
            {
                format!("assets/icons/menubar/{}/{}.svg", folder, filename)
            } else {
                format!("assets/icons/system/ui/{}.svg", filename)
            };

            return iced::widget::svg(iced::widget::svg::Handle::from_path(best_path))
                .width(size)
                .height(size)
                .into();
        }

        // 3. DESKTOP FALLBACK
        #[cfg(not(target_arch = "wasm32"))]
        {
            let handle = peak_core::icons::get_ui_icon(&name, &hex_color);
            iced::widget::svg(handle).width(size).height(size).into()
        }
    }

    fn divider<Message: 'static>(context: &Context) -> Self::AnyView<Message> {
        use iced::widget::{container, Rule};
        let divider_color = context.theme.colors.divider;
        container(Rule::horizontal(1))
            .style(move |_| container::Style {
                text_color: Some(divider_color),
                ..Default::default()
            })
            .into()
    }

    fn space<Message: 'static>(width: Length, height: Length) -> Self::AnyView<Message> {
        iced::widget::Space::new(width, height).into()
    }

    fn rectangle<Message: 'static>(
        width: Length,
        height: Length,
        color: Option<Color>,
        radius: f32,
        border_width: f32,
        border_color: Option<Color>,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;

        container(iced::widget::Space::new(Length::Fill, Length::Fill))
            .width(width)
            .height(height)
            .style({
                let b_color = border_color.unwrap_or(Color::TRANSPARENT);
                move |_| container::Style {
                    background: color.map(iced::Background::Color),
                    border: iced::Border {
                        color: b_color,
                        width: border_width,
                        radius: if cfg!(target_arch = "wasm32") {
                            0.0
                        } else {
                            radius
                        }
                        .into(),
                    },
                    ..Default::default()
                }
            })
            .into()
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        on_press: Option<Message>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        use iced::widget::button;
        button(content).on_press_maybe(on_press).into()
    }

    fn sidebar_item<Message: 'static>(
        title: String,
        icon: String,
        is_selected: bool,
        context: &Context,
    ) -> Self::AnyView<Message> {
        use crate::atoms::{Icon, Text};
        use crate::layout::HStack;
        use iced::widget::container;

        let theme = context.theme;
        let content = HStack::<Message, Self>::new_generic()
            .spacing(12.0)
            .padding(iced::Padding {
                top: 8.0,
                right: 12.0,
                bottom: 8.0,
                left: 12.0,
            })
            .align_y(iced::Alignment::Center)
            .push(Icon::<Self>::new(icon).size(18.0))
            .push(Text::<Self>::new(title).body().bold());

        if is_selected {
            container(content.view(context))
                .style({
                    let bg_color = theme.colors.primary;
                    let radius_val = if cfg!(target_arch = "wasm32") {
                        0.0
                    } else {
                        8.0
                    };
                    move |_| container::Style {
                        background: Some(bg_color.into()),
                        border: iced::Border {
                            radius: radius_val.into(),
                            ..Default::default()
                        },
                        text_color: Some(iced::Color::WHITE),
                        ..Default::default()
                    }
                })
                .width(Length::Fill)
                .into()
        } else {
            content.view(context)
        }
    }

    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        on_change: impl Fn(String) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        iced::widget::text_input(&placeholder, &value)
            .on_input(on_change)
            .padding(10)
            .into()
    }

    fn slider<Message: Clone + 'static>(
        range: std::ops::RangeInclusive<f32>,
        value: f32,
        on_change: impl Fn(f32) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        iced::widget::slider(range, value, on_change).into()
    }

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        on_toggle: impl Fn(bool) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        iced::widget::toggler(is_active)
            .label(label)
            .on_toggle(on_toggle)
            .into()
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        width: Length,
        height: Length,
    ) -> Self::AnyView<Message> {
        let s = iced::widget::stack(children).width(width).height(height);
        s.into()
    }

    fn grid<Message: 'static>(
        mut children: Vec<Self::AnyView<Message>>,
        columns: usize,
        spacing: f32,
    ) -> Self::AnyView<Message> {
        let mut rows = Vec::new();
        while !children.is_empty() {
            let chunk: Vec<_> = children
                .drain(0..std::cmp::min(columns, children.len()))
                .collect();
            rows.push(iced::widget::row(chunk).spacing(spacing).into());
        }
        iced::widget::column(rows).spacing(spacing).into()
    }

    fn image<Message: 'static>(
        path: std::path::PathBuf,
        width: Length,
        height: Length,
        radius: f32,
    ) -> Self::AnyView<Message> {
        use iced::widget::container;

        // WASM FIX: Ensure path starts with "assets/", but preserve subfolders!
        let final_path = if cfg!(target_arch = "wasm32") {
            let p_str = path.to_string_lossy();
            let result = if p_str.starts_with("/assets") || p_str.starts_with("assets") {
                path.clone()
            } else {
                std::path::PathBuf::from("assets").join(&path)
            };

            // --- DEBUGGING LOGS ---
            log::info!(
                "[IMAGE DEBUG] Orig: '{:?}' | Resolved: '{:?}'",
                path,
                result
            );
            // ---------------------
            result
        } else {
            path
        };

        let img = iced::widget::image(final_path).width(width).height(height);

        container(img)
            .width(width)
            .height(height)
            .style({
                let radius_val = if cfg!(target_arch = "wasm32") {
                    0.0
                } else {
                    radius
                };
                move |_| container::Style {
                    border: iced::Border {
                        radius: radius_val.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .into()
    }
}

/// A Terminal-based TUI backend.
#[derive(Clone, Copy, Debug, Default)]
pub struct TermBackend;

impl Backend for TermBackend {
    type AnyView<Message: 'static> = String;

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _scale: f32,
    ) -> Self::AnyView<Message> {
        children.join("\n")
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_y: Alignment,
        _scale: f32,
    ) -> Self::AnyView<Message> {
        children.join(" ")
    }

    fn text<Message: 'static>(
        content: String,
        _size: f32,
        _color: Option<Color>,
        is_bold: bool,
        is_dim: bool,
        intent: Option<Intent>,
        _font: Option<iced::Font>,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let mut out = content;
        if is_bold {
            out = format!("\x1b[1m{}\x1b[0m", out);
        } else if is_dim {
            out = format!("\x1b[2m{}\x1b[0m", out);
        }

        if let Some(i) = intent {
            let code = match i {
                Intent::Primary => "34",
                Intent::Success => "32",
                Intent::Warning => "33",
                Intent::Danger => "31",
                Intent::Info => "36",
                _ => "0",
            };
            out = format!("\x1b[{}m{}\x1b[0m", code, out);
        }
        out
    }

    fn icon<Message: 'static>(
        name: String,
        _size: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        let symbol = match name.as_str() {
            "settings" => "⚙",
            "terminal" => "",
            "chevron_right" => "",
            _ => "○",
        };
        format!("\x1b[36m{}\x1b[0m", symbol)
    }

    fn divider<Message: 'static>(_context: &Context) -> Self::AnyView<Message> {
        "────────────────────".to_string()
    }

    fn space<Message: 'static>(_width: Length, _height: Length) -> Self::AnyView<Message> {
        " ".to_string()
    }

    fn rectangle<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _radius: f32,
        _border_width: f32,
        _border_color: Option<Color>,
    ) -> Self::AnyView<Message> {
        "█".to_string()
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_press: Option<Message>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("[ {} ]", content)
    }

    fn sidebar_item<Message: 'static>(
        title: String,
        _icon: String,
        is_selected: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        if is_selected {
            format!("\x1b[1;34m {}\x1b[0m", title)
        } else {
            format!("  {}", title)
        }
    }

    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        _on_change: impl Fn(String) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("[ {} ] (Placeholder: {})", value, placeholder)
    }

    fn slider<Message: Clone + 'static>(
        _range: std::ops::RangeInclusive<f32>,
        value: f32,
        _on_change: impl Fn(f32) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("[---X---] {:.2}", value)
    }

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        _on_toggle: impl Fn(bool) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        format!("{} [{}]", label, if is_active { "ON" } else { "OFF" })
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _width: Length,
        _height: Length,
    ) -> Self::AnyView<Message> {
        children.join("\n")
    }

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _columns: usize,
        _spacing: f32,
    ) -> Self::AnyView<Message> {
        children.join(" | ")
    }

    fn image<Message: 'static>(
        path: std::path::PathBuf,
        _width: Length,
        _height: Length,
        _radius: f32,
    ) -> Self::AnyView<Message> {
        format!("[IMG: {:?}]", path)
    }
}

/// A View describes *what* to render, given a Context.
pub trait View<Message: 'static, B: Backend = IcedBackend> {
    fn view(&self, context: &Context) -> B::AnyView<Message>;

    /// Generates a semantic description of the view for AI agents.
    fn describe(&self, _context: &Context) -> SemanticNode {
        // Default implementation for basic views
        SemanticNode {
            role: "view".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
        }
    }

    fn into_box(self) -> Box<dyn View<Message, B>>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for Box<dyn View<Message, B>> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        self.as_ref().view(context)
    }

    fn describe(&self, context: &Context) -> SemanticNode {
        self.as_ref().describe(context)
    }
}

/// A semantic representation of a UI component for AI agents.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SemanticNode {
    pub role: String,
    pub label: Option<String>,
    pub content: Option<String>,
    pub children: Vec<SemanticNode>,
}

/// An AI-focused backend that renders UIs into semantic data.
#[derive(Clone, Copy, Debug, Default)]
pub struct AIBackend;

impl Backend for AIBackend {
    type AnyView<Message: 'static> = SemanticNode;

    fn vstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_x: Alignment,
        _scale: f32,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "vstack".to_string(),
            label: None,
            content: None,
            children,
        }
    }

    fn hstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _spacing: f32,
        _padding: Padding,
        _width: Length,
        _height: Length,
        _align_y: Alignment,
        _scale: f32,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "hstack".to_string(),
            label: None,
            content: None,
            children,
        }
    }

    fn text<Message: 'static>(
        content: String,
        _size: f32,
        _color: Option<Color>,
        _is_bold: bool,
        _is_dim: bool,
        _intent: Option<Intent>,
        _font: Option<iced::Font>,
        _alignment: Alignment,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "text".to_string(),
            label: None,
            content: Some(content),
            children: Vec::new(),
        }
    }

    fn icon<Message: 'static>(
        name: String,
        _size: f32,
        _color: Option<Color>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "icon".to_string(),
            label: Some(name),
            content: None,
            children: Vec::new(),
        }
    }

    fn divider<Message: 'static>(_context: &Context) -> Self::AnyView<Message> {
        SemanticNode {
            role: "divider".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
        }
    }

    fn space<Message: 'static>(_width: Length, _height: Length) -> Self::AnyView<Message> {
        SemanticNode {
            role: "space".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
        }
    }

    fn rectangle<Message: 'static>(
        _width: Length,
        _height: Length,
        _color: Option<Color>,
        _radius: f32,
        _border_width: f32,
        _border_color: Option<Color>,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "rectangle".to_string(),
            label: None,
            content: None,
            children: Vec::new(),
        }
    }

    fn button<Message: Clone + 'static>(
        content: Self::AnyView<Message>,
        _on_press: Option<Message>,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "button".to_string(),
            label: None,
            content: None,
            children: vec![content],
        }
    }

    fn sidebar_item<Message: 'static>(
        title: String,
        icon: String,
        is_selected: bool,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "sidebar_item".to_string(),
            label: Some(title),
            content: Some(icon),
            children: vec![SemanticNode {
                role: "state".to_string(),
                label: Some("selected".to_string()),
                content: Some(is_selected.to_string()),
                children: Vec::new(),
            }],
        }
    }

    fn text_input<Message: Clone + 'static>(
        value: String,
        placeholder: String,
        _on_change: impl Fn(String) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "text_input".to_string(),
            label: Some(placeholder),
            content: Some(value),
            children: Vec::new(),
        }
    }

    fn slider<Message: Clone + 'static>(
        _range: std::ops::RangeInclusive<f32>,
        value: f32,
        _on_change: impl Fn(f32) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "slider".to_string(),
            label: None,
            content: Some(value.to_string()),
            children: Vec::new(),
        }
    }

    fn toggle<Message: Clone + 'static>(
        label: String,
        is_active: bool,
        _on_toggle: impl Fn(bool) -> Message + 'static,
        _context: &Context,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "toggle".to_string(),
            label: Some(label),
            content: Some(is_active.to_string()),
            children: Vec::new(),
        }
    }

    fn zstack<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        _width: Length,
        _height: Length,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "zstack".to_string(),
            label: None,
            content: None,
            children,
        }
    }

    fn grid<Message: 'static>(
        children: Vec<Self::AnyView<Message>>,
        columns: usize,
        _spacing: f32,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "grid".to_string(),
            label: Some(format!("columns: {}", columns)),
            content: None,
            children,
        }
    }

    fn image<Message: 'static>(
        path: std::path::PathBuf,
        _width: Length,
        _height: Length,
        _radius: f32,
    ) -> Self::AnyView<Message> {
        SemanticNode {
            role: "image".to_string(),
            label: Some(path.to_string_lossy().into_owned()),
            content: None,
            children: Vec::new(),
        }
    }
}

/// A responsive helper.
pub fn responsive<Message>(
    mode: ShellMode,
    theme: peak_theme::ThemeTokens,
    f: impl Fn(Context) -> iced::Element<'static, Message, Theme, Renderer> + 'static,
) -> iced::Element<'static, Message, Theme, Renderer>
where
    Message: 'static,
{
    iced::widget::responsive(move |size| {
        let context = Context::new(mode, theme, size);
        f(context)
    })
    .into()
}

pub struct ProxyView<Message: 'static, B: Backend = IcedBackend> {
    view_fn: Box<dyn Fn(&Context) -> B::AnyView<Message>>,
}

impl<Message: 'static, B: Backend> ProxyView<Message, B> {
    pub fn new<F>(view_fn: F) -> Self
    where
        F: Fn(&Context) -> B::AnyView<Message> + 'static,
    {
        Self {
            view_fn: Box::new(view_fn),
        }
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for ProxyView<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        (self.view_fn)(context)
    }
}
