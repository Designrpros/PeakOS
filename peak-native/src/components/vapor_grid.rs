#![allow(dead_code)]
use iced::mouse::Cursor;
use iced::widget::canvas::{self, Cache, Canvas, Geometry, Path, Stroke};
use iced::{Color, Element, Length, Point, Rectangle, Size, Theme};
use std::time::Instant;

pub struct VaporGrid {
    start_time: Instant,
    cache: Cache,
}

impl VaporGrid {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            cache: Cache::default(),
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

impl<Message> canvas::Program<Message> for VaporGrid {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let width = frame.width();
            let height = frame.height();

            // 0. Background Void
            let bg = Path::rectangle(Point::ORIGIN, Size::new(width, height));
            frame.fill(&bg, Color::from_rgb8(20, 10, 40));

            let center_x = width / 2.0;
            let horizon_y = height * 0.4; // Horizon line is higher up

            // 1. The Horizon Glow (Sun)
            let sun_center = Point::new(center_x, horizon_y);
            let sun_radius = 80.0;
            let sun_path = Path::circle(sun_center, sun_radius);
            frame.fill(&sun_path, Color::from_rgb8(255, 100, 150)); // Retro Pink Sun

            // 2. The Grid Lines (Perspective)
            let time_offset = self.start_time.elapsed().as_secs_f32() * 50.0;
            let grid_color = Color::from_rgb8(0, 255, 255); // Cyan Neon

            // Vertical Lines (Fan out from vanishing point)
            for i in -10..=10 {
                let x_offset = i as f32 * 100.0;
                let bottom_x = center_x + x_offset * 3.0; // Wider at bottom

                let line = Path::new(|p| {
                    p.move_to(Point::new(center_x + x_offset * 0.1, horizon_y)); // Converge at horizon
                    p.line_to(Point::new(bottom_x, height));
                });
                frame.stroke(
                    &line,
                    Stroke::default().with_color(grid_color).with_width(2.0),
                );
            }

            // Horizontal Lines (Move towards viewer)
            // We use modulo to make them loop endlessly
            let spacing = 40.0;
            for i in 0..20 {
                let depth = (i as f32 * spacing + time_offset) % (height - horizon_y);
                // Non-linear spacing for perspective trick (futher lines are closer together)
                let y_pos = horizon_y + (depth * depth / 400.0);

                if y_pos > height {
                    continue;
                }

                let line = Path::new(|p| {
                    p.move_to(Point::new(0.0, y_pos));
                    p.line_to(Point::new(width, y_pos));
                });

                // Fade out lines near horizon
                let alpha = (y_pos - horizon_y) / (height - horizon_y);
                let fade_color = Color {
                    a: alpha,
                    ..grid_color
                };

                frame.stroke(
                    &line,
                    Stroke::default().with_color(fade_color).with_width(2.0),
                );
            }
        });

        vec![geometry]
    }
}
