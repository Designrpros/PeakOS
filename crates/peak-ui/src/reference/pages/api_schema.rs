use super::super::app::Message;
use super::super::mcp;
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
            .push(
                Text::<IcedBackend>::new("Framework Semantic Schema")
                    .large_title()
                    .bold(),
            )
            .push(
                Text::<IcedBackend>::new("This metadata is served via MCP for LLM extraction.")
                    .title3()
                    .secondary(),
            )
            .push(Divider::<IcedBackend>::new())
            .push(ProxyView::new(move |ctx| {
                let theme = ctx.theme;
                container(
                    Text::<IcedBackend>::new(format!("{:#?}", mcp::get_framework_schema()))
                        .body()
                        .view(ctx),
                )
                .padding(24)
                .width(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(theme.colors.surface_variant.scale_alpha(0.3).into()),
                    border: Border {
                        radius: 12.0.into(),
                        color: theme.colors.border.scale_alpha(0.1),
                        width: 1.0,
                    },
                    ..Default::default()
                })
                .into()
            })),
    )
}
