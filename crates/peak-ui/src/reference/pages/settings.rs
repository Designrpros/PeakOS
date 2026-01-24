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
            .push(Text::<IcedBackend>::new("Settings").large_title().bold())
            .push(
                Text::<IcedBackend>::new(
                    "Demonstrating dynamic theme switching and user preferences.",
                )
                .title3()
                .secondary(),
            )
            .push(Divider::<IcedBackend>::new())
            .push(
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(16.0)
                    .push(Text::<IcedBackend>::new("Appearance").title2().bold())
                    .push(
                        HStack::<Message, IcedBackend>::new_generic()
                            .spacing(12.0)
                            .push(Button::<Message>::label("Light Mode"))
                            .push(Button::<Message>::label("Dark Mode"))
                            .push(Button::<Message>::label("System")),
                    ),
            )
            .push(
                VStack::<Message, IcedBackend>::new_generic()
                    .spacing(16.0)
                    .push(Text::<IcedBackend>::new("Scaling").title2().bold())
                    .push(
                        Text::<IcedBackend>::new(
                            "Adjust the UI scaling for different screen densities.",
                        )
                        .secondary(),
                    ),
            ),
    )
}
