use crate::core::{Context, View};
use iced::widget::{column, container, row};
use iced::{Element, Length, Renderer, Theme};

pub struct NavigationSplitView<Message> {
    sidebar: Box<dyn View<Message>>,
    content: Box<dyn View<Message>>,
    inspector: Option<Box<dyn View<Message>>>,
    force_sidebar_on_slim: bool,
    on_back: Option<Message>,
}

impl<Message: Clone> NavigationSplitView<Message> {
    pub fn new(
        sidebar: impl View<Message> + 'static,
        content: impl View<Message> + 'static,
    ) -> Self {
        Self {
            sidebar: Box::new(sidebar),
            content: Box::new(content),
            inspector: None,
            force_sidebar_on_slim: false,
            on_back: None,
        }
    }

    pub fn inspector(mut self, inspector: impl View<Message> + 'static) -> Self {
        self.inspector = Some(Box::new(inspector));
        self
    }

    pub fn force_sidebar_on_slim(mut self, force: bool) -> Self {
        self.force_sidebar_on_slim = force;
        self
    }

    pub fn on_back(mut self, msg: Message) -> Self {
        self.on_back = Some(msg);
        self
    }
}

#[allow(unused_imports)]
use crate::scroll_view::ScrollView;

impl<Message: Clone + 'static> View<Message> for NavigationSplitView<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        if context.is_slim() {
            if self.force_sidebar_on_slim {
                // Mobile Sidebar View
                let sidebar_scroll = crate::scroll_view::ScrollView::apply_style(
                    iced::widget::scrollable(self.sidebar.view(context)),
                    &theme,
                );

                container(sidebar_scroll)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(move |_| container::Style {
                        background: Some(theme.colors.background.into()),
                        text_color: Some(theme.colors.text_primary),
                        ..Default::default()
                    })
                    .into()
            } else {
                // Mobile Content View (with optional back button)
                let mut content_col = column![]
                    .spacing(0)
                    .width(Length::Fill)
                    .height(Length::Fill);

                if let Some(back_msg) = self.on_back.clone() {
                    let back_button = crate::controls::Button::label("Back")
                        .icon("chevron_left")
                        .variant(crate::modifiers::Variant::Ghost)
                        .on_press(back_msg)
                        .width(Length::Shrink)
                        .view(context);

                    content_col = content_col.push(
                        container(back_button)
                            .padding([16, 20])
                            .width(Length::Fill)
                            .style(move |_| container::Style {
                                background: Some(theme.colors.background.into()),
                                ..Default::default()
                            }),
                    );
                }

                let content_view = crate::scroll_view::ScrollView::apply_style(
                    iced::widget::scrollable(self.content.view(context)),
                    &theme,
                )
                .width(Length::Fill)
                .height(Length::Fill);

                content_col = content_col.push(
                    container(content_view)
                        .width(Length::Fill)
                        .height(Length::Fill),
                );

                let base_content = container(content_col)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(move |_| container::Style {
                        background: Some(theme.colors.background.into()),
                        text_color: Some(theme.colors.text_primary),
                        ..Default::default()
                    });

                // Use Stack to support Inspector Overlay
                let mut stack = iced::widget::stack![base_content];

                if let Some(inspector) = &self.inspector {
                    // Dimmed Background
                    stack = stack.push(
                        container(iced::widget::Space::new(Length::Fill, Length::Fill)).style(
                            |_| container::Style {
                                background: Some(
                                    iced::Color {
                                        a: 0.5,
                                        ..iced::Color::BLACK
                                    }
                                    .into(),
                                ),
                                ..Default::default()
                            },
                        ),
                    );

                    // Sheet Content
                    let sheet = container(crate::scroll_view::ScrollView::apply_style(
                        iced::widget::scrollable(inspector.view(context)),
                        &theme,
                    ))
                    .width(Length::Fill)
                    .height(Length::FillPortion(1)) // Take up half screen? Or fixed height?
                    .padding(16)
                    .style(move |_| container::Style {
                        background: Some(theme.colors.surface_variant.into()),
                        border: iced::Border {
                            radius: 16.0.into(), // Top corners rounded - simplified to all for now
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    // Align to bottom
                    stack = stack.push(
                        container(column![
                            // Push content down
                            iced::widget::Space::new(Length::Fill, Length::FillPortion(1)),
                            // The Sheet
                            sheet.height(Length::FillPortion(1))
                        ])
                        .height(Length::Fill)
                        .align_y(iced::alignment::Vertical::Bottom),
                    );
                }

                stack.into()
            }
        } else {
            // Desktop Layout
            let mut main_row = row![
                // 1. Sidebar
                container(
                    crate::scroll_view::ScrollView::apply_style(
                        iced::widget::scrollable(self.sidebar.view(context)),
                        &theme,
                    )
                    .height(Length::Fill)
                )
                .width(Length::Fixed(180.0))
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(if theme.colors.background.r < 0.1 {
                        iced::Color::from_rgb8(28, 28, 30).into()
                    } else {
                        let mut c = theme.colors.surface;
                        c.a = theme.glass_opacity;
                        c.into()
                    }),
                    border: iced::Border {
                        color: theme.colors.divider,
                        width: 1.0,
                        ..Default::default()
                    },
                    text_color: Some(theme.colors.text_primary),
                    ..Default::default()
                }),
                // 2. Content
                container(
                    crate::scroll_view::ScrollView::apply_style(
                        iced::widget::scrollable(self.content.view(context)),
                        &theme,
                    )
                    .height(Length::Fill)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(theme.colors.background.into()),
                    text_color: Some(theme.colors.text_primary),
                    ..Default::default()
                })
            ]
            .height(Length::Fill);

            // 3. Inspector (Optional)
            if let Some(inspector) = &self.inspector {
                main_row = main_row.push(
                    container(
                        crate::scroll_view::ScrollView::apply_style(
                            iced::widget::scrollable(inspector.view(context)),
                            &theme,
                        )
                        .height(Length::Fill),
                    )
                    .width(Length::Fixed(220.0)) // Standard inspector width
                    .height(Length::Fill)
                    .style(move |_| container::Style {
                        background: Some(if theme.colors.background.r < 0.1 {
                            iced::Color::from_rgb8(28, 28, 30).into() // Darker sidebar match
                        } else {
                            let mut c = theme.colors.surface;
                            c.a = 0.5; // Slightly more transparent than sidebar
                            c.into()
                        }),
                        border: iced::Border {
                            color: theme.colors.divider,
                            width: 1.0,
                            ..Default::default()
                        },
                        text_color: Some(theme.colors.text_primary),
                        ..Default::default()
                    }),
                );
            }

            container(main_row)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(theme.colors.background.into()),
                    ..Default::default()
                })
                .into()
        }
    }
}
