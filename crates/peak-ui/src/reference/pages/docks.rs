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
            .push(Text::<IcedBackend>::new("Dock Component").large_title().bold())
            .push(Text::<IcedBackend>::new("The Dock is a minimalistic, floating navigation component used for high-level app switching.").title3().secondary())
            .push(Divider::<IcedBackend>::new())
            .push(ProxyView::new(move |theme_ctx| {
                let theme = theme_ctx.theme;
                container(
                    VStack::new_generic()
                        .spacing(12.0)
                        .push(Text::<IcedBackend>::new("Floating & Responsive").body().bold())
                        .push(Text::<IcedBackend>::new("It automatically adapts to screen width and can be positioned at the bottom center or sides.").secondary())
                        .view(theme_ctx)
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
            }))
    )
}
