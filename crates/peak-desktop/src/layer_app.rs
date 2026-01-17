#![cfg(target_os = "linux")]

use crate::app::{Message, PeakNative};
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer, LayershellCustomActions};
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::Application;

pub const ID_DOCK: &str = "dock";

#[derive(Debug, Clone)]
pub enum LayerMessage {
    App(Message),
    Shell(LayershellCustomActions),
}

impl TryInto<LayershellCustomActions> for LayerMessage {
    type Error = Self;
    fn try_into(self) -> Result<LayershellCustomActions, Self> {
        match self {
            LayerMessage::Shell(action) => Ok(action),
            _ => Err(self),
        }
    }
}

pub struct PeakLayerShell {
    pub native: PeakNative,
}

impl Application for PeakLayerShell {
    type Message = LayerMessage;
    type Flags = String;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (native, command) = PeakNative::new(flags);
        (Self { native }, command.map(LayerMessage::App))
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            LayerMessage::App(msg) => self.native.update(msg).map(LayerMessage::App),
            LayerMessage::Shell(_) => iced::Command::none(),
        }
    }

    fn namespace(&self) -> String {
        "PeakOS".to_string()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        // Multi-surface support not implemented yet
        self.native.view_desktop().map(LayerMessage::App)
    }

    fn title(&self) -> String {
        self.native.title()
    }

    fn theme(&self) -> Self::Theme {
        self.native.theme()
    }
}

pub fn get_menubar_settings() -> LayerShellSettings {
    LayerShellSettings {
        anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
        layer: Layer::TOP,
        exclusive_zone: 40,
        keyboard_interactivity: KeyboardInteractivity::None,
        ..Default::default()
    }
}

pub fn get_menubar_settings() -> LayerShellSettings {
    LayerShellSettings {
        anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
        layer: Layer::TOP,
        exclusive_zone: 40,
        keyboard_interactivity: KeyboardInteractivity::None,
        ..Default::default()
    }
}

pub fn get_dock_settings() -> LayerShellSettings {
    LayerShellSettings {
        anchor: Anchor::BOTTOM,
        layer: Layer::BOTTOM,
        exclusive_zone: 80,
        keyboard_interactivity: KeyboardInteractivity::None,
        ..Default::default()
    }
}
