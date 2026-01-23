use super::super::app::Message;
use crate::prelude::*;
use std::collections::HashSet;

pub struct SidebarView {
    pub active_tab: String,
    pub expanded_sections: HashSet<String>,
}

impl SidebarView {
    pub fn new(active_tab: String, expanded_sections: HashSet<String>) -> Self {
        Self {
            active_tab,
            expanded_sections,
        }
    }
}

impl View<Message, IcedBackend> for SidebarView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let active_tab = &self.active_tab;

        container(
            ScrollView::new(
                VStack::new_generic()
                    .spacing(4.0)
                    .padding(Padding {
                        top: 96.0,
                        right: 16.0,
                        bottom: 40.0,
                        left: 16.0,
                    })
                    .push(sidebar_section_header("GETTING STARTED"))
                    .push(sidebar_item(
                        "Introduction",
                        "book",
                        active_tab == "Introduction",
                    ))
                    .push(sidebar_item("Roadmap", "map", active_tab == "Roadmap"))
                    .push(sidebar_item(
                        "Community",
                        "users",
                        active_tab == "Community",
                    ))
                    .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
                    .push(sidebar_section_header("CORE CONCEPTS"))
                    .push(sidebar_item("Overview", "layers", active_tab == "Overview"))
                    .push(sidebar_item(
                        "Customizations",
                        "palette",
                        active_tab == "Customizations",
                    ))
                    .push(sidebar_item(
                        "Basic Sizing",
                        "maximize",
                        active_tab == "Basic Sizing",
                    ))
                    .push(sidebar_item(
                        "Typography",
                        "type",
                        active_tab == "Typography",
                    ))
                    .push(sidebar_item("Layout", "grid", active_tab == "Layout"))
                    .push(sidebar_item("Docks", "monitor", active_tab == "Docks"))
                    .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
                    .push(self.tree_section(
                        "COMPONENTS",
                        "box",
                        vec![
                            ("Atoms", vec!["Text", "Icon", "Divider", "Button"]),
                            (
                                "Containers",
                                vec!["VStack", "HStack", "ZStack", "ScrollView", "Card"],
                            ),
                            ("Navigation", vec!["Sidebar", "Tabbar", "Modal"]),
                        ],
                    ))
                    .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
                    .push(sidebar_section_header("DEVELOPER"))
                    .push(sidebar_item(
                        "API Schema",
                        "code",
                        active_tab == "API Schema",
                    )),
            )
            .view(context),
        )
        .width(260)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(theme.colors.surface.scale_alpha(0.5).into()),
            border: Border {
                color: theme.colors.border.scale_alpha(0.05),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    }
}

impl SidebarView {
    fn tree_section(
        &self,
        title: &str,
        icon: &str,
        sections: Vec<(&str, Vec<&str>)>,
    ) -> impl View<Message, IcedBackend> {
        let expanded = self.expanded_sections.contains(title);
        let title_owned = title.to_string();
        let icon_owned = icon.to_string();
        let active_tab = self.active_tab.clone();

        let sections_owned: Vec<(String, Vec<String>)> = sections
            .into_iter()
            .map(|(s, items)| {
                (
                    s.to_string(),
                    items.into_iter().map(|i| i.to_string()).collect(),
                )
            })
            .collect();

        ProxyView::new(move |ctx| {
            let mut col = VStack::new_generic().spacing(4.0);

            // Header
            col = col.push(
                Button::new(
                    HStack::new_generic()
                        .spacing(8.0)
                        .align_y(Alignment::Center)
                        .push(
                            Icon::<IcedBackend>::new(if expanded {
                                "chevron-down"
                            } else {
                                "chevron-right"
                            })
                            .size(12.0)
                            .secondary(),
                        )
                        .push(
                            Icon::<IcedBackend>::new(icon_owned.clone())
                                .size(14.0)
                                .secondary(),
                        )
                        .push(
                            Text::<IcedBackend>::new(title_owned.clone())
                                .caption1()
                                .bold()
                                .secondary(),
                        ),
                )
                .variant(Variant::Ghost)
                .on_press(Message::ToggleSection(title_owned.clone())),
            );

            if expanded {
                for (sub_title, items) in sections_owned.clone() {
                    let sub_title_inner = sub_title.clone();
                    let items_inner = items.clone();
                    let active_tab_inner = active_tab.clone();

                    col = col.push(
                        VStack::new_generic()
                            .spacing(2.0)
                            .padding(Padding {
                                left: 20.0,
                                ..Default::default()
                            })
                            .push(ProxyView::new(move |ctx_inner| {
                                container(
                                    Text::<IcedBackend>::new(sub_title_inner.clone())
                                        .caption2()
                                        .bold()
                                        .secondary()
                                        .view(ctx_inner),
                                )
                                .padding(Padding::from([4, 8]))
                                .into()
                            }))
                            .push(
                                VStack::new_generic()
                                    .spacing(2.0)
                                    .padding(Padding {
                                        left: 8.0,
                                        ..Default::default()
                                    })
                                    .extend(items_inner.into_iter().map(move |item| {
                                        sidebar_item(
                                            item.clone(),
                                            "circle",
                                            active_tab_inner == item,
                                        )
                                    })),
                            ),
                    );
                }
            }
            col.view(ctx)
        })
    }
}

fn sidebar_section_header(label: &str) -> impl View<Message, IcedBackend> {
    let label = label.to_string();
    ProxyView::new(move |ctx| {
        container(
            Text::<IcedBackend>::new(label.clone())
                .caption2()
                .bold()
                .secondary()
                .view(ctx),
        )
        .padding(Padding::from([8, 12]))
        .into()
    })
}

fn sidebar_item(
    label: impl Into<String>,
    icon: impl Into<String>,
    active: bool,
) -> impl View<Message, IcedBackend> {
    SidebarItem::new(label, icon, active)
}

struct SidebarItem {
    label: String,
    icon: String,
    active: bool,
}

impl SidebarItem {
    fn new(label: impl Into<String>, icon: impl Into<String>, active: bool) -> Self {
        Self {
            label: label.into(),
            icon: icon.into(),
            active,
        }
    }
}

impl View<Message, IcedBackend> for SidebarItem {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let active = self.active;

        Button::new(
            HStack::new_generic()
                .spacing(12.0)
                .padding(Padding::from([6, 12]))
                .align_y(Alignment::Center)
                .push(
                    Icon::<IcedBackend>::new(self.icon.clone())
                        .size(14.0)
                        .color(if active {
                            theme.colors.primary
                        } else {
                            theme.colors.text_secondary
                        }),
                )
                .push(if active {
                    Text::<IcedBackend>::new(self.label.clone())
                        .caption1()
                        .bold()
                } else {
                    Text::<IcedBackend>::new(self.label.clone()).caption1()
                }),
        )
        .variant(if active {
            Variant::Soft
        } else {
            Variant::Ghost
        })
        .width(Length::Fill)
        .on_press(Message::SetTab(self.label.clone()))
        .view(context)
    }
}
