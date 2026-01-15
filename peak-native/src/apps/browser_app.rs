// In-process browser using wry WebView
// This replaces the separate process approach

use std::sync::{Arc, Mutex};
use wry::{WebContext, WebView, WebViewBuilder};

pub struct BrowserApp {
    webview: Option<Arc<Mutex<WebView>>>,
    url: String,
    is_visible: bool,
}

#[derive(Debug, Clone)]
pub enum BrowserMessage {
    Navigate(String),
    Close,
}

impl BrowserApp {
    pub fn new() -> Self {
        Self {
            webview: None,
            url: String::from("about:blank"),
            is_visible: false,
        }
    }

    pub fn open(&mut self, url: String) {
        self.url = url;
        self.is_visible = true;
        // Webview will be created when we have window handle
    }

    pub fn close(&mut self) {
        self.is_visible = false;
        self.webview = None;
    }

    pub fn navigate(&mut self, url: String) {
        self.url = url.clone();
        if let Some(webview) = &self.webview {
            let webview = webview.lock().unwrap();
            let _ = webview.load_url(&url);
        }
    }

    pub fn update(&mut self, message: BrowserMessage) {
        match message {
            BrowserMessage::Navigate(url) => self.navigate(url),
            BrowserMessage::Close => self.close(),
        }
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn current_url(&self) -> &str {
        &self.url
    }
}
