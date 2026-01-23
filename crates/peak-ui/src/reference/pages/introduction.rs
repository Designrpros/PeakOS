use super::super::app::Message;
use crate::prelude::*;

pub fn view(_context: &Context, is_mobile: bool) -> Box<dyn View<Message, IcedBackend>> {
    Box::new(
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(48.0)
            .padding(Padding {
                top: 96.0,
                right: if is_mobile { 20.0 } else { 64.0 },
                bottom: 120.0,
                left: if is_mobile { 20.0 } else { 64.0 },
            })
            .push(
                VStack::new_generic()
                    .spacing(12.0)
                    .push(Text::<IcedBackend>::new("GETTING STARTED").caption2().secondary().bold())
                    .push(Text::<IcedBackend>::new("Introduction").large_title().bold())
                    .push(Text::<IcedBackend>::new("Welcome to the PeakUI Reference application. Here you can explore all components, patterns, and best practices.").title3().secondary())
            )
    )
}
