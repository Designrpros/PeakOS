use std::path::PathBuf;

pub fn get_asset_path(relative_path: &str) -> PathBuf {
    // 1. Check environment variable for production/custom paths
    if let Ok(assets_root) = std::env::var("PEAK_ASSETS") {
        let mut path = PathBuf::from(assets_root);
        path.push(relative_path);
        return path;
    }

    // 2. Dev mode fallback (using compile-time manifest dir)
    let dev_root = env!("CARGO_MANIFEST_DIR");
    let mut path = PathBuf::from(dev_root);
    path.pop(); // Up to crates/
    path.pop(); // Up to root
    path.push("assets");
    path.push(relative_path);

    if path.exists() {
        return path;
    }

    // 3. Production fallback (system-wide install)
    let mut path = PathBuf::from("/usr/share/peakos/assets");
    path.push(relative_path);
    path
}
