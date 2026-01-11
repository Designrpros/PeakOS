#![allow(dead_code)]
use iced::widget::canvas::{self, Cache, Canvas, Geometry, Path, Stroke};
use iced::{mouse, Color, Element, Length, Point, Rectangle, Theme};

pub struct Graph {
    data: Vec<f32>,
    cache: Cache,
    color: Color,
    label: String,
}

impl Graph {
    pub fn new(data: Vec<f32>, color: Color, label: String) -> Self {
        Self {
            data,
            cache: Cache::default(),
            color,
            label,
        }
    }

    pub fn view<'a, Message>(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<Message> canvas::Program<Message> for Graph {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // 1. Draw Background Grid
            let grid_color = Color {
                a: 0.1,
                ..self.color
            };
            let path = Path::new(|p| {
                // Horizontal lines
                for i in 0..5 {
                    let y = (frame.height() / 4.0) * i as f32;
                    p.move_to(Point::new(0.0, y));
                    p.line_to(Point::new(frame.width(), y));
                }
            });
            frame.stroke(
                &path,
                Stroke::default().with_color(grid_color).with_width(1.0),
            );

            // 2. Draw Data Line
            if self.data.len() > 1 {
                let step_x = frame.width() / (self.data.len() - 1) as f32;

                let line_path = Path::new(|p| {
                    for (i, &val) in self.data.iter().enumerate() {
                        // Normalize 0.0-100.0 to height
                        let x = i as f32 * step_x;
                        let y = frame.height() - (val.clamp(0.0, 100.0) / 100.0 * frame.height());

                        if i == 0 {
                            p.move_to(Point::new(x, y));
                        } else {
                            p.line_to(Point::new(x, y));
                        }
                    }
                });

                // Neon Glow Stroke
                frame.stroke(
                    &line_path,
                    Stroke::default().with_color(self.color).with_width(2.0),
                );
            }

            // 3. Draw Label
            frame.fill_text(canvas::Text {
                content: self.label.clone(),
                position: Point::new(10.0, 10.0),
                color: self.color,
                size: 14.0.into(),
                ..canvas::Text::default()
            });
        });

        vec![geometry]
    }
}
