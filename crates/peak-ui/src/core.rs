use iced::{Element, Renderer, Size, Theme};
use peak_core::registry::ShellMode;
use peak_theme::ThemeTokens;

/// Experimental: Core View Trait for Declarative UI
///
/// This moves away from the "Widget" composition model to a "View" description model.

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
}

/// A View describes *what* to render, given a Context.
pub trait View<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer>;
}

impl<Message> View<Message> for Box<dyn View<Message>> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        self.as_ref().view(context)
    }
}

/// A responsive helper that automatically manages the UI context.
pub fn responsive<Message>(
    mode: ShellMode,
    theme: peak_theme::ThemeTokens,
    f: impl Fn(Context) -> Element<'static, Message, Theme, Renderer> + 'static,
) -> Element<'static, Message, Theme, Renderer>
where
    Message: 'static,
{
    iced::widget::responsive(move |size| {
        let context = Context::new(mode, theme, size);
        f(context)
    })
    .into()
}
