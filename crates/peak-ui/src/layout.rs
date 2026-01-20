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
        let scale = context.theme.scaling;
        let scaled_spacing = self.spacing * scale;
        let p = self.padding;
        let scaled_padding = iced::Padding {
            top: p.top * scale,
            right: p.right * scale,
            bottom: p.bottom * scale,
            left: p.left * scale,
        };

        container(
            column(children)
                .spacing(scaled_spacing)
                .align_x(self.align_x),
        )
        .padding(scaled_padding)
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
        let scale = context.theme.scaling;
        let scaled_spacing = self.spacing * scale;
        let p = self.padding;
        let scaled_padding = iced::Padding {
            top: p.top * scale,
            right: p.right * scale,
            bottom: p.bottom * scale,
            left: p.left * scale,
        };

        container(row(children).spacing(scaled_spacing).align_y(self.align_y))
            .padding(scaled_padding)
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

        // Calculate columns based on screen width
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

        // Create rows with equal-width columns
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

        // Handle last row with fillers
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
