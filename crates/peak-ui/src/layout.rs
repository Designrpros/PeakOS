use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use iced::Length;

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
            width: Length::Shrink,
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

    pub fn extend<I, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: View<Message, B> + 'static,
    {
        for child in iter {
            self.children.push(Box::new(child));
        }
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

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let children = self.children.iter().map(|c| c.describe(context)).collect();
        crate::core::SemanticNode {
            role: "vstack".to_string(),
            label: None,
            content: None,
            children,
        }
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
            width: Length::Shrink,
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

    pub fn extend<I, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: View<Message, B> + 'static,
    {
        for child in iter {
            self.children.push(Box::new(child));
        }
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

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let children = self.children.iter().map(|c| c.describe(context)).collect();
        crate::core::SemanticNode {
            role: "hstack".to_string(),
            label: None,
            content: None,
            children,
        }
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
            width: Length::Shrink,
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

impl<Message: 'static, B: Backend> View<Message, B> for ZStack<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let child_views = self.children.iter().map(|c| c.view(context)).collect();
        B::zstack(child_views, self.width, self.height)
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let children = self.children.iter().map(|c| c.describe(context)).collect();
        crate::core::SemanticNode {
            role: "zstack".to_string(),
            label: None,
            content: None,
            children,
        }
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

impl<Message: 'static, B: Backend> View<Message, B> for ResponsiveGrid<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
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

        let child_views = self.children.iter().map(|c| c.view(context)).collect();
        B::grid(child_views, items_per_row, self.spacing)
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
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

        let children = self.children.iter().map(|c| c.describe(context)).collect();
        crate::core::SemanticNode {
            role: "grid".to_string(),
            label: Some(format!("responsive_columns: {}", items_per_row)),
            content: None,
            children,
        }
    }
}
