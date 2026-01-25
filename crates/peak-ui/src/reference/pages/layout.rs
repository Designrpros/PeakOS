use super::super::page::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;

use super::super::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool) -> PageResult {
    let preview = ZStack::new_generic()
        .push(
            Image::new(crate::assets::Asset::MeshBackground.path())
                .width(Length::Fill)
                .height(Length::Fixed(340.0))
                .radius(16.0),
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
                                    .radius(4.0),
                            )
                            .push(
                                Rectangle::<IcedBackend>::new(8.0.into(), 8.0.into())
                                    .color(Color::from_rgb(1.0, 0.8, 0.2))
                                    .radius(4.0),
                            )
                            .push(
                                Rectangle::<IcedBackend>::new(8.0.into(), 8.0.into())
                                    .color(Color::from_rgb(0.2, 0.8, 0.2))
                                    .radius(4.0),
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
        }));

    PageResult::new(ComponentDoc::new(
        "Layout Engine",
        "Use VStack, HStack, and ZStack to compose complex, responsive layouts with ease.",
        r#"
VStack::new()
    .spacing(16.0)
    .align_x(Alignment::Center)
    .push(Header)
    .push(Content)

HStack::new()
    .push(Sidebar)
    .push(MainContent)
"#,
        Arc::new(preview),
    ))
    .searchable("", "Search layout...", |_| Message::ToggleSearch)
}
