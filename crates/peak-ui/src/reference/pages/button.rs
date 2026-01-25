use super::super::views::ComponentDoc;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use std::sync::Arc;

pub fn view(_context: &Context) -> PageResult {
    PageResult::new(
        ComponentDoc::new(
            "Button",
            "A versatile button component with support for multiple variants, icons, and reactive states.",
            r#"
Button::label("Primary Button")
    .variant(Variant::Solid)
    .on_press(Message::DoSomething)

Button::new(
    HStack::new()
        .push(Icon::new("plus"))
        .push(Text::new("Add Item"))
)
.variant(Variant::Soft)
"#,
            Arc::new(VStack::<Message, IcedBackend>::new_generic()
                .spacing(16.0)
                .push(
                    HStack::<Message, IcedBackend>::new_generic()
                        .spacing(12.0)
                        .push(Button::<Message>::label("Solid").variant(Variant::Solid))
                        .push(Button::<Message>::label("Soft").variant(Variant::Soft))
                        .push(Button::<Message>::label("Outline").variant(Variant::Outline))
                        .push(Button::<Message>::label("Ghost").variant(Variant::Ghost))
                )
                .push(
                    Button::<Message>::new(
                        HStack::<Message, IcedBackend>::new_generic()
                            .spacing(8.0)
                            .push(Icon::<IcedBackend>::new("download").size(14.0))
                            .push(Text::<IcedBackend>::new("Download with Icon").caption1())
                    )
                    .variant(Variant::Solid)
                ))
        )
    )
}
