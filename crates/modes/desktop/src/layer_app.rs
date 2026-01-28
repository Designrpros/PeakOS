#![cfg(target_os = "linux")]

use crate::app::{Message, PeakNative};
use iced_layershell::actions::LayershellCustomActionWithId;
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer};
use iced_layershell::settings::LayerShellSettings;

pub const ID_DOCK: &str = "dock";

#[derive(Debug, Clone)]
pub enum LayerMessage {
    App(Message),
    Shell(LayershellCustomActionWithId),
}

impl TryInto<LayershellCustomActionWithId> for LayerMessage {
    type Error = Self;
    fn try_into(self) -> Result<LayershellCustomActionWithId, Self> {
        match self {
            LayerMessage::Shell(action) => Ok(action),
            _ => Err(self),
        }
    }
}

pub struct PeakLayerShell {
    pub native: PeakNative,
}

impl PeakLayerShell {
    pub fn new(flags: crate::app::PeakNativeFlags) -> (Self, iced::Task<LayerMessage>) {
        let (native, command) = PeakNative::new(flags);
        (Self { native }, command.map(LayerMessage::App))
    }

    pub fn update(&mut self, message: LayerMessage) -> iced::Task<LayerMessage> {
        match message {
            LayerMessage::App(msg) => self.native.update(msg).map(LayerMessage::App),
            LayerMessage::Shell(_) => iced::Task::none(),
        }
    }

    pub fn view(&self) -> iced::Element<'_, LayerMessage> {
        self.native.view().map(LayerMessage::App)
    }

    pub fn theme(&self) -> iced::Theme {
        self.native.theme()
    }

    pub fn subscription(&self) -> iced::Subscription<LayerMessage> {
        self.native.subscription().map(LayerMessage::App)
    }

    pub fn title(_state: &PeakLayerShell) -> String {
        "PeakOS".to_string()
    }
}

pub fn get_menubar_settings() -> LayerShellSettings {
    LayerShellSettings {
        anchor: Anchor::Top | Anchor::Left | Anchor::Right,
        layer: Layer::Top,
        exclusive_zone: 40,
        keyboard_interactivity: KeyboardInteractivity::None,
        ..Default::default()
    }
}

pub fn get_dock_settings() -> LayerShellSettings {
    LayerShellSettings {
        anchor: Anchor::Bottom,
        layer: Layer::Overlay,
        exclusive_zone: 60,
        keyboard_interactivity: KeyboardInteractivity::None,
        ..Default::default()
    }
}

pub fn get_desktop_settings() -> LayerShellSettings {
    LayerShellSettings {
        anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
        layer: Layer::Bottom,
        exclusive_zone: -1,
        keyboard_interactivity: KeyboardInteractivity::OnDemand, // Needed for search/inputs
        ..Default::default()
    }
}
