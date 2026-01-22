use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::widget::stack;
use iced::{Length, Renderer, Theme};

pub struct VStack<Message: 'static, B: Backend = IcedBackend> {
    children: Vec<Box<dyn View<Message, B>>>,
    spacing: f32,
    padding: iced::Padding,
    width: Length,
    height: Length,
    align_x: iced::Alignment,
}

impl<Message: 'static> VStack<Message, IcedBackend> {
    pub fn new() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static> VStack<Message, TermBackend> {
    pub fn new_tui() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static, B: Backend> VStack<Message, B> {
    pub fn new_generic() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
            padding: iced::Padding::from(0.0),
            width: Length::Fill,
            height: Length::Shrink,
            align_x: iced::Alignment::Start,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn align_x(mut self, align: iced::Alignment) -> Self {
        self.align_x = align;
        self
    }

    pub fn push(mut self, view: impl View<Message, B> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for VStack<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let child_views = self.children.iter().map(|c| c.view(context)).collect();
        B::vstack(
            child_views,
            self.spacing,
            self.padding,
            self.width,
            self.height,
            self.align_x,
            context.theme.scaling,
        )
    }
}

pub struct HStack<Message: 'static, B: Backend = IcedBackend> {
    children: Vec<Box<dyn View<Message, B>>>,
    spacing: f32,
    padding: iced::Padding,
    width: Length,
    height: Length,
    align_y: iced::Alignment,
}

impl<Message: 'static> HStack<Message, IcedBackend> {
    pub fn new() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static> HStack<Message, TermBackend> {
    pub fn new_tui() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static, B: Backend> HStack<Message, B> {
    pub fn new_generic() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
            padding: iced::Padding::from(0.0),
            width: Length::Fill,
            height: Length::Shrink,
            align_y: iced::Alignment::Start,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn align_y(mut self, align: iced::Alignment) -> Self {
        self.align_y = align;
        self
    }

    pub fn push(mut self, view: impl View<Message, B> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }
}

impl<Message: 'static, B: Backend> View<Message, B> for HStack<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let child_views = self.children.iter().map(|c| c.view(context)).collect();
        B::hstack(
            child_views,
            self.spacing,
            self.padding,
            self.width,
            self.height,
            self.align_y,
            context.theme.scaling,
        )
    }
}

pub struct ZStack<Message: 'static, B: Backend = IcedBackend> {
    children: Vec<Box<dyn View<Message, B>>>,
    width: Length,
    height: Length,
}

impl<Message: 'static> ZStack<Message, IcedBackend> {
    pub fn new() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static> ZStack<Message, TermBackend> {
    pub fn new_tui() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static, B: Backend> ZStack<Message, B> {
    pub fn new_generic() -> Self {
        Self {
            children: Vec::new(),
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn push(mut self, view: impl View<Message, B> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }
}

impl<Message: 'static> View<Message, IcedBackend> for ZStack<Message, IcedBackend> {
    fn view(&self, context: &Context) -> iced::Element<'static, Message, Theme, Renderer> {
        let mut s = stack(Vec::new()).width(self.width).height(self.height);

        for child in &self.children {
            s = s.push(child.view(context));
        }

        s.into()
    }
}

impl<Message: 'static> View<Message, TermBackend> for ZStack<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        self.children
            .iter()
            .map(|c| c.view(context))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub struct ResponsiveGrid<Message: 'static, B: Backend = IcedBackend> {
    children: Vec<Box<dyn View<Message, B>>>,
    spacing: f32,
}

impl<Message: 'static> ResponsiveGrid<Message, IcedBackend> {
    pub fn new() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static> ResponsiveGrid<Message, TermBackend> {
    pub fn new_tui() -> Self {
        Self::new_generic()
    }
}

impl<Message: 'static, B: Backend> ResponsiveGrid<Message, B> {
    pub fn new_generic() -> Self {
        Self {
            children: Vec::new(),
            spacing: 20.0,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn push(mut self, view: impl View<Message, B> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }
}

impl<Message: 'static> View<Message, IcedBackend> for ResponsiveGrid<Message, IcedBackend> {
    fn view(&self, context: &Context) -> iced::Element<'static, Message, Theme, Renderer> {
        use iced::widget::{column, container, row};
        let children: Vec<_> = self.children.iter().map(|c| c.view(context)).collect();

        let items_per_row = if context.size.width < 600.0 {
            1
        } else if context.size.width < 900.0 {
            2
        } else if context.size.width < 1200.0 {
            3
        } else if context.size.width < 1600.0 {
            4
        } else {
            5
        };

        let scale = context.theme.scaling;
        let scaled_spacing = self.spacing * scale;

        let mut rows = Vec::new();
        let mut current_row = Vec::new();
        let mut count = 0;

        for child in children {
            current_row.push(container(child).width(Length::FillPortion(1)).into());
            count += 1;

            if count == items_per_row {
                rows.push(
                    row(current_row)
                        .spacing(scaled_spacing)
                        .width(Length::Fill)
                        .into(),
                );
                current_row = Vec::new();
                count = 0;
            }
        }

        if !current_row.is_empty() {
            for _ in count..items_per_row {
                current_row.push(
                    container(iced::widget::Space::new(
                        Length::FillPortion(1),
                        Length::Shrink,
                    ))
                    .into(),
                );
            }
            rows.push(
                row(current_row)
                    .spacing(scaled_spacing)
                    .width(Length::Fill)
                    .into(),
            );
        }

        container(column(rows).spacing(scaled_spacing).width(Length::Fill))
            .width(Length::Fill)
            .into()
    }
}

impl<Message: 'static> View<Message, TermBackend> for ResponsiveGrid<Message, TermBackend> {
    fn view(&self, context: &Context) -> String {
        self.children
            .iter()
            .map(|c| c.view(context))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
