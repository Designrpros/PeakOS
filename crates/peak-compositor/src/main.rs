#[cfg(target_os = "linux")]
mod backend_winit;
#[cfg(target_os = "linux")]
mod state;

#[cfg(target_os = "linux")]
use smithay::reexports::wayland_server::Display;
#[cfg(target_os = "linux")]
use state::PeakCompositor;

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    tracing::info!("Peak Compositor starting...");

    let mut event_loop = smithay::reexports::calloop::EventLoop::try_new()?;
    let display = Display::new()?;
    let display_handle = display.handle();

    let mut state = PeakCompositor::new(display_handle);

    // In a real implementation, we would initialize backends here
    // For now, this is the skeleton of the compositor process.

    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Peak Compositor is only supported on Linux.");
}
