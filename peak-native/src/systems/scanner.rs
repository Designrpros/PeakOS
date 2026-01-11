#![allow(dead_code)]
use crate::models::{MediaItem, MediaKind, MediaStatus};
use std::path::Path;

pub struct AppScanner;

impl AppScanner {
    pub fn scan() -> Vec<MediaItem> {
        let mut apps = Vec::new();
        // Check for Stremio
        if Path::new("/Applications/Stremio.app").exists() {
            apps.push(MediaItem {
                id: "stremio".into(),
                title: "Stremio".into(),
                cover_image: "stremio_icon.png".into(), // Needs asset
                launch_command: "open /Applications/Stremio.app".into(),
                kind: MediaKind::Movie,
                status: MediaStatus::Ready,
                image_handle: None,
            });
        }
        // Check for VLC
        if Path::new("/Applications/VLC.app").exists() {
            apps.push(MediaItem {
                id: "vlc".into(),
                title: "VLC Media Player".into(),
                cover_image: "vlc_icon.png".into(),
                launch_command: "open /Applications/VLC.app".into(),
                kind: MediaKind::Movie,
                status: MediaStatus::Ready,
                image_handle: None,
            });
        }

        apps
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
