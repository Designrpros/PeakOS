use peak_core::registry::AppId;
use iced::keyboard::Key;
use iced::{Point, Size};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct WindowState {
    #[allow(dead_code)]
    pub app_id: AppId,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub reality: peak_core::registry::ShellMode,
    pub desktop_idx: usize,
    pub is_sticky: bool, // Visible on all desktops
}

pub struct WindowManager {
    pub window_states: HashMap<AppId, WindowState>,
    pub z_order: Vec<AppId>,
    pub screen_size: Size,
    pub dragging: Option<(AppId, Point)>,
    #[allow(dead_code)]
    pub resizing: Option<(AppId, Point)>, // For future resizing support
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            window_states: HashMap::new(),
            z_order: Vec::new(),
            screen_size: Size::new(1920.0, 1080.0), // Default, updated on event
            dragging: None,
            resizing: None,
        }
    }

    pub fn ensure_window_state(
        &mut self,
        app_id: AppId,
        w: f32,
        h: f32,
        reality: peak_core::registry::ShellMode,
        desktop_idx: usize,
    ) {
        if !self.window_states.contains_key(&app_id) {
            let x = (self.screen_size.width - w) / 2.0;
            let y = (self.screen_size.height - h) / 2.0;
            self.window_states.insert(
                app_id,
                WindowState {
                    app_id,
                    x,
                    y,
                    width: w,
                    height: h,
                    reality,
                    desktop_idx,
                    is_sticky: false,
                },
            );
        }
        self.bring_to_front(app_id);
    }

    pub fn close_window(&mut self, app_id: AppId) {
        self.window_states.remove(&app_id);
        if let Some(pos) = self.z_order.iter().position(|&id| id == app_id) {
            self.z_order.remove(pos);
        }
    }

    pub fn bring_to_front(&mut self, app_id: AppId) {
        if !self.z_order.contains(&app_id) {
            self.z_order.push(app_id);
        } else {
            if let Some(pos) = self.z_order.iter().position(|&id| id == app_id) {
                self.z_order.remove(pos);
                self.z_order.push(app_id);
            }
        }
    }

    pub fn handle_snapping(&mut self, app_id: AppId, key: Key, dock_visible: bool) {
        if let Some(state) = self.window_states.get_mut(&app_id) {
            let menubar_h = 40.0;
            let dock_h = if dock_visible { 70.0 } else { 0.0 };

            let avail_w = self.screen_size.width;
            let avail_h = self.screen_size.height - menubar_h - dock_h;

            match key {
                Key::Character(c) if c == "1" => {
                    // Top Left
                    state.x = 0.0;
                    state.y = menubar_h;
                    state.width = avail_w / 2.0;
                    state.height = avail_h / 2.0;
                }
                Key::Character(c) if c == "2" => {
                    // Top Right
                    state.x = avail_w / 2.0;
                    state.y = menubar_h;
                    state.width = avail_w / 2.0;
                    state.height = avail_h / 2.0;
                }
                Key::Character(c) if c == "3" => {
                    // Bottom Left
                    state.x = 0.0;
                    state.y = menubar_h + (avail_h / 2.0);
                    state.width = avail_w / 2.0;
                    state.height = avail_h / 2.0;
                }
                Key::Character(c) if c == "4" => {
                    // Bottom Right
                    state.x = avail_w / 2.0;
                    state.y = menubar_h + (avail_h / 2.0);
                    state.width = avail_w / 2.0;
                    state.height = avail_h / 2.0;
                }
                Key::Named(iced::keyboard::key::Named::ArrowLeft) => {
                    // Cycle Left: 50% -> 33% -> 66%
                    if (state.width - avail_w / 2.0).abs() < 1.0 {
                        state.width = avail_w / 3.0;
                    } else if (state.width - avail_w / 3.0).abs() < 1.0 {
                        state.width = avail_w * 2.0 / 3.0;
                    } else {
                        state.width = avail_w / 2.0;
                    }
                    state.x = 0.0;
                    state.y = menubar_h;
                    state.height = avail_h;
                }
                Key::Named(iced::keyboard::key::Named::ArrowRight) => {
                    // Cycle Right: 50% -> 33% -> 66%
                    if (state.width - avail_w / 2.0).abs() < 1.0 {
                        state.width = avail_w / 3.0;
                    } else if (state.width - avail_w / 3.0).abs() < 1.0 {
                        state.width = avail_w * 2.0 / 3.0;
                    } else {
                        state.width = avail_w / 2.0;
                    }
                    state.x = avail_w - state.width;
                    state.y = menubar_h;
                    state.height = avail_h;
                }
                Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                    // Top Half
                    state.x = 0.0;
                    state.y = menubar_h;
                    state.width = avail_w;
                    state.height = avail_h / 2.0;
                }
                Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                    // Bottom Half
                    state.x = 0.0;
                    state.y = menubar_h + (avail_h / 2.0);
                    state.width = avail_w;
                    state.height = avail_h / 2.0;
                }
                Key::Named(iced::keyboard::key::Named::Enter) => {
                    // Maximum (Maximize preserving Dock if visible)
                    state.x = 0.0;
                    state.y = menubar_h;
                    state.width = avail_w;
                    state.height = avail_h;
                }
                Key::Character(c) if c.to_lowercase() == "c" => {
                    // Center
                    let w = (avail_w * 0.7).min(1000.0);
                    let h = (avail_h * 0.7).min(700.0);
                    state.width = w;
                    state.height = h;
                    state.x = (self.screen_size.width - w) / 2.0;
                    state.y = menubar_h + (avail_h - h) / 2.0;
                }
                Key::Character(f) if f.to_lowercase() == "f" => {
                    // Real Fullscreen (Behind everything)
                    state.x = 0.0;
                    state.y = 0.0;
                    state.width = self.screen_size.width;
                    state.height = self.screen_size.height;
                }
                _ => {}
            }
        }
    }
}
