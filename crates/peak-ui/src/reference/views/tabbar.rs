use super::super::app::Message;
use crate::prelude::*;

pub struct TabBarView {
    pub navigation_mode: String,
}

impl TabBarView {
    pub fn new(navigation_mode: String) -> Self {
        Self { navigation_mode }
    }
}

impl View<Message, IcedBackend> for TabBarView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let nav_mode = &self.navigation_mode;

        ToolbarGroup::new()
            .padding(Padding::from([12, 16]))
            .push(
                ToolbarItem::new()
                    .icon("book")
                    .active(nav_mode == "Guide")
                    .on_press(Message::SetNavigationMode("Guide".into())),
            )
            .push(
                ToolbarItem::new()
                    .icon("file-text")
                    .active(nav_mode == "Documentation")
                    .on_press(Message::SetNavigationMode("Documentation".into())),
            )
            .push(
                ToolbarItem::new()
                    .icon("monitor")
                    .active(nav_mode == "Docks")
                    .on_press(Message::SetNavigationMode("Docks".into())),
            )
            .push(
                ToolbarItem::new()
                    .icon("zap")
                    .active(nav_mode == "Hooks")
                    .on_press(Message::SetNavigationMode("Hooks".into())),
            )
            .push(
                ToolbarItem::new()
                    .icon("code")
                    .active(nav_mode == "API")
                    .on_press(Message::SetNavigationMode("API".into())),
            )
            .view(context)
    }
}
