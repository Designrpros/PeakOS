#[cfg(target_os = "macos")]
use tao::platform::macos::WindowBuilderExtMacOS;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

pub struct BrowserApp;

impl BrowserApp {
    /// Spawns a new browser process
    pub fn open(url: &str) {
        let current_exe = std::env::current_exe().unwrap_or_else(|_| "peak-native".into());

        let mut child = std::process::Command::new(current_exe)
            .arg("--browser")
            .arg(url)
            .spawn()
            .expect("Failed to spawn browser process");

        // We don't wait for the child process to exit
        std::thread::spawn(move || {
            let _ = child.wait();
        });
    }

    /// Runs the browser window (must be called from main thread)
    pub fn run(url: &str) {
        let event_loop = EventLoop::new();

        #[cfg(target_os = "macos")]
        let window = WindowBuilder::new()
            .with_title("Netscape")
            .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 768.0))
            // On macOS, we make the titlebar transparent and content full-size for a "Peak" look
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .build(&event_loop)
            .unwrap();

        #[cfg(not(target_os = "macos"))]
        let window = WindowBuilder::new()
            .with_title("Netscape")
            .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 768.0))
            .build(&event_loop)
            .unwrap();

        let _webview = WebViewBuilder::new().with_url(url).build(&window).unwrap();

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
}
