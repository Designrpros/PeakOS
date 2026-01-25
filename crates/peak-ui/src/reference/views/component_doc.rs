use crate::atoms::Text;
use crate::core::{Backend, Context, IcedBackend, View};
use crate::layout::VStack;
use crate::views::CodeBlock;
use iced::{Length, Padding};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct ComponentDoc<Message: 'static, B: Backend = IcedBackend> {
    title: String,
    description: String,
    code_snippet: String,
    preview: Arc<dyn View<Message, B>>,
    _phantom: PhantomData<B>,
}

impl<Message: 'static, B: Backend> ComponentDoc<Message, B> {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        code_snippet: impl Into<String>,
        preview: Arc<dyn View<Message, B>>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            code_snippet: code_snippet.into(),
            preview,
            _phantom: PhantomData,
        }
    }
}

impl<Message: Clone + 'static> View<Message, IcedBackend> for ComponentDoc<Message, IcedBackend> {
    fn view(
        &self,
        context: &Context,
    ) -> iced::Element<'static, Message, iced::Theme, iced::Renderer> {
        let theme = context.theme;

        // 1. Header with Title and Description
        // IMPORTANT: We set width(Length::Fill) on the VStack to ensure text can expand
        let header = VStack::<Message, IcedBackend>::new_generic()
            .spacing(12.0)
            .width(Length::Fill)
            .push(
                Text::<IcedBackend>::new(self.title.clone())
                    .large_title()
                    .bold()
                    .color(theme.colors.text_primary),
            )
            .push(
                Text::<IcedBackend>::new(self.description.clone())
                    .body()
                    .color(theme.colors.text_secondary)
                    .width(Length::Fill),
            );

        // 2. Playground / Preview Area
        let preview_area = crate::containers::Section::<Message, IcedBackend>::new_generic(
            "Preview",
            crate::containers::Card::<Message, IcedBackend>::new_generic(
                crate::core::ProxyView::new({
                    let preview = self.preview.clone();
                    move |ctx| {
                        iced::widget::container(preview.view(ctx))
                            .width(Length::Fill)
                            .center_x(Length::Fill)
                            .into()
                    }
                }),
            )
            .width(Length::Fill),
        )
        .width(Length::Fill);

        // 3. Code Block with Copy (Using the Shared CodeBlock component)
        let code_snippet = self.code_snippet.clone();

        let code_area = crate::containers::Section::<Message, IcedBackend>::new_generic(
            "Usage",
            crate::core::ProxyView::new(move |ctx| {
                CodeBlock::<Message>::rust(code_snippet.clone()).view(ctx)
            }),
        )
        .width(Length::Fill);

        // Assemble
        VStack::<Message, IcedBackend>::new_generic()
            .spacing(40.0)
            .padding(Padding {
                top: context.safe_area.top.max(48.0),
                right: if context.is_slim() { 24.0 } else { 48.0 },
                bottom: context.safe_area.bottom.max(48.0),
                left: if context.is_slim() { 24.0 } else { 48.0 },
            })
            .width(Length::Fill)
            .push(header)
            .push(preview_area)
            .push(code_area)
            .view(context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode {
            role: "component_doc".to_string(),
            label: Some(self.title.clone()),
            content: Some(self.description.clone()),
            children: Vec::new(),
        }
    }
}
