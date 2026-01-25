use super::super::app::Message;
use super::super::page::PageResult;
use crate::prelude::*;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let preview = ProxyView::new(move |theme_ctx| {
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
    });

    PageResult::new(ComponentDoc::new(
        "Customizations",
        "Learn how to customize themes, colors, and global styles.",
        r#"
// Accessing theme in a view
let theme = context.theme;
let primary = theme.colors.primary;

// Applying rounded corners
let radius = context.radius(12.0);
"#,
        Arc::new(preview),
    ))
}
