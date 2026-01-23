use super::super::app::{App, Message};
use super::{CanvasView, InspectorView, SidebarView, TabBarView};
use crate::nav_split_view::NavigationSplitView;
use crate::prelude::*;

pub struct ContentView {
    pub active_tab: String,
    pub show_search: bool,
    pub show_inspector: bool,
    pub show_sidebar: bool,
    pub show_user_profile: bool,
    pub navigation_mode: String,
    pub expanded_sections: std::collections::HashSet<String>,
}

impl ContentView {
    pub fn new(app: &App) -> Self {
        Self {
            active_tab: app.active_tab.clone(),
            show_search: app.show_search,
            show_inspector: app.show_inspector,
            show_sidebar: app.show_sidebar,
            show_user_profile: app.show_user_profile,
            navigation_mode: app.navigation_mode.clone(),
            expanded_sections: app.expanded_sections.clone(),
        }
    }

    pub fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let is_mobile = context.size.width < 900.0;

        // --- 1. Sub-Views ---
        let sidebar = SidebarView::new(self.active_tab.clone(), self.expanded_sections.clone());
        let canvas = CanvasView::new(self.active_tab.clone(), self.navigation_mode.clone());
        let inspector = InspectorView::new();
        let tabbar = TabBarView::new(self.navigation_mode.clone());

        // --- 2. Main Layout (Three-Column Split) ---
        let mut split_view = NavigationSplitView::new(sidebar, canvas)
            .force_sidebar_on_slim(self.show_sidebar && is_mobile);

        if self.show_inspector {
            split_view = split_view.inspector(inspector);
        }

        // --- 3. UI Overlays (Toolbars & Dock) ---
        let show_search = self.show_search;
        let show_inspector = self.show_inspector;

        let toolbar = HStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .push(Space::<IcedBackend>::new(Length::Fill, 0.0.into()))
            .push(
                ToolbarGroup::new()
                    .push(ProxyView::new(move |ctx| {
                        if ctx.size.width < 900.0 {
                            ToolbarItem::new()
                                .icon("menu")
                                .on_press(Message::ToggleSidebar)
                                .view(ctx)
                        } else {
                            Space::<IcedBackend>::new(0.0.into(), 0.0.into()).view(ctx)
                        }
                    }))
                    .push(
                        ToolbarItem::new()
                            .icon("search")
                            .active(show_search)
                            .on_press(Message::ToggleSearch),
                    )
                    .push(
                        ToolbarItem::new()
                            .icon("sidebar")
                            .active(show_inspector)
                            .on_press(Message::ToggleInspector),
                    )
                    .push(
                        ToolbarItem::new()
                            .icon("user")
                            .on_press(Message::ToggleUserProfile),
                    ),
            )
            .padding(12);

        // --- 4. Final Assembly ---
        let mut final_view: Element<'_, Message> = stack![
            split_view.view(context),
            // Right-aligned pill toolbar
            toolbar.view(context),
            // Bottom Dock
            container(
                HStack::<Message, IcedBackend>::new_generic()
                    .width(Length::Fill)
                    .push(Space::<IcedBackend>::new(Length::Fill, 0.0.into()))
                    .push(tabbar)
                    .push(Space::<IcedBackend>::new(Length::Fill, 0.0.into()))
                    .view(context)
            )
            .padding(Padding {
                top: 0.0,
                right: 20.0,
                bottom: 32.0,
                left: 20.0
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::End),
        ]
        .into();

        if self.show_user_profile {
            final_view = stack![final_view, self.user_profile_menu(context)].into();
        }

        final_view
    }

    fn user_profile_menu(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let menu = container(
            VStack::new_generic()
                .spacing(12.0)
                .padding(16)
                .push(Text::<IcedBackend>::new("Guest User").body().bold())
                .push(Divider::<IcedBackend>::new())
                .push(Text::<IcedBackend>::new("Profile Settings").caption1())
                .push(Text::<IcedBackend>::new("API Keys").caption1())
                .push(Text::<IcedBackend>::new("Sign Out").caption1().secondary())
                .view(context),
        )
        .width(200)
        .style(move |_| container::Style {
            background: Some(theme.colors.surface.into()),
            border: Border {
                radius: 12.0.into(),
                color: theme.colors.border,
                width: 1.0,
            },
            shadow: Shadow {
                color: Color {
                    a: 0.1,
                    ..Color::BLACK
                },
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        });

        container(menu)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::End)
            .align_y(Alignment::Start)
            .padding(Padding {
                top: 74.0,
                right: 40.0,
                bottom: 0.0,
                left: 0.0,
            })
            .into()
    }
}
