use super::super::app::Message;
use crate::prelude::*;

pub fn view(_context: &Context, is_mobile: bool) -> Box<dyn View<Message, IcedBackend>> {
    Box::new(
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(32.0)
            .padding(Padding {
                top: 96.0,
                right: if is_mobile { 20.0 } else { 64.0 },
                bottom: 120.0,
                left: if is_mobile { 20.0 } else { 64.0 },
            })
            .push(
                VStack::new_generic()
                    .spacing(8.0)
                    .push(Text::<IcedBackend>::new("Architecture Overview").large_title().bold())
                    .push(Text::<IcedBackend>::new("The core pillars of the PeakUI framework.").title3().secondary())
            )
            .push(Divider::<IcedBackend>::new())
            .push(
                VStack::new_generic()
                    .spacing(24.0)
                    .push(pillar_card(
                        "Cross-Platform Core",
                        "PeakUI is built on a custom backend-agnostic core. The same view logic can render to Iced (GPU) or a Terminal (ANSI).",
                        "monitor"
                    ))
                    .push(pillar_card(
                        "Semantic Design",
                        "Styles are based on semantic intent (e.g., .title1(), .caption1()) rather than raw pixel values.",
                        "type"
                    ))
                    .push(pillar_card(
                        "High Density",
                        "Designed for power users, PeakUI prioritizes information density and keyboard-first navigation.",
                        "cpu"
                    ))
            )
            .push(ProxyView::new(move |ctx| {
                let theme = ctx.theme;
                container(
                    VStack::new_generic()
                        .spacing(16.0)
                        .push(Text::<IcedBackend>::new("Next Steps").title2().bold())
                        .push(Text::<IcedBackend>::new("Check out the Basic Sizing or Typography sections to learn how to use the framework components."))
                        .push(
                            Button::new(Text::<IcedBackend>::new("Go to Basic Sizing"))
                                .on_press(Message::SetTab("Basic Sizing".into()))
                        )
                        .view(ctx)
                )
                .padding(32)
                .width(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(theme.colors.surface_variant.scale_alpha(0.2).into()),
                    border: Border {
                        radius: 16.0.into(),
                        color: theme.colors.border.scale_alpha(0.1),
                        width: 1.0,
                    },
                    ..Default::default()
                })
                .into()
            }))
    )
}

fn pillar_card(title: &str, description: &str, icon: &str) -> impl View<Message, IcedBackend> {
    let title = title.to_string();
    let description = description.to_string();
    let icon = icon.to_string();

    ProxyView::new(move |ctx| {
        let theme = ctx.theme;
        let icon_inner = icon.clone();
        let title_inner = title.clone();
        let description_inner = description.clone();

        container(
            HStack::new_generic()
                .spacing(20.0)
                .align_y(Alignment::Center)
                .push(ProxyView::new(move |ctx_inner| {
                    let t = ctx_inner.theme;
                    container(
                        Icon::<IcedBackend>::new(icon_inner.clone())
                            .size(24.0)
                            .color(t.colors.primary)
                            .view(ctx_inner),
                    )
                    .padding(16)
                    .style(move |_| container::Style {
                        background: Some(t.colors.primary.scale_alpha(0.1).into()),
                        border: Border {
                            radius: 12.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .into()
                }))
                .push(
                    VStack::new_generic()
                        .spacing(4.0)
                        .push(
                            Text::<IcedBackend>::new(title_inner.clone())
                                .title3()
                                .bold(),
                        )
                        .push(
                            Text::<IcedBackend>::new(description_inner.clone())
                                .callout()
                                .secondary(),
                        ),
                )
                .view(ctx),
        )
        .padding(20)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(theme.colors.surface.into()),
            border: Border {
                radius: 16.0.into(),
                color: theme.colors.border.scale_alpha(0.1),
                width: 1.0,
            },
            ..Default::default()
        })
        .into()
    })
}
