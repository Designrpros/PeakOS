use crate::core::{Context, View};
use iced::widget::{column, container, row, stack};

use iced::{Element, Length, Renderer, Theme};

pub struct VStack<Message> {
    children: Vec<Box<dyn View<Message>>>,
    spacing: f32,
    padding: iced::Padding,
    width: Length,
    height: Length,
    align_x: iced::Alignment,
}

impl<Message> VStack<Message> {
    pub fn new() -> Self {
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

    pub fn push(mut self, view: impl View<Message> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }
}

impl<Message: 'static> View<Message> for VStack<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let children: Vec<_> = self.children.iter().map(|c| c.view(context)).collect();

        container(column(children).spacing(self.spacing).align_x(self.align_x))
            .padding(self.padding)
            .width(self.width)
            .height(self.height)
            .into()
    }
}

pub struct HStack<Message> {
    children: Vec<Box<dyn View<Message>>>,
    spacing: f32,
    padding: iced::Padding,
    width: Length,
    height: Length,
    align_y: iced::Alignment,
}

impl<Message> HStack<Message> {
    pub fn new() -> Self {
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

    pub fn push(mut self, view: impl View<Message> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }
}

impl<Message: 'static> View<Message> for HStack<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let children: Vec<_> = self.children.iter().map(|c| c.view(context)).collect();

        container(row(children).spacing(self.spacing).align_y(self.align_y))
            .padding(self.padding)
            .width(self.width)
            .height(self.height)
            .into()
    }
}

pub struct ZStack<Message> {
    children: Vec<Box<dyn View<Message>>>,
    width: Length,
    height: Length,
}

impl<Message> ZStack<Message> {
    pub fn new() -> Self {
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

    pub fn push(mut self, view: impl View<Message> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }
}

impl<Message: 'static> View<Message> for ZStack<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let mut s = stack(Vec::new()).width(self.width).height(self.height);

        for child in &self.children {
            s = s.push(child.view(context));
        }

        s.into()
    }
}

pub struct ResponsiveGrid<Message> {
    children: Vec<Box<dyn View<Message>>>,
    spacing: f32,
}

impl<Message> ResponsiveGrid<Message> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 20.0,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn push(mut self, view: impl View<Message> + 'static) -> Self {
        self.children.push(Box::new(view));
        self
    }
}

impl<Message: 'static> View<Message> for ResponsiveGrid<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let children: Vec<_> = self.children.iter().map(|c| c.view(context)).collect();
        if context.is_slim() {
            container(iced::widget::column(children).spacing(self.spacing))
                .width(Length::Fill)
                .into()
        } else {
            container(iced::widget::row(children).spacing(self.spacing))
                .width(Length::Fill)
                .into()
        }
    }
}
