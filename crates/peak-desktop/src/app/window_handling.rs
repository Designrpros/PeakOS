// Window management helpers

use super::PeakNative;

impl PeakNative {
    pub(crate) fn ensure_window_state(&mut self, app_id: peak_core::registry::AppId, w: f32, h: f32) {
        self.window_manager.ensure_window_state(app_id, w, h, self.mode, self.current_desktop);
    }

    pub(crate) fn close_window(&mut self, app_id: peak_core::registry::AppId) {
        self.window_manager.close_window(app_id);
    }

    pub(crate) fn handle_snapping(&mut self, app_id: peak_core::registry::AppId, key: iced::keyboard::Key) {
        self.window_manager.handle_snapping(app_id, key, self.dock_visible);
    }
}
