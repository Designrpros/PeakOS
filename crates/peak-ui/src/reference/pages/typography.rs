use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    // 1. Create the preview content
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .width(Length::Fill)
        .push(TypeRow::new(
            "Large Title",
            Text::<IcedBackend>::new("PeakOS").large_title(),
        ))
        .push(TypeRow::new(
            "Title 1",
            Text::<IcedBackend>::new("PeakOS").title1(),
        ))
        .push(TypeRow::new(
            "Title 2",
            Text::<IcedBackend>::new("PeakOS").title2(),
        ))
        .push(TypeRow::new(
            "Title 3",
            Text::<IcedBackend>::new("PeakOS").title3(),
        ))
        .push(TypeRow::new(
            "Headline",
            Text::<IcedBackend>::new("The quick brown fox").headline(),
        ))
        .push(TypeRow::new(
            "Body",
            Text::<IcedBackend>::new("The quick brown fox jumps over the lazy dog.").body(),
        ))
        .push(TypeRow::new(
            "Callout",
            Text::<IcedBackend>::new("The quick brown fox").callout(),
        ))
        .push(TypeRow::new(
            "Subheadline",
            Text::<IcedBackend>::new("The quick brown fox").subheadline(),
        ))
        .push(TypeRow::new(
            "Footnote",
            Text::<IcedBackend>::new("The quick brown fox").footnote(),
        ))
        .push(TypeRow::new(
            "Caption 1",
            Text::<IcedBackend>::new("The quick brown fox").caption1(),
        ))
        .push(TypeRow::new(
            "Caption 2",
            Text::<IcedBackend>::new("The quick brown fox").caption2(),
        ));

    // 2. Wrap in ComponentDoc
    PageResult::new(ComponentDoc::new(
        "Typography",
        "Use semantic styles to ensure consistency and accessibility across different platforms.",
        r#"
Text::new("PeakOS")
    .large_title()
    .bold()
    .color(theme.colors.text_primary)

Text::new("Caption text")
    .caption1()
    .secondary()
"#,
        Arc::new(preview),
    ))
}

struct TypeRow {
    label: String,
    sample: Text<IcedBackend>,
}

impl TypeRow {
    fn new(label: impl Into<String>, sample: Text<IcedBackend>) -> Self {
        Self {
            label: label.into(),
            sample,
        }
    }
}

impl View<Message, IcedBackend> for TypeRow {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        container(
            HStack::new_generic()
                .spacing(24.0)
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .push(
                    Text::<IcedBackend>::new(self.label.clone())
                        .caption1()
                        .secondary()
                        .width(Length::Fixed(100.0)),
                )
                .push(self.sample.clone())
                .view(context),
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
    }
}
