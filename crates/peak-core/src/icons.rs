use iced::widget::svg::Handle;

/// Generic loader that reads from assets/icons/system/{category}/{name}.svg
/// and replaces "currentColor" with the provided hex color string.
pub fn load_system_svg(category: &str, name: &str, color: &str) -> Handle {
    let rel_path = format!("icons/system/{}/{}.svg", category, name);
    let path = crate::utils::assets::get_asset_path(&rel_path);

    match std::fs::read_to_string(&path) {
        Ok(content) => {
            let colored_svg = content.replace("currentColor", color);
            Handle::from_memory(colored_svg.into_bytes())
        }
        Err(_) => {
            // Fallback: A simple circle if the icon is missing
            Handle::from_memory(format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><circle cx="12" cy="12" r="4" fill="{}" /></svg>"#,
                color
            ).into_bytes())
        }
    }
}

// --- Category Wrappers ---

pub fn get_app_icon(id: crate::registry::AppId, color: &str) -> Handle {
    use crate::registry::AppId;
    let name = match id {
        AppId::Terminal => "terminal",
        AppId::Browser => "browser",
        AppId::Turntable => "cassette",
        AppId::Library => "console",
        AppId::FileManager => "folder",
        AppId::Settings => "settings",
        AppId::Store => "store",
        AppId::Cortex => "cpu",
        AppId::AppGrid => "library",

        AppId::Antigravity => "sparkles",
        AppId::Editor => "editor",
        AppId::Desktop => "monitor",
        AppId::Spotify => "spotify",
    };
    load_system_svg("apps", name, color)
}

pub fn get_status_icon(name: &str, color: &str) -> Handle {
    load_system_svg("status", name, color)
}

pub fn get_ui_icon(name: &str, color: &str) -> Handle {
    load_system_svg("ui", name, color)
}

pub fn get_avatar_handle(name: &str, color: &str) -> Handle {
    load_system_svg("avatars", name, color)
}

pub const AVATAR_OPTIONS: [&str; 5] = ["robot", "alien", "ghost", "peak", "smile"];
