use super::super::app::Message;
use super::super::pages;
use crate::prelude::*;

pub struct CanvasView {
    pub active_tab: String,
    pub navigation_mode: String,
}

impl CanvasView {
    pub fn new(active_tab: String, navigation_mode: String) -> Self {
        Self {
            active_tab,
            navigation_mode,
        }
    }
}

impl View<Message, IcedBackend> for CanvasView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let is_mobile = context.size.width < 900.0;

        let content: Box<dyn View<Message, IcedBackend>> = if self.navigation_mode == "API" {
            pages::api_schema::view(context, is_mobile)
        } else {
            match self.active_tab.as_str() {
                "Introduction" => pages::introduction::view(context, is_mobile),
                "Roadmap" => pages::roadmap::view(context, is_mobile),
                "Community" => pages::community::view(context, is_mobile),
                "Overview" => pages::overview::view(context, is_mobile),
                "Customizations" => pages::customizations::view(context, is_mobile),
                "Basic Sizing" => pages::sizing::view(context, is_mobile),
                "Typography" => pages::typography::view(context, is_mobile),
                "Layout" => pages::layout::view(context, is_mobile),
                "Docks" => pages::docks::view(context, is_mobile),
                "API Schema" => pages::api_schema::view(context, is_mobile),
                tab => pages::component_detail::view(tab, context, is_mobile),
            }
        };

        container(ScrollView::from_boxed(content).view(context))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
