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

        // Expanded Tab List for "Different Purposes"
        ToolbarGroup::new()
            .padding(Padding::from([12, 16]))
            // 1. Guide: The Narrative Book
            .push(
                ToolbarItem::new()
                    .icon("book")
                    .active(nav_mode == "Guide")
                    .on_press(Message::SetNavigationMode("Guide".into())),
            )
            // 2. Documentation: API & Technical Docs
            .push(
                ToolbarItem::new()
                    .icon("file-text")
                    .active(nav_mode == "Documentation")
                    .on_press(Message::SetNavigationMode("Documentation".into())),
            )
            // 3. Components: Visual Gallery (NEW)
            .push(
                ToolbarItem::new()
                    .icon("grid") // Uses 'grid' or 'apps' icon
                    .active(nav_mode == "Components")
                    .on_press(Message::SetNavigationMode("Components".into())),
            )
            // 5. Hooks: State Management Examples
            .push(
                ToolbarItem::new()
                    .icon("zap")
                    .active(nav_mode == "Hooks")
                    .on_press(Message::SetNavigationMode("Hooks".into())),
            )
            // 6. Settings: Customization (NEW)
            .push(
                ToolbarItem::new()
                    .icon("settings")
                    .active(nav_mode == "Settings")
                    .on_press(Message::SetNavigationMode("Settings".into())),
            )
            .view(context)
    }
}
