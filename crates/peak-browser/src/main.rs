use clap::Parser;

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

fn main() {
    let args = Args::parse();

    let initial_layout = if let (Some(x), Some(y), Some(width), Some(height)) =
        (args.x, args.y, args.width, args.height)
    {
        Some(peak_apps::browser::BrowserCommand::Layout {
            x,
            y,
            width,
            height,
        })
    } else {
        None
    };

    peak_apps::browser::BrowserApp::run(&args.url, initial_layout);
}
