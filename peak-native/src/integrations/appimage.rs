// AppImage integration for PeakOS
// Enables running self-contained Linux applications

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

#[derive(Debug, Clone)]
pub struct AppImageInfo {
    pub path: PathBuf,
    pub name: String,
    pub is_executable: bool,
}

pub struct AppImageManager {
    install_dir: PathBuf,
}

impl AppImageManager {
    pub fn new() -> Self {
        let install_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".peak")
            .join("appimages");

        // Create directory if it doesn't exist
        let _ = fs::create_dir_all(&install_dir);

        Self { install_dir }
    }

    /// Check if a file is an AppImage
    pub fn is_appimage(path: &Path) -> bool {
        // Check file extension
        if let Some(ext) = path.extension() {
            if ext == "AppImage" {
                return true;
            }
        }

        // TODO: Could also check ELF magic bytes + squashfs signature
        // for now, extension is good enough

        false
    }

    /// Install an AppImage (copy to managed directory and make executable)
    pub fn install(&self, source_path: &Path) -> Result<AppImageInfo, String> {
        if !Self::is_appimage(source_path) {
            return Err("Not an AppImage file".to_string());
        }

        if !source_path.exists() {
            return Err("File does not exist".to_string());
        }

        // Get filename
        let filename = source_path
            .file_name()
            .ok_or("Invalid filename")?
            .to_string_lossy()
            .to_string();

        // Copy to install directory
        let dest_path = self.install_dir.join(&filename);
        fs::copy(source_path, &dest_path).map_err(|e| format!("Failed to copy AppImage: {}", e))?;

        // Make executable
        let mut perms = fs::metadata(&dest_path)
            .map_err(|e| format!("Failed to get permissions: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest_path, perms)
            .map_err(|e| format!("Failed to set permissions: {}", e))?;

        Ok(AppImageInfo {
            path: dest_path,
            name: filename.trim_end_matches(".AppImage").to_string(),
            is_executable: true,
        })
    }

    /// Run an AppImage
    pub fn run(&self, info: &AppImageInfo) -> Result<Child, String> {
        if !info.path.exists() {
            return Err("AppImage not found".to_string());
        }

        Command::new(&info.path)
            .spawn()
            .map_err(|e| format!("Failed to launch AppImage: {}", e))
    }

    /// List installed AppImages
    pub fn list_installed(&self) -> Vec<AppImageInfo> {
        let mut appimages = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.install_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if Self::is_appimage(&path) {
                    appimages.push(AppImageInfo {
                        name: path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                        path,
                        is_executable: true,
                    });
                }
            }
        }

        appimages
    }

    /// Remove an installed AppImage
    pub fn uninstall(&self, info: &AppImageInfo) -> Result<(), String> {
        fs::remove_file(&info.path).map_err(|e| format!("Failed to remove AppImage: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_appimage() {
        assert!(AppImageManager::is_appimage(Path::new("test.AppImage")));
        assert!(!AppImageManager::is_appimage(Path::new("test.deb")));
        assert!(!AppImageManager::is_appimage(Path::new("test")));
    }
}
