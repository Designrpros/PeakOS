use iced::{Element, Task};
use peak_core::app_traits::{PeakApp, ShellContext};
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
            peak_core::theme::Theme::Dark => peak_ui_theme::ThemeTone::Dark,
            peak_core::theme::Theme::Light => peak_ui_theme::ThemeTone::Light,
            _ => peak_ui_theme::ThemeTone::Dark,
        };

        let mode = peak_ui::core::ShellMode::Desktop;
        let tokens = peak_ui_theme::ThemeTokens::get(mode, tone);
        let catalog = self.catalog.clone();

        responsive(move |_device_type| {
            let catalog = catalog.clone();
            ProxyView::new(move |ctx| catalog.view(ctx).map(Message::Catalog)).into_box()
        })
        .view(&Context::new(
            mode,
            tokens,
            Size::new(1024.0, 768.0),
            Localization::default(),
        ))
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }
}
