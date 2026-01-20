use crate::core::{Context, View};
use iced::{Element, Length, Renderer, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormStyle {
    Grouped,
    Plain,
}

pub struct Form<Message> {
    sections: Vec<Box<dyn View<Message>>>,
    style: FormStyle,
}

impl<Message> Form<Message> {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            style: FormStyle::Grouped,
        }
    }

    pub fn style(mut self, style: FormStyle) -> Self {
        self.style = style;
        self
    }

    pub fn push(mut self, section: impl View<Message> + 'static) -> Self {
        self.sections.push(Box::new(section));
        self
    }
}

impl<Message: 'static> View<Message> for Form<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let mut column = iced::widget::Column::new()
            .spacing(24.0)
            .width(Length::Fill);

        let theme = context.theme;

        for section in &self.sections {
            match self.style {
                FormStyle::Grouped => {
                    let section_view = section.view(context);

                    // Manually construct the card effect to avoid WrapperView/transmute_copy issues
                    let card = iced::widget::container(
                        iced::widget::container(section_view)
                            .padding(16)
                            .width(Length::Fill)
                            .style(move |_| iced::widget::container::Style {
                                background: Some(theme.card_bg.into()),
                                border: iced::Border {
                                    radius: theme.radius.into(),
                                    color: theme.inner_border,
                                    width: 1.0,
                                },
                                text_color: Some(theme.text),
                                ..Default::default()
                            }),
                    )
                    .width(Length::Fill)
                    .style(move |_| iced::widget::container::Style {
                        border: iced::Border {
                            radius: theme.radius.into(),
                            color: theme.glass_border,
                            width: 1.0,
                        },
                        shadow: iced::Shadow {
                            color: theme.shadow_color,
                            offset: iced::Vector::new(
                                theme.shadow_offset[0],
                                theme.shadow_offset[1],
                            ),
                            blur_radius: theme.shadow_blur,
                        },
                        ..Default::default()
                    });

                    column = column.push(card);
                }
                FormStyle::Plain => {
                    column = column.push(section.view(context));
                }
            }
        }

        column.into()
    }
}
