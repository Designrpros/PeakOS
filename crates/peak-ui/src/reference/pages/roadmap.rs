use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(_context: &Context, is_mobile: bool) -> PageResult {
    VStack::new_generic()
        .width(Length::Fill)
        .spacing(32.0)
        .padding(Padding {
            top: 96.0,
            right: if is_mobile { 20.0 } else { 64.0 },
            bottom: 120.0,
            left: if is_mobile { 20.0 } else { 64.0 },
        })
        .push(MarkdownView::new(include_str!(
            "../../../docs/roadmap/framework_roadmap.md"
        )))
        .sidebar_toggle(Message::ToggleSidebar)
}
