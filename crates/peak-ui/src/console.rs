use crate::atoms::Text;
use crate::core::{Context, View};
use crate::scroll_view::ScrollView;
use iced::{Element, Renderer, Theme};

pub struct Console<Message> {
    content: String,
    input: Option<Box<dyn View<Message>>>,
}

impl<Message> Console<Message> {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            input: None,
        }
    }

    pub fn input(mut self, input: impl View<Message> + 'static) -> Self {
        self.input = Some(Box::new(input));
        self
    }
}

impl<Message: 'static> View<Message> for Console<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        let output = Text::new(&self.content)
            .font(iced::Font::MONOSPACE)
            .size(12.0)
            .color(theme.colors.text_primary);

        // Use standard Column for internal layout to avoid recursive view calls if needed,
        // but VStack is preferred for PeakUI context.
        let mut col = iced::widget::Column::new()
            .push(ScrollView::new(output).view(context))
            .spacing(8.0 * context.theme.scaling);

        if let Some(input) = &self.input {
            col = col.push(input.view(context));
        }

        col.into()
    }
}
