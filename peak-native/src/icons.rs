use iced::widget::svg::Handle;

pub fn wifi_full(color: &str) -> Handle {
    let svg = format!(
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M5 12.55a11 11 0 0 1 14.08 0"></path>
            <path d="M1.42 9a16 16 0 0 1 21.16 0"></path>
            <path d="M8.53 16.11a6 6 0 0 1 6.95 0"></path>
            <line x1="12" y1="20" x2="12.01" y2="20"></line>
        </svg>
    "#,
        color
    );
    Handle::from_memory(svg.into_bytes())
}

pub fn sun() -> Handle {
    Handle::from_memory(br#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/>
            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
            <line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/>
            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
        </svg>
    "#.to_vec())
}

pub fn moon() -> Handle {
    Handle::from_memory(br#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
        </svg>
    "#.to_vec())
}

// Avatars (Neutral colors for generic usage, can be tinted by container if SVG allows, but hardcoded for now)
// We'll use a standard stroke color that works on both dark/light or rely on opacity.
// Actually, let's use "currentColor" and let the parent widget set the color? Iced SVG doesn't always support that easily.
// I'll stick to a safe neutral gray or specific colors for avatars.

pub fn avatar_robot(color: &str) -> Handle {
    let svg = format!(
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="11" width="18" height="10" rx="2"/><circle cx="12" cy="5" r="2"/><path d="M12 7v4"/><line x1="8" y1="16" x2="8" y2="16"/><line x1="16" y1="16" x2="16" y2="16"/>
        </svg>
    "#,
        color
    );
    Handle::from_memory(svg.into_bytes())
}

pub fn avatar_alien(color: &str) -> Handle {
    let svg = format!(
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 2c-5.5 0-10 4.5-10 10s4.5 10 10 10 10-4.5 10-10-4.5-10-10-10z"/><line x1="15" y1="9" x2="15.01" y2="9"/><line x1="9" y1="9" x2="9.01" y2="9"/><path d="M15 15a3 3 0 0 1-6 0"/>
        </svg>
    "#,
        color
    );
    Handle::from_memory(svg.into_bytes())
}

pub fn avatar_ghost(color: &str) -> Handle {
    let svg = format!(
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M9 22v-5 h6 v5 h-6 M9 22H5a2 2 0 0 1-2-2V9c0-5 3.5-9 9-9s9 4 9 9v11a2 2 0 0 1-2 2h-4"/><path d="M9 9h.01"/><path d="M15 9h.01"/>
        </svg>
    "#,
        color
    );
    Handle::from_memory(svg.into_bytes())
}

pub fn avatar_peak(color: &str) -> Handle {
    // A simplified mountain logo
    let svg = format!(
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M2 20h20"/><path d="M12 4l-10 16"/><path d="M12 4l10 16"/><path d="M17 14l-5-6-3 4"/>
        </svg>
    "#,
        color
    );
    Handle::from_memory(svg.into_bytes())
}

pub fn avatar_smile(color: &str) -> Handle {
    let svg = format!(
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="{}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"/><path d="M8 14s1.5 2 4 2 4-2 4-2"/><line x1="9" y1="9" x2="9.01" y2="9"/><line x1="15" y1="9" x2="15.01" y2="9"/>
        </svg>
    "#,
        color
    );
    Handle::from_memory(svg.into_bytes())
}

pub fn get_avatar_handle(name: &str, color: &str) -> Handle {
    match name {
        "robot" => avatar_robot(color),
        "alien" => avatar_alien(color),
        "ghost" => avatar_ghost(color),
        "peak" => avatar_peak(color),
        "smile" => avatar_smile(color),
        _ => avatar_smile(color), // Default
    }
}

pub const AVATAR_OPTIONS: [&str; 5] = ["robot", "alien", "ghost", "peak", "smile"];
