use super::super::app::Message;
use super::super::model::Page;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(context: &Context, is_mobile: bool) -> PageResult {
    let theme = context.theme;

    // --- Hero Section ---
    let hero = VStack::<Message, IcedBackend>::new_generic()
        .spacing(32.0)
        .align_x(iced::Alignment::Center)
        .width(Length::Fill)
        .push(
            Icon::<IcedBackend>::new("monitor") // Or a better logo icon if available
                .size(64.0)
                .color(theme.colors.primary),
        )
        .push(
            VStack::new_generic()
                .spacing(16.0)
                .align_x(iced::Alignment::Center)
                .push(
                    Text::<IcedBackend>::new("PeakUI")
                        .size(64.0) // Very large title
                        .bold()
                        .color(theme.colors.text_primary),
                )
                .push(
                    Text::<IcedBackend>::new("The Operating System for your User Interface")
                        .title2()
                        .color(theme.colors.text_secondary),
                ),
        )
        .push(
            HStack::new_generic()
                .spacing(24.0)
                .align_y(iced::Alignment::Center)
                .push(
                    Button::label("Read the Guide")
                        .on_press(Message::SetTab(Page::Architecture))
                        .size(ControlSize::Large)
                        .width(Length::Fixed(200.0)),
                )
                .push(
                    Button::label("Browse Catalog")
                        .variant(Variant::Soft)
                        .on_press(Message::SetTab(Page::ShowcaseButtons)) // Points to Button page
                        .size(ControlSize::Large)
                        .width(Length::Fixed(200.0)),
                ),
        );

    // --- Value Props Section ---
    let feature_card = |icon: &str, title: &str, desc: &str| {
        let icon = icon.to_string();
        let title = title.to_string();
        let desc = desc.to_string();

        ProxyView::new(move |ctx| {
            let t = ctx.theme;
            iced::widget::container(
                VStack::new_generic()
                    .spacing(16.0)
                    .push(
                        Icon::<IcedBackend>::new(icon.clone())
                            .size(32.0)
                            .color(t.colors.primary),
                    )
                    .push(
                        VStack::new_generic()
                            .spacing(8.0)
                            .push(
                                Text::<IcedBackend>::new(title.clone())
                                    .title3()
                                    .bold()
                                    .color(t.colors.text_primary),
                            )
                            .push(
                                Text::<IcedBackend>::new(desc.clone())
                                    .body()
                                    .color(t.colors.text_secondary),
                            ),
                    )
                    .view(&ctx), // Compile Fix: Render the view
            )
            .padding(32)
            .width(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(t.colors.surface.into()),
                border: iced::Border {
                    radius: 16.0.into(),
                    color: t.colors.border.scale_alpha(0.5),
                    width: 1.0,
                },
                ..Default::default()
            })
            .into()
        })
    };

    let features = if is_mobile {
        VStack::new_generic()
            .spacing(24.0)
            .width(Length::Fill)
            .push(feature_card(
                "layers",
                "Modular Architecture",
                "Composed of independent atoms and molecules for maximum reusability.",
            ))
            .push(feature_card(
                "zap",
                "High Performance",
                "Built on Iced and WGPU for fluid, 120fps rendering.",
            ))
            .push(feature_card(
                "shield",
                "Type Safe",
                "Leveraging Rust's type system for crash-free reliability.",
            ))
    } else {
        // Desktop Grid (Manual HStack)
        VStack::new_generic()
            .spacing(24.0)
            .width(Length::Fill)
            .push(
                HStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .push(feature_card(
                        "layers",
                        "Modular Architecture",
                        "Composed of independent atoms and molecules for maximum reusability.",
                    ))
                    .push(feature_card(
                        "zap",
                        "High Performance",
                        "Built on Iced and WGPU for fluid, 120fps rendering.",
                    ))
                    .push(feature_card(
                        "shield",
                        "Type Safe",
                        "Leveraging Rust's type system for crash-free reliability.",
                    )),
            )
    };

    // --- Footer / CTA ---
    let footer = VStack::new_generic()
        .spacing(16.0)
        .align_x(iced::Alignment::Center)
        .width(Length::Fill)
        .push(
            Text::<IcedBackend>::new("Ready to start building?")
                .title2()
                .bold()
                .color(theme.colors.text_primary),
        )
        .push(
            Button::<Message, IcedBackend>::label("View Project Structure")
                .variant(Variant::Ghost)
                .on_press(Message::SetTab(Page::ProjectStructure)),
        );

    PageResult::new(
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(96.0) // Big spacing between sections
            .padding(Padding {
                top: context.safe_area.top,
                right: if is_mobile { 24.0 } else { 96.0 },
                bottom: context.safe_area.bottom.max(120.0),
                left: if is_mobile { 24.0 } else { 96.0 },
            })
            .align_x(iced::Alignment::Center)
            .push(hero)
            .push(features)
            .push(footer),
    )
}
