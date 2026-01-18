use iced::mouse;
use iced::widget::canvas::{self, Cache, Canvas, Geometry, Path, Stroke};
use iced::{Color, Element, Length, Point, Rectangle, Vector};
use peak_core::theme::Theme;
use rand::Rng;
use std::time::Instant;

const NODE_COUNT: usize = 80;
const CONNECTION_DISTANCE: f32 = 140.0;
const SPEED: f32 = 5.0; // Slower, more elegant

struct Node {
    position: Point,
    velocity: Vector,
}

pub struct VectorBackground {
    nodes: Vec<Node>,
    cache: Cache,
    start_time: Instant,
}

impl VectorBackground {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let nodes = (0..NODE_COUNT)
            .map(|_| Node {
                position: Point::new(
                    rng.gen_range(0.0..1920.0), // Initial assumption, will wrap
                    rng.gen_range(0.0..1080.0),
                ),
                velocity: Vector::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
            })
            .collect();

        Self {
            nodes,
            cache: Cache::default(),
            start_time: Instant::now(),
        }
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn view<'a, Message>(&'a self, theme: Theme) -> Element<'a, Message>
    where
        Message: 'a,
    {
        Canvas::new(VectorLayer { bg: self, theme })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

struct VectorLayer<'a> {
    bg: &'a VectorBackground,
    theme: Theme,
}

impl<'a, Message> canvas::Program<Message> for VectorLayer<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.bg.cache.draw(renderer, bounds.size(), |frame| {
            let width = frame.width();
            let height = frame.height();

            let palette = self.theme.palette();
            let bg_color = palette.background;
            let line_color = palette.line_color;
            let node_color = palette.node_color;

            // Background
            let bg_path = Path::rectangle(Point::ORIGIN, bounds.size());
            frame.fill(&bg_path, bg_color);

            let elapsed = self.bg.start_time.elapsed().as_secs_f32();

            // Compute current positions
            let current_nodes: Vec<Point> = self
                .bg
                .nodes
                .iter()
                .map(|n| {
                    let dx = n.velocity.x * SPEED * elapsed;
                    let dy = n.velocity.y * SPEED * elapsed;

                    let mut x = (n.position.x + dx) % width;
                    let mut y = (n.position.y + dy) % height;

                    if x < 0.0 {
                        x += width;
                    }
                    if y < 0.0 {
                        y += height;
                    }

                    Point::new(x, y)
                })
                .collect();

            // Draw Connections
            for (i, &p1) in current_nodes.iter().enumerate() {
                let circle = Path::circle(p1, 1.5);
                frame.fill(&circle, node_color);

                for &p2 in current_nodes.iter().skip(i + 1) {
                    let dist = p1.distance(p2);
                    if dist < CONNECTION_DISTANCE {
                        let alpha = 1.0 - (dist / CONNECTION_DISTANCE);
                        let path = Path::new(|p| {
                            p.move_to(p1);
                            p.line_to(p2);
                        });

                        frame.stroke(
                            &path,
                            Stroke::default()
                                .with_color(Color {
                                    a: alpha * line_color.a,
                                    ..line_color
                                })
                                .with_width(0.8),
                        );
                    }
                }
            }
        });

        vec![geometry]
    }
}
