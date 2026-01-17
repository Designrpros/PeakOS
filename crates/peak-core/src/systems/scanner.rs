#![allow(dead_code)]
use crate::models::{MediaItem, MediaKind, MediaStatus};
use std::path::Path;

pub struct AppScanner;

impl AppScanner {
    pub fn scan() -> Vec<MediaItem> {
        let mut apps = Vec::new();

        // Scan for system binaries (Alpine/Linux apps)
        apps.extend(Self::scan_system_binaries());

        // Scan for macOS apps (development environment)
        apps.extend(Self::scan_macos_apps());

        apps
    }

    fn scan_macos_apps() -> Vec<MediaItem> {
        let mut apps = Vec::new();

        // Check if /Applications exists (macOS only)
        let app_dir = Path::new("/Applications");
        if !app_dir.exists() {
            return apps;
        }

        // Scan all .app bundles in /Applications
        if let Ok(entries) = std::fs::read_dir(app_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "app" {
                        if let Some(app_name) = path.file_stem() {
                            let name = app_name.to_string_lossy().to_string();
                            let id = name.to_lowercase().replace(' ', "_");

                            apps.push(MediaItem {
                                id,
                                title: name.clone(),
                                cover_image: format!("{}_icon.png", name.to_lowercase()),
                                launch_command: format!("open \"{}\"", path.display()),
                                kind: MediaKind::App,
                                status: MediaStatus::Ready,
                                image_handle: None,
                            });
                        }
                    }
                }
            }
        }

        apps
    }

    fn scan_system_binaries() -> Vec<MediaItem> {
        let mut apps = Vec::new();

        // List of known apps to detect
        let known_apps = [
            ("chromium", "Chromium", "Open source web browser."),
            ("brave", "Brave", "Privacy-focused browser."),
            ("gimp", "GIMP", "GNU Image Manipulation Program."),
            ("inkscape", "Inkscape", "Vector graphics editor."),
            ("vlc", "VLC", "Media player."),
            ("code", "VS Code", "Code editor."),
            ("blender", "Blender", "3D creation suite."),
        ];

        for (binary, name, _description) in known_apps {
            if Self::check_binary_installed(binary) {
                apps.push(MediaItem {
                    id: binary.to_string(),
                    title: name.to_string(),
                    cover_image: format!("{}_icon.png", binary),
                    launch_command: binary.to_string(),
                    kind: MediaKind::App,
                    status: MediaStatus::Ready,
                    image_handle: None,
                });
            }
        }

        apps
    }

    fn check_binary_installed(binary: &str) -> bool {
        // Check if binary exists in PATH
        std::process::Command::new("which")
            .arg(binary)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

pub struct MusicScanner;

impl MusicScanner {
    pub fn scan() -> Vec<MediaItem> {
        let mut tracks = Vec::new();
        if let Some(user_dirs) = directories::UserDirs::new() {
            if let Some(audio_dir) = user_dirs.audio_dir() {
                // Initial simple scan: just list files
                // Real impl: Use `walkdir`
                if let Ok(entries) = std::fs::read_dir(audio_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if let Some(ext) = path.extension() {
                            if ext == "mp3" || ext == "flac" || ext == "wav" {
                                let filename = path.file_stem().unwrap().to_string_lossy();
                                // Basic cover art assumption? or default icon
                                tracks.push(MediaItem {
                                    id: filename.to_string(),
                                    title: filename.to_string(),
                                    cover_image: "music_icon.png".into(),
                                    launch_command: format!("play \"{}\"", path.display()), // Needs specialized logic
                                    kind: MediaKind::Music,
                                    status: MediaStatus::Ready,
                                    image_handle: None,
                                });
                            }
                        }
                    }
                }
            }
        }
        tracks
    }
}
