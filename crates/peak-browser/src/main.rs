use clap::Parser;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Initial URL to open
    #[arg(short, long, default_value = "https://peakos.org")]
    url: String,

    /// Initial Window X Position
    #[arg(long)]
    x: Option<f64>,

    /// Initial Window Y Position
    #[arg(long)]
    y: Option<f64>,

    /// Initial Window Width
    #[arg(long)]
    width: Option<f64>,

    /// Initial Window Height
    #[arg(long)]
    height: Option<f64>,
}

fn main() -> wry::Result<()> {
    let args = Args::parse();
    let event_loop = EventLoop::new();

    let mut window_builder = WindowBuilder::new()
        .with_title("Peak Browser")
        .with_decorations(false) // Frameless for OS integration
        .with_transparent(true);

    if let (Some(width), Some(height)) = (args.width, args.height) {
        window_builder = window_builder.with_inner_size(tao::dpi::LogicalSize::new(width, height));
    }

    if let (Some(x), Some(y)) = (args.x, args.y) {
        window_builder = window_builder.with_position(tao::dpi::LogicalPosition::new(x, y));
    }

    let window = window_builder.build(&event_loop).unwrap();

    #[cfg(target_os = "macos")]
    {
        use tao::platform::macos::WindowExtMacOS;
        // Make it partially transparent/vibrant if possible, mimicking the glassmorphism
        let _ = window.set_titlebar_transparent(true);
    }

    let _webview = WebViewBuilder::new()
        .with_url(&args.url)
        .with_transparent(true)
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
