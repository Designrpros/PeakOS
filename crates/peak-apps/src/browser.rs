#[cfg(target_os = "macos")]
use tao::platform::macos::WindowBuilderExtMacOS;
use tao::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::WindowBuilder,
};
use wry::{WebContext, WebViewBuilder};

pub struct BrowserApp;

use serde::{Deserialize, Serialize};
use std::io::BufRead;
use tao::event_loop::EventLoopBuilder;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BrowserCommand {
    #[serde(rename = "layout")]
    Layout {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    #[serde(rename = "navigate")]
    Navigate { url: String },
}

#[derive(Debug)]
pub enum UserEvent {
    BrowserCommand(BrowserCommand),
}

impl BrowserApp {
    /// Runs the browser window (must be called from main thread)
    pub fn run(url: &str) {
        // Use UserEvent for custom events
        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
        let proxy = event_loop.create_proxy();

        // Spawn stdin listener thread
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                if let Ok(line) = line {
                    if let Ok(cmd) = serde_json::from_str::<BrowserCommand>(&line) {
                        let _ = proxy.send_event(UserEvent::BrowserCommand(cmd));
                    } else {
                        eprintln!("Failed to parse browser command: {}", line);
                    }
                }
            }
        });

        #[cfg(target_os = "macos")]
        let window = WindowBuilder::new()
            .with_title("Netscape")
            .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 768.0))
            // On macOS, we make the titlebar transparent and content full-size for a "Peak" look
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .with_decorations(false) // Remove titlebar/borders for embedded look
            .with_has_shadow(false) // Remove drop shadow to blend with PeakOS frame
            .build(&event_loop)
            .unwrap();

        #[cfg(not(target_os = "macos"))]
        let window = {
            // Force Wayland and software rendering for VM compatibility
            std::env::set_var("GDK_BACKEND", "wayland");
            std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
            std::env::set_var("WEBKIT_DISABLE_ACCELERATED_2D_CANVAS", "1");
            std::env::set_var("WEBKIT_FORCE_SANDBOX", "0");
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

            WindowBuilder::new()
                .with_title("Netscape")
                .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 768.0))
                .with_decorations(false) // Embedded look on Linux too
                .build(&event_loop)
                .unwrap()
        };

        // Create a data directory for the browser to persist cookies and storage
        let data_dir = dirs::home_dir()
            .map(|h| h.join(".peak-os").join("browser-data"))
            .unwrap_or_else(|| std::path::PathBuf::from("./browser-data"));

        // Ensure it exists
        if let Err(e) = std::fs::create_dir_all(&data_dir) {
            eprintln!("Failed to create browser data directory: {}", e);
        }

        let mut web_context = WebContext::new(Some(data_dir));

        let webview = WebViewBuilder::with_web_context(&mut web_context)
            .with_url(url)
            .build(&window)
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::UserEvent(UserEvent::BrowserCommand(cmd)) => match cmd {
                    BrowserCommand::Layout {
                        x,
                        y,
                        width,
                        height,
                    } => {
                        window.set_outer_position(tao::dpi::LogicalPosition::new(x, y));
                        window.set_inner_size(tao::dpi::LogicalSize::new(width, height));
                    }
                    BrowserCommand::Navigate { url } => {
                        let _ = webview.load_url(&url);
                    }
                },
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}
