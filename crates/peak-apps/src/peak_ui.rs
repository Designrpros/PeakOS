use iced::{Element, Task};
use peak_core::app_traits::{PeakApp, ShellContext};
use peak_core::registry::ShellMode;
use peak_theme::{ThemeTokens, ThemeTone};
use peak_ui::prelude::*;

#[derive(Debug, Clone)]
pub enum Message {
    Catalog(CatalogMessage),
}

pub struct PeakUIApp {
    catalog: Catalog,
}

impl PeakUIApp {
    pub fn new() -> Self {
        Self {
            catalog: Catalog::new(),
        }
    }
}

impl Default for PeakUIApp {
    fn default() -> Self {
        Self::new()
    }
}

impl PeakApp for PeakUIApp {
    type Message = Message;

    fn title(&self) -> String {
        "PeakUI".into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        match message {
            Message::Catalog(msg) => self.catalog.update(msg).map(Message::Catalog),
        }
    }

    fn view(&self, theme: &peak_core::theme::Theme) -> Element<'_, Self::Message> {
        let tone = match theme {
            peak_core::theme::Theme::Dark => ThemeTone::Dark,
            peak_core::theme::Theme::Light => ThemeTone::Light,
            _ => ThemeTone::Dark,
        };

        let mode = ShellMode::Desktop;
        let tokens = ThemeTokens::get(mode, tone);
        let catalog = self.catalog.clone();

        responsive(mode, tokens, move |context| {
            // Render the shared Catalog View
            catalog.view(&context).map(Message::Catalog)
        })
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }
}
