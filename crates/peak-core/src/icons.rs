use crate::registry::AppId;
use iced::widget::image::Handle as ImageHandle;
use iced::widget::svg::Handle as SvgHandle;

#[derive(Debug, Clone)]
pub enum AppIcon {
    Svg(SvgHandle),
    Image(ImageHandle),
}

/// Generic loader that reads from assets/icons/system/{category}/{name}.svg
/// and replaces "currentColor" with the provided hex color string.
pub fn load_system_svg(category: &str, name: &str, color: &str) -> SvgHandle {
    let rel_path = format!("icons/system/{}/{}.svg", category, name);
    let path = crate::utils::assets::get_asset_path(&rel_path);

    match std::fs::read_to_string(&path) {
        Ok(content) => {
            let colored_svg = content
                .replace("currentColor", color)
                .replace("stroke=\"white\"", &format!("stroke=\"{}\"", color))
                .replace("stroke=\"black\"", &format!("stroke=\"{}\"", color))
                .replace("fill=\"white\"", &format!("fill=\"{}\"", color))
                .replace("fill=\"black\"", &format!("fill=\"{}\"", color));
            SvgHandle::from_memory(colored_svg.into_bytes())
        }
        Err(_) => {
            // Fallback: A simple circle if the icon is missing
            SvgHandle::from_memory(format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><circle cx="12" cy="12" r="4" fill="{}" /></svg>"#,
                color
            ).into_bytes())
        }
    }
}

pub struct IconResolver;

impl IconResolver {
    /// Resolves an app icon using the priority layers:
    /// 1. PNG Logo (assets/icons/logos/)
    /// 2. Brand SVG (assets/icons/cache/simple-icons/)
    /// 3. System SVG (assets/icons/system/apps/)
    /// 4. Category Fallback (assets/icons/system/categories/)
    /// 5. Generic Fallback
    pub fn resolve_app_icon(id: AppId, color: &str) -> AppIcon {
        let metadata = id.metadata();

        // 1. Try PNG Logo
        if let Some(logo_name) = &metadata.icons.logo {
            let path = crate::utils::assets::get_asset_path(&format!("icons/logos/{}", logo_name));
            if path.exists() {
                return AppIcon::Image(ImageHandle::from_path(path));
            }
        }

        // 2. Try Brand SVG
        if let Some(brand) = &metadata.icons.brand_icon {
            let path = crate::utils::assets::get_asset_path(&format!(
                "icons/cache/simple-icons/{}.svg",
                brand
            ));
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    return AppIcon::Svg(SvgHandle::from_memory(content.into_bytes()));
                }
            }
        }

        // 3. Try System App SVG
        if let Some(sys_icon) = &metadata.icons.system_icon {
            let rel_path = format!("icons/system/apps/{}.svg", sys_icon);
            let path = crate::utils::assets::get_asset_path(&rel_path);
            if path.exists() {
                return AppIcon::Svg(load_system_svg("apps", sys_icon, color));
            }
        }

        // 4. Category Fallback
        let category_name = metadata.category.icon_name();
        let rel_path = format!("icons/system/categories/{}.svg", category_name);
        let path = crate::utils::assets::get_asset_path(&rel_path);
        if path.exists() {
            return AppIcon::Svg(load_system_svg("categories", category_name, color));
        }

        // 5. Ultimate Fallback (Generic System Icon)
        AppIcon::Svg(load_system_svg("system", "utilities", color))
    }
}

// --- Category Wrappers ---

pub fn get_app_icon(id: AppId, color: &str) -> SvgHandle {
    match IconResolver::resolve_app_icon(id, color) {
        AppIcon::Svg(h) => h,
        AppIcon::Image(_) => {
            // If we specifically need an SVG handle but got an image,
            // fallback to the system icon or category icon.
            let metadata = id.metadata();
            if let Some(sys_icon) = &metadata.icons.system_icon {
                load_system_svg("apps", sys_icon, color)
            } else {
                load_system_svg("categories", metadata.category.icon_name(), color)
            }
        }
    }
}

pub fn get_status_icon(name: &str, color: &str) -> SvgHandle {
    load_system_svg("status", name, color)
}

pub fn get_ui_icon(name: &str, color: &str) -> SvgHandle {
    load_system_svg("ui", name, color)
}

pub fn get_mode_icon(mode: crate::registry::ShellMode, color: &str) -> SvgHandle {
    use crate::registry::ShellMode;
    match mode {
        ShellMode::Desktop => load_system_svg("apps", "terminal", color), // monitor doesn't exist, using terminal
        ShellMode::Mobile => load_system_svg("status", "sun", color),     // mobile doesn't exist
        ShellMode::TV => load_system_svg("apps", "console", color),
        ShellMode::Console => load_system_svg("apps", "console", color),
        ShellMode::Kiosk => load_system_svg("ui", "logo", color),
        ShellMode::Fireplace => load_system_svg("ui", "sparkles", color),
        ShellMode::Auto => load_system_svg("ui", "trigger", color),
        ShellMode::Robot => load_system_svg("avatars", "robot", color),
        ShellMode::Server => load_system_svg("apps", "cpu", color),
        ShellMode::SmartHome => load_system_svg("ui", "apps", color),
    }
}

pub fn get_avatar_handle(name: &str, color: &str) -> SvgHandle {
    load_system_svg("avatars", name, color)
}

pub const AVATAR_OPTIONS: [&str; 5] = ["robot", "alien", "ghost", "peak", "smile"];
