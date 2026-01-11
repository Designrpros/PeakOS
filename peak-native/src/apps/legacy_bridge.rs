use std::process::Command;

#[allow(dead_code)]
pub fn launch_matrix_space() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string());
    // Path to the copied binary
    let bin_path = format!("{}/assets/bin/peak-web", root_dir);

    println!("Launching Matrix Uplink at: {}", bin_path);

    // Launch detached so it survives if we close (or dependent, your choice)
    let result = Command::new(bin_path).spawn();

    match result {
        Ok(_) => println!("Matrix Space Jump Successful."),
        Err(e) => eprintln!("Failed to launch Matrix Space: {}. Did you run 'npm run tauri build' and copy the binary?", e),
    }
}
