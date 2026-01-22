use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub full_name: String,
    pub password_hash: String, // Simple placeholder hash for now
    pub avatar_path: Option<String>,
    pub avatar_icon: Option<String>, // New: Key for src/icons.rs
    pub theme_preference: String,    // "Peak" or "Riviera"
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            username: "guest".to_string(),
            full_name: "Guest User".to_string(),
            password_hash: "".to_string(),
            avatar_path: None,
            avatar_icon: None,
            theme_preference: "Peak".to_string(),
        }
    }
}

pub fn get_config_dir() -> PathBuf {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut path = dirs::config_dir().unwrap_or(PathBuf::from("."));
        path.push("peakos");
        fs::create_dir_all(&path).ok();
        path
    }
    #[cfg(target_arch = "wasm32")]
    PathBuf::from("/")
}

pub fn load_user() -> Option<UserProfile> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_config_dir().join("user.json");
        if let Ok(content) = fs::read_to_string(path) {
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }
    #[cfg(target_arch = "wasm32")]
    None
}

pub fn save_user(profile: &UserProfile) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_config_dir().join("user.json");
        if let Ok(content) = serde_json::to_string_pretty(profile) {
            fs::write(path, content).is_ok()
        } else {
            false
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = profile;
        true
    }
}
