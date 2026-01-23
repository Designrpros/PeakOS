use super::super::app::Message;
use crate::prelude::*;

pub fn view(
    name: &str,
    _context: &Context,
    is_mobile: bool,
) -> Box<dyn View<Message, IcedBackend>> {
    let name = name.to_string();
    Box::new(
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(24.0)
            .padding(Padding {
                top: 96.0,
                right: if is_mobile { 20.0 } else { 64.0 },
                bottom: 120.0,
                left: if is_mobile { 20.0 } else { 64.0 },
            })
            .push(Text::<IcedBackend>::new(format!("Component: {}", name)).large_title().bold())
            .push(Text::<IcedBackend>::new("This component is part of the PeakUI Standard Library.").title3().secondary())
            .push(Divider::<IcedBackend>::new())
            .push(ProxyView::new(move |theme_ctx| {
                let theme = theme_ctx.theme;
                container(
                    VStack::new_generic()
                        .spacing(12.0)
                        .push(Text::<IcedBackend>::new("Coming Soon").body().bold())
                        .push(Text::<IcedBackend>::new("A comprehensive interactive playground for this component is currently under development.").secondary())
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
