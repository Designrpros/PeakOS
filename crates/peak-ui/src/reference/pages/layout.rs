use super::super::page::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;

pub fn view(_context: &Context, is_mobile: bool) -> PageResult {
    VStack::new_generic()
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
                .push(
                    HStack::new_generic()
                        .spacing(8.0)
                        .push(Text::<IcedBackend>::new("Overview").caption2().secondary())
                        .push(Text::<IcedBackend>::new(">").caption2().secondary())
                        .push(
                            Text::<IcedBackend>::new("Customizations")
                                .caption2()
                                .secondary(),
                        )
                        .push(Text::<IcedBackend>::new(">").caption2().secondary())
                        .push(Text::<IcedBackend>::new("Layout").caption2().secondary()),
                )
                .push(Text::<IcedBackend>::new("Layout").large_title().bold())
                .push(
                    Text::<IcedBackend>::new(
                        "Customizations for setting the layout of an element.",
                    )
                    .title3()
                    .secondary(),
                ),
        )
        .push(
            ZStack::new_generic()
                .push(
                    Image::new("crates/peak-ui/assets/mesh_bg.png")
                        .width(Length::Fill)
                        .height(Length::Fixed(340.0))
                        .corner_radius(16.0),
                )
                .push(ProxyView::new(move |ctx| {
                    let theme = ctx.theme;
                    container(
                        VStack::new_generic()
                            .spacing(16.0)
                            .push(
                                HStack::new_generic()
                                    .spacing(6.0)
                                    .align_y(Alignment::Center)
                                    .push(
                                        Rectangle::<IcedBackend>::new(8.0.into(), 8.0.into())
                                            .color(Color::from_rgb(1.0, 0.4, 0.4))
                                            .corner_radius(4.0),
                                    )
                                    .push(
                                        Rectangle::<IcedBackend>::new(8.0.into(), 8.0.into())
                                            .color(Color::from_rgb(1.0, 0.8, 0.2))
                                            .corner_radius(4.0),
                                    )
                                    .push(
                                        Rectangle::<IcedBackend>::new(8.0.into(), 8.0.into())
                                            .color(Color::from_rgb(0.2, 0.8, 0.2))
                                            .corner_radius(4.0),
                                    )
                                    .push(Space::<IcedBackend>::new(Length::Fill, 0.0.into()))
                                    .push(
                                        Text::<IcedBackend>::new("flex-layout")
                                            .caption2()
                                            .bold()
                                            .secondary(),
                                    ),
                            )
                            .view(ctx),
                    )
                    .padding(24)
                    .width(if ctx.size.width < 500.0 {
                        Length::Fill
                    } else {
                        Length::Fixed(420.0)
                    })
                    .style({
                        let radius = ctx.radius(12.0);
                        move |_| container::Style {
                            background: Some(theme.colors.surface.into()),
                            border: Border {
                                radius,
                                color: theme.colors.border.scale_alpha(0.1),
                                width: 1.0,
                            },
                            ..Default::default()
                        }
                    })
                    .into()
                })),
        )
        .searchable("", "Search layout...", |_| Message::ToggleSearch)
}
