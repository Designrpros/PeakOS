use super::super::app::Message;
use super::super::pages;
use crate::prelude::*;

pub struct CanvasView {
    pub active_tab: String,
    pub navigation_mode: String,
}

use super::super::page::PageResult;

impl CanvasView {
    pub fn new(active_tab: String, navigation_mode: String) -> Self {
        Self {
            active_tab,
            navigation_mode,
        }
    }

    pub fn render_page(&self, context: &Context) -> PageResult {
        let is_mobile = context.size.width < 900.0;

        match self.active_tab.as_str() {
            "Introduction" | "Roadmap" | "Community" | "Overview" | "API Schema" => {
                pages::introduction::view(context, is_mobile)
            }
            "Customizations" => pages::customizations::view(context, is_mobile),
            "Basic Sizing" => pages::sizing::view(context, is_mobile),
            "Typography" => pages::typography::view(context, is_mobile),
            "Layout" => pages::layout::view(context, is_mobile),

            // Components Gallery
            "Buttons" | "Inputs" | "Toggles" | "Sliders" | "Pickers" => {
                pages::components::view(context, is_mobile)
            }

            // Hooks Gallery
            "use_state" | "use_effect" | "use_memo" | "use_callback" => {
                pages::hooks::view(context, is_mobile)
            }

            // Settings Gallery
            "Appearance" | "Scaling" | "Shortcuts" | "About" | "Updates" => {
                pages::settings::view(context, is_mobile)
            }

            tab => pages::component_detail::view(tab, context, is_mobile),
        }
    }
}

impl View<Message, IcedBackend> for CanvasView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let page = self.render_page(context);

        container(ScrollView::from_boxed(page.view).view(context))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
