use super::super::app::Message;
use crate::prelude::*;

pub fn view(_context: &Context, is_mobile: bool) -> Box<dyn View<Message, IcedBackend>> {
    Box::new(
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
                    .push(Text::<IcedBackend>::new("Typography").large_title().bold())
                    .push(Text::<IcedBackend>::new("Use semantic styles to ensure consistency and accessibility across different platforms.").title3().secondary())
            )
            .push(Divider::<IcedBackend>::new())
            .push(
                VStack::new_generic()
                    .spacing(24.0)
                    .push(type_row("Large Title", Text::<IcedBackend>::new("PeakOS").large_title()))
                    .push(type_row("Title 1", Text::<IcedBackend>::new("PeakOS").title1()))
                    .push(type_row("Title 2", Text::<IcedBackend>::new("PeakOS").title2()))
                    .push(type_row("Title 3", Text::<IcedBackend>::new("PeakOS").title3()))
                    .push(type_row("Headline", Text::<IcedBackend>::new("The quick brown fox").headline()))
                    .push(type_row("Body", Text::<IcedBackend>::new("The quick brown fox jumps over the lazy dog.").body()))
                    .push(type_row("Callout", Text::<IcedBackend>::new("The quick brown fox").callout()))
                    .push(type_row("Subheadline", Text::<IcedBackend>::new("The quick brown fox").subheadline()))
                    .push(type_row("Footnote", Text::<IcedBackend>::new("The quick brown fox").footnote()))
                    .push(type_row("Caption 1", Text::<IcedBackend>::new("The quick brown fox").caption1()))
                    .push(type_row("Caption 2", Text::<IcedBackend>::new("The quick brown fox").caption2()))
            )
            .push(
                VStack::new_generic()
                    .spacing(16.0)
                    .push(Text::<IcedBackend>::new("Modifiers").title2().bold())
                    .push(
                        HStack::new_generic()
                            .spacing(16.0)
                            .push(Text::<IcedBackend>::new("Bold Text").body().bold())
                            .push(Text::<IcedBackend>::new("Dimmed Text").body().secondary())
                    )
            )
    )
}

fn type_row(label: &str, sample: Text<IcedBackend>) -> impl View<Message, IcedBackend> {
    let label = label.to_string();
    ProxyView::new(move |ctx| {
        let theme = ctx.theme;
        let label_inner = label.clone();
        let sample_inner = sample.clone();

        container(
            HStack::new_generic()
                .spacing(24.0)
                .align_y(Alignment::Center)
                .push(ProxyView::new(move |ctx_label| {
                    container(
                        Text::<IcedBackend>::new(label_inner.clone())
                            .caption1()
                            .secondary()
                            .view(ctx_label),
                    )
                    .width(100)
                    .into()
                }))
                .push(sample_inner)
                .view(ctx),
        )
        .padding(Padding::from([12, 0]))
        .width(Length::Fill)
        .style(move |_| container::Style {
            border: Border {
                color: theme.colors.border.scale_alpha(0.05),
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    })
}
