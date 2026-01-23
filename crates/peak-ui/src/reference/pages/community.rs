use super::super::app::Message;
use crate::prelude::*;

pub fn view(_context: &Context, is_mobile: bool) -> Box<dyn View<Message, IcedBackend>> {
    Box::new(
        VStack::new_generic()
            .spacing(24.0)
            .padding(Padding {
                top: 96.0,
                right: if is_mobile { 20.0 } else { 64.0 },
                bottom: 120.0,
                left: if is_mobile { 20.0 } else { 64.0 },
            })
            .push(Text::<IcedBackend>::new("Community").large_title().bold())
            .push(
                Text::<IcedBackend>::new(
                    "Connect with other developers and share your PeakUI projects.",
                )
                .title3()
                .secondary(),
            )
            .push(Divider::<IcedBackend>::new())
            .push(ProxyView::new(move |theme_ctx| {
                let theme = theme_ctx.theme;
                container(
                    VStack::new_generic()
                        .spacing(12.0)
                        .push(Text::<IcedBackend>::new("Join our Discord").body().bold())
                        .push(
                            Text::<IcedBackend>::new(
                                "Discuss features, get help, and show off what you've built.",
                            )
                            .secondary(),
                        )
                        .view(theme_ctx),
                )
                .padding(40)
                .style(move |_| container::Style {
                    background: Some(theme.colors.surface_variant.scale_alpha(0.1).into()),
                    border: Border {
                        radius: 12.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into()
            })),
    )
}
