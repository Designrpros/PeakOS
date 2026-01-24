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
            .push(Text::<IcedBackend>::new("Customizations").large_title().bold())
            .push(
                Text::<IcedBackend>::new("Learn how to customize themes, colors, and global styles.")
                    .title3()
                    .secondary(),
            )
            .push(Divider::<IcedBackend>::new())
            .push(ProxyView::new(move |theme_ctx| {
                let theme = theme_ctx.theme;
                container(
                    VStack::<Message, IcedBackend>::new_generic()
                        .spacing(12.0)
                        .push(Text::<IcedBackend>::new("Flexible Design System").body().bold())
                        .push(
                            Text::<IcedBackend>::new(
                                "PeakUI uses ThemeTokens to control everything from colors to border radii. You can swap themes at runtime for instant UI updates.",
                            )
                            .secondary(),
                        )
                        .view(theme_ctx),
                )
                .padding(40)
                .style({
                    let radius = theme_ctx.radius(12.0);
                    let bg_color = theme.colors.surface_variant.scale_alpha(0.1);
                    move |_| container::Style {
                        background: Some(bg_color.into()),
                        border: Border {
                            radius,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .into()
            })),
    )
}
