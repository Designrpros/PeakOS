#![cfg(target_os = "linux")]

use crate::app::{Message, PeakNative};
use iced_layershell::reexports::{Anchor, KeyboardInteractivity, Layer};
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::Application;

pub const ID_DOCK: &str = "dock";

pub struct PeakLayerShell {
    pub native: PeakNative,
}

impl Application for PeakLayerShell {
    type Message = Message;
    type Flags = String;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (native, command) = PeakNative::new(flags);

        // We start with the Menubar, and we'll open the Dock as a separate layer soon.
        // For now, let's assume the first window is the Menubar.
        (Self { native }, command)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        self.native.update(message)
    }

    fn view(&self, id: iced_layershell::reexports::Id) -> iced::Element<Self::Message> {
        let tokens = &self.native.tokens;

        // Match the surface ID to the specific shell component
        // This is a placeholder logic until we have reliable ID management
        if id.to_string().contains("menubar") {
            peak_shell::menubar::view(tokens).map(Message::MenubarAction)
        } else if id.to_string().contains("dock") {
            let dock_element = peak_shell::dock::view(
                &self.native.pinned_apps,
                &self.native.running_apps,
                &self.native.registry.app_icons,
                tokens,
            )
            .map(Message::DockInteraction);

            iced::widget::container(dock_element)
                .width(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .into()
        } else {
            self.native.view_desktop()
        }
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

pub fn get_dock_settings() -> LayerShellSettings {
    LayerShellSettings {
        anchor: Anchor::BOTTOM,
        layer: Layer::BOTTOM,
        exclusive_zone: 80,
        keyboard_interactivity: KeyboardInteractivity::None,
        ..Default::default()
    }
}
