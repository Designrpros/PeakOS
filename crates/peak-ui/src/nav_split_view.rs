use crate::core::{Context, View};
use iced::widget::{column, container, row};
use iced::{Element, Length, Renderer, Theme};

pub struct NavigationSplitView<Message> {
    sidebar: Box<dyn View<Message>>,
    content: Box<dyn View<Message>>,
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
            force_sidebar_on_slim: false,
            on_back: None,
        }
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
                        background: Some(theme.background.into()),
                        text_color: Some(theme.text),
                        ..Default::default()
                    })
                    .into()
            } else {
                // Mobile Content View (with optional back button)
                let mut content_stack: iced::widget::Column<'static, Message, Theme, Renderer> =
                    column![]
                        .spacing(0)
                        .width(Length::Fill)
                        .height(Length::Fill);

                if let Some(back_msg) = self.on_back.clone() {
                    let back_button = crate::controls::Button::new("Back")
                        .icon("chevron_left")
                        .style(crate::controls::ButtonStyle::Ghost)
                        .on_press(back_msg)
                        .width(Length::Shrink)
                        .view(context);

                    content_stack = content_stack.push(
                        container(back_button)
                            .padding([16, 20])
                            .width(Length::Fill)
                            .style(move |_| container::Style {
                                background: Some(theme.background.into()),
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

                content_stack = content_stack.push(
                    container(content_view)
                        .width(Length::Fill)
                        .height(Length::Fill),
                );

                container(content_stack)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(move |_| container::Style {
                        background: Some(theme.background.into()),
                        text_color: Some(theme.text),
                        ..Default::default()
                    })
                    .into()
            }
        } else {
            // Desktop: Sidebar + Content
            container(
                row![
                    container(
                        crate::scroll_view::ScrollView::apply_style(
                            iced::widget::scrollable(self.sidebar.view(context)),
                            &theme,
                        )
                        .height(Length::Fill)
                    )
                    .width(Length::Fixed(260.0))
                    .height(Length::Fill)
                    .style(move |_| container::Style {
                        background: Some(if theme.background.r < 0.1 {
                            iced::Color::from_rgb8(28, 28, 30).into()
                        } else {
                            theme.glass_bg.into()
                        }),
                        border: iced::Border {
                            color: theme.divider,
                            width: 1.0,
                            ..Default::default()
                        },
                        text_color: Some(theme.text),
                        ..Default::default()
                    }),
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
                        background: Some(theme.background.into()),
                        text_color: Some(theme.text),
                        ..Default::default()
                    })
                ]
                .height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(theme.background.into()),
                ..Default::default()
            })
            .into()
        }
    }
}
