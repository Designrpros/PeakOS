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
    pub fn run(url: &str, initial_layout: Option<BrowserCommand>) {
        // Use UserEvent for custom events
        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
        let proxy = event_loop.create_proxy();

        // Spawn stdin listener thread
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            // Use buffered reader for better performance
            let reader = std::io::BufReader::new(stdin);
            for line in reader.lines().map_while(Result::ok) {
                if let Ok(cmd) = serde_json::from_str::<BrowserCommand>(&line) {
                    let _ = proxy.send_event(UserEvent::BrowserCommand(cmd));
                }
            }
        });

        #[cfg(target_os = "macos")]
        let window = {
            let mut builder = WindowBuilder::new()
                .with_title("Netscape")
                .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 768.0))
                .with_titlebar_transparent(true)
                .with_fullsize_content_view(true)
                .with_decorations(false)
                .with_has_shadow(false)
                .with_transparent(true);

            if let Some(BrowserCommand::Layout {
                x,
                y,
                width,
                height,
            }) = initial_layout
            {
                builder = builder
                    .with_position(tao::dpi::LogicalPosition::new(x, y))
                    .with_inner_size(tao::dpi::LogicalSize::new(width, height));
            }

            builder.build(&event_loop).unwrap()
        };

        #[cfg(not(target_os = "macos"))]
        let window = {
            // Force Wayland and software rendering for VM compatibility
            std::env::set_var("GDK_BACKEND", "wayland");
            std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

            let mut builder = WindowBuilder::new()
                .with_title("Netscape")
                .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 768.0))
                .with_decorations(false)
                .with_transparent(true);

            if let Some(BrowserCommand::Layout {
                x,
                y,
                width,
                height,
            }) = initial_layout
            {
                builder = builder
                    .with_position(tao::dpi::LogicalPosition::new(x, y))
                    .with_inner_size(tao::dpi::LogicalSize::new(width, height));
            }

            builder.build(&event_loop).unwrap()
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
