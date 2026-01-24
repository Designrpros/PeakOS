use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(_context: &Context, is_mobile: bool) -> PageResult {
    PageResult::new(
        VStack::<Message, IcedBackend>::new_generic()
            .spacing(24.0)
            .padding(Padding {
                top: 96.0,
                right: if is_mobile { 20.0 } else { 64.0 },
                bottom: 120.0,
                left: if is_mobile { 20.0 } else { 64.0 },
            })
            .push(
                Text::<IcedBackend>::new("Hooks & State")
                    .large_title()
                    .bold(),
            )
            .push(
                Text::<IcedBackend>::new("Examples of functional components and state management.")
                    .title3()
                    .secondary(),
            )
            .push(Divider::<IcedBackend>::new())
            .push(
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(32.0)
                    .push(
                        VStack::<Message, IcedBackend>::new_generic()
                            .spacing(12.0)
                            .push(Text::<IcedBackend>::new("Counter Example").title2().bold())
                            .push(
                                Text::<IcedBackend>::new(
                                    "This demonstrates using simple message-based state updates.",
                                )
                                .secondary(),
                            )
                            .push(
                                HStack::<Message, IcedBackend>::new_generic()
                                    .spacing(12.0)
                                    .push(Button::<Message>::label("-"))
                                    .push(Text::<IcedBackend>::new("0").title3().bold())
                                    .push(Button::<Message>::label("+")),
                            ),
                    ),
            ),
    )
}
