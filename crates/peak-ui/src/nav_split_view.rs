use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::widget::{column, container, row};
use iced::{Element, Length, Renderer, Theme};

pub struct NavigationSplitView<Message: 'static, B: Backend = IcedBackend> {
    sidebar: Box<dyn View<Message, B>>,
    content: Box<dyn View<Message, B>>,
    inspector: Option<Box<dyn View<Message, B>>>,
    force_sidebar_on_slim: bool,
    on_back: Option<Message>,
}

impl<Message: Clone + 'static> NavigationSplitView<Message, IcedBackend> {
    pub fn new(
        sidebar: impl View<Message, IcedBackend> + 'static,
        content: impl View<Message, IcedBackend> + 'static,
    ) -> Self {
        Self::new_generic(sidebar, content)
    }
}

impl<Message: Clone + 'static> NavigationSplitView<Message, TermBackend> {
    pub fn new_tui(
        sidebar: impl View<Message, TermBackend> + 'static,
        content: impl View<Message, TermBackend> + 'static,
    ) -> Self {
        Self::new_generic(sidebar, content)
    }
}

impl<Message: Clone + 'static, B: Backend> NavigationSplitView<Message, B> {
    pub fn new_generic(
        sidebar: impl View<Message, B> + 'static,
        content: impl View<Message, B> + 'static,
    ) -> Self {
        Self {
            sidebar: Box::new(sidebar),
            content: Box::new(content),
            inspector: None,
            force_sidebar_on_slim: false,
            on_back: None,
        }
    }

    pub fn inspector(mut self, inspector: impl View<Message, B> + 'static) -> Self {
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

impl<Message: Clone + 'static> View<Message, IcedBackend>
    for NavigationSplitView<Message, IcedBackend>
{
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        if context.is_slim() {
            if self.force_sidebar_on_slim {
                // Mobile Sidebar View
                container(self.sidebar.view(context))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style({
                        let bg_color = theme.colors.background;
                        let text_color = theme.colors.text_primary;
                        move |_| container::Style {
                            background: Some(bg_color.into()),
                            text_color: Some(text_color),
                            ..Default::default()
                        }
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
                            .style({
                                let bg_color = theme.colors.background;
                                move |_| container::Style {
                                    background: Some(bg_color.into()),
                                    ..Default::default()
                                }
                            }),
                    );
                }

                let content_view = self.content.view(context);

                content_col = content_col.push(
                    container(content_view)
                        .width(Length::Fill)
                        .height(Length::Fill),
                );

                let base_content = container(content_col)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style({
                        let bg_color = theme.colors.background;
                        let text_color = theme.colors.text_primary;
                        move |_| container::Style {
                            background: Some(bg_color.into()),
                            text_color: Some(text_color),
                            ..Default::default()
                        }
                    });

                // Use Stack to support Inspector Overlay
                let mut stack = iced::widget::stack![base_content]
                    .width(Length::Fill)
                    .height(Length::Fill);

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

                    let radius = context.radius(16.0);
                    let bg_color = theme.colors.surface_variant;
                    let sheet = container(inspector.view(context))
                        .width(Length::Fill)
                        .height(Length::FillPortion(1)) // Take up half screen? Or fixed height?
                        .padding(16)
                        .style({
                            let bg = bg_color;
                            let r = radius;
                            move |_| container::Style {
                                background: Some(bg.into()),
                                border: iced::Border {
                                    radius: r, // Top corners rounded - simplified to all for now
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
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
                container(self.sidebar.view(context))
                    .width(Length::Fixed(240.0 * context.theme.scaling))
                    .height(Length::Fill)
                    .style({
                        let bg = if theme.colors.background.r < 0.1 {
                            iced::Color::from_rgb8(28, 28, 30)
                        } else {
                            let mut c = theme.colors.surface_variant;
                            c.a = 0.5; // High transparency for glass
                            c
                        };
                        let text_color = theme.colors.text_primary;
                        move |_| container::Style {
                            background: Some(bg.into()),
                            text_color: Some(text_color),
                            ..Default::default()
                        }
                    }),
                // Right border divider
                container(iced::widget::Space::new(Length::Fixed(1.0), Length::Fill)).style({
                    let div_color = theme.colors.divider;
                    move |_| container::Style {
                        background: Some(div_color.into()),
                        ..Default::default()
                    }
                }),
                container(self.content.view(context))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style({
                        let bg_color = theme.colors.background;
                        let text_color = theme.colors.text_primary;
                        move |_| container::Style {
                            background: Some(bg_color.into()),
                            text_color: Some(text_color),
                            ..Default::default()
                        }
                    })
            ]
            .width(Length::Fill)
            .height(Length::Fill);

            // 3. Inspector (Optional)
            if let Some(inspector) = &self.inspector {
                main_row = main_row.push(
                    container(inspector.view(context))
                        .width(Length::Fixed(220.0)) // Standard inspector width
                        .height(Length::Fill)
                        .style({
                            let bg = if theme.colors.background.r < 0.1 {
                                iced::Color::from_rgb8(28, 28, 30)
                            } else {
                                let mut c = theme.colors.surface;
                                c.a = 0.5; // Slightly more transparent than sidebar
                                c
                            };
                            let div_color = theme.colors.divider;
                            let text_color = theme.colors.text_primary;
                            move |_| container::Style {
                                background: Some(bg.into()),
                                border: iced::Border {
                                    color: div_color,
                                    width: 1.0,
                                    ..Default::default()
                                },
                                text_color: Some(text_color),
                                ..Default::default()
                            }
                        }),
                );
            }

            container(main_row)
                .width(Length::Fill)
                .height(Length::Fill)
                .style({
                    let bg_color = theme.colors.background;
                    move |_| container::Style {
                        background: Some(bg_color.into()),
                        ..Default::default()
                    }
                })
                .into()
        }
    }
}

impl<Message: Clone + 'static> View<Message, TermBackend>
    for NavigationSplitView<Message, TermBackend>
{
    fn view(&self, context: &Context) -> String {
        let mut out = String::new();
        out.push_str("=== SIDEBAR ===\n");
        out.push_str(&self.sidebar.view(context));
        out.push_str("\n=== CONTENT ===\n");
        out.push_str(&self.content.view(context));
        out
    }
}
