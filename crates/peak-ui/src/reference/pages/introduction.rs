use super::super::app::Message;
use super::super::model::Page;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(context: &Context, is_mobile: bool) -> PageResult {
    let theme = context.theme;

    // --- Hero Section ---
    let hero = VStack::<Message, IcedBackend>::new_generic()
        .spacing(32.0)
        .align_x(if is_mobile {
            iced::Alignment::Center
        } else {
            iced::Alignment::Start
        })
        .width(Length::Fill)
        .push(
            VStack::new_generic()
                .spacing(12.0)
                .align_x(if is_mobile {
                    iced::Alignment::Center
                } else {
                    iced::Alignment::Start
                })
                .push(
                    Text::<IcedBackend>::new("PeakUI")
                        .size(if is_mobile { 48.0 } else { 84.0 })
                        .bold()
                        .color(theme.colors.text_primary),
                )
                .push(
                    Text::<IcedBackend>::new("The Operating System for your User Interface")
                        .size(if is_mobile { 24.0 } else { 32.0 })
                        .color(theme.colors.text_secondary),
                ),
        )
        .push(
            Text::<IcedBackend>::new("PeakUI is a cross-platform design system engine built for performance, type-safety, and absolute developer control across GUI, Terminal, and Neural interfaces.")
                .body()
                .color(theme.colors.text_secondary)
                .width(if is_mobile { Length::Fill } else { Length::Fixed(600.0) }),
        )
        .push(
            HStack::new_generic()
                .spacing(20.0)
                .align_y(iced::Alignment::Center)
                .push(
                    Button::label("Quick Start")
                        .on_press(Message::SetTab(Page::Architecture))
                        .size(ControlSize::Large)
                        .width(Length::Fixed(180.0)),
                )
                .push(
                    Button::label("Browse Catalog")
                        .variant(Variant::Soft)
                        .on_press(Message::SetTab(Page::ShowcaseButtons))
                        .size(ControlSize::Large)
                        .width(Length::Fixed(180.0)),
                ),
        );

    // --- Value Props Section ---
    let feature_card = |icon: &str, title: &str, desc: &str| {
        let icon = icon.to_string();
        let title = title.to_string();
        let desc = desc.to_string();

        ProxyView::<Message, IcedBackend>::new(move |ctx| {
            let t = ctx.theme;
            iced::widget::container(
                VStack::new_generic()
                    .spacing(20.0)
                    .push(
                        Icon::<IcedBackend>::new(icon.clone())
                            .size(28.0)
                            .color(t.colors.primary),
                    )
                    .push(
                        VStack::new_generic()
                            .spacing(12.0)
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
                    .view(&ctx),
            )
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill) // Uniform height
            .style(move |_| iced::widget::container::Style {
                background: Some(t.colors.surface.into()),
                border: iced::Border {
                    radius: 20.0.into(),
                    color: t.colors.border.scale_alpha(0.3),
                    width: 1.0,
                },
                ..Default::default()
            })
            .into()
        })
    };

    // --- Features Section ---
    let features = ProxyView::<Message, IcedBackend>::new(move |ctx| {
        if is_mobile {
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
                    "Built on Iced and WGPU for fluid, hardware-accelerated 120fps rendering.",
                ))
                .push(feature_card(
                    "shield",
                    "Type Safe",
                    "Leveraging Rust's ownership and type system for guaranteed reliability.",
                ))
                .view(ctx)
        } else {
            // Desktop/Tablet Row
            HStack::new_generic()
                .spacing(24.0)
                .width(Length::Fill)
                .height(Length::Fixed(240.0)) // Restore synchronization height
                .push(feature_card(
                    "layers",
                    "Modular Architecture",
                    "Composed of independent atoms and molecules for maximum reusability.",
                ))
                .push(feature_card(
                    "zap",
                    "High Performance",
                    "Built on Iced and WGPU for fluid, hardware-accelerated 120fps rendering.",
                ))
                .push(feature_card(
                    "shield",
                    "Type Safe",
                    "Leveraging Rust's ownership and type system for guaranteed reliability.",
                ))
                .view(ctx)
        }
    });

    // --- Quick Start Section ---
    let quick_start = VStack::new_generic()
        .spacing(24.0)
        .align_x(if is_mobile { iced::Alignment::Center } else { iced::Alignment::Start })
        .width(Length::Fill)
        .push(
            Text::<IcedBackend>::new("Seamless Implementation")
                .title2()
                .bold()
                .color(theme.colors.text_primary),
        )
        .push(ProxyView::<Message, IcedBackend>::new(move |ctx| {
            let t = ctx.theme;
            iced::widget::container(
                Text::<IcedBackend>::new(
                    "VStack::new()\n  .spacing(16.0)\n  .push(Text::new(\"Hello PeakUI\").title1())\n  .push(Button::label(\"Get Started\"))",
                )
                .size(14.0)
                .color(t.colors.text_primary)
                .view(ctx),
            )
            .padding(24)
            .width(if is_mobile {
                Length::Fill
            } else {
                Length::Fixed(500.0)
            })
            .style(move |_| iced::widget::container::Style {
                background: Some(t.colors.surface.scale_alpha(0.5).into()),
                border: iced::Border {
                    radius: 12.0.into(),
                    color: t.colors.border.scale_alpha(0.2),
                    width: 1.0,
                },
                ..Default::default()
            })
            .into()
        }));

    // --- Footer ---
    let footer = VStack::new_generic()
        .spacing(32.0)
        .align_x(if is_mobile {
            iced::Alignment::Center
        } else {
            iced::Alignment::Start
        })
        .width(Length::Fill)
        .push(
            VStack::new_generic()
                .spacing(16.0)
                .align_x(if is_mobile {
                    iced::Alignment::Center
                } else {
                    iced::Alignment::Start
                })
                .push(
                    Text::<IcedBackend>::new("Ready to scale?")
                        .title1()
                        .bold()
                        .color(theme.colors.text_primary),
                )
                .push(
                    Text::<IcedBackend>::new(
                        "Explore our technical documentation and design patterns.",
                    )
                    .body()
                    .color(theme.colors.text_secondary),
                ),
        )
        .push(
            HStack::new_generic()
                .spacing(24.0)
                .push(
                    Button::<Message, IcedBackend>::label("View Roadmap")
                        .variant(Variant::Ghost)
                        .on_press(Message::SetTab(Page::Roadmap)),
                )
                .push(
                    Button::<Message, IcedBackend>::label("Project Structure")
                        .variant(Variant::Ghost)
                        .on_press(Message::SetTab(Page::ProjectStructure)),
                ),
        );

    PageResult::new(
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(if is_mobile { 64.0 } else { 80.0 })
            .padding(Padding {
                top: if is_mobile { 48.0 } else { 48.0 },
                right: if is_mobile { 24.0 } else { 48.0 },
                bottom: 120.0,
                left: if is_mobile { 24.0 } else { 48.0 },
            })
            .align_x(iced::Alignment::Start) // CRITICAL: Standardized Left Alignment
            .push(hero)
            .push(features)
            .push(quick_start)
            .push(footer),
    )
}
