use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(_context: &Context, is_mobile: bool) -> PageResult {
    VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .padding(Padding {
            top: 96.0,
            right: if is_mobile { 20.0 } else { 64.0 },
            bottom: 120.0,
            left: if is_mobile { 20.0 } else { 64.0 },
        })
        .push(Text::<IcedBackend>::new("Components").large_title().bold())
        .push(
            Text::<IcedBackend>::new("Explore the building blocks of PeakUI.")
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
                        .push(Text::<IcedBackend>::new("Buttons").title2().bold())
                        .push(
                            HStack::<Message, IcedBackend>::new_generic()
                                .spacing(12.0)
                                .push(Button::<Message>::label("Primary"))
                                .push(
                                    Button::<Message>::label("Secondary")
                                        .variant(crate::modifiers::Variant::Soft),
                                )
                                .push(
                                    Button::<Message>::label("Ghost")
                                        .variant(crate::modifiers::Variant::Ghost),
                                ),
                        ),
                )
                .push(
                    VStack::<Message, IcedBackend>::new_generic()
                        .spacing(12.0)
                        .push(Text::<IcedBackend>::new("Inputs").title2().bold())
                        .push(
                            VStack::<Message, IcedBackend>::new_generic()
                                .spacing(24.0)
                                .push(Toggle::<Message>::new("Toggle Switch", true, |_| {
                                    Message::ToggleSearch
                                }))
                                .push(Slider::<Message>::new(0.0..=100.0, 50.0, |_| {
                                    Message::ToggleSearch
                                }))
                                .push(TextField::<Message>::new("", "Search...", |_| {
                                    Message::ToggleSearch
                                })),
                        ),
                ),
        )
        .sidebar_toggle(Message::ToggleSidebar)
}
