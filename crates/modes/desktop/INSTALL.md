# Installing Peak Native on Linux

Peak Native is a standalone Desktop Environment shell written in Rust. It can run on any Linux distribution that supports Wayland (or X11 via XWayland).

## Prerequisites

You need the following system packages installed (Development Headers):
- **Rust Toolchain**: `cargo` (Install via [rustup](https://rustup.rs))
- **System Libraries**:
  - **Debian/Ubuntu**: `build-essential libssl-dev pkg-config libwayland-dev libx11-dev libxkbcommon-dev libfontconfig1-dev libwebkit2gtk-4.1-dev libgtk-3-dev`
  - **Arch Linux**: `base-devel wayland libxkbcommon fontconfig webkit2gtk gtk3`
  - **Fedora**: `openssl-devel wayland-devel libxkbcommon-devel fontconfig-devel webkit2gtk4.1-devel gtk3-devel`

## Installation

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/Designrpros/PeakOS.git
    cd PeakOS
    ```

2.  **Build**:
    ```bash
    cargo build -p peak-desktop --release
    ```

3.  **Run**:
    You can run it directly from your terminal within an existing desktop session:
    ```bash
    ./target/release/peak-desktop
    ```

## Running as a Desktop Session (Advanced)

To log in to PeakOS directly from your display manager (GDM/SDDM):

1.  **Create a Session File** at `/usr/share/wayland-sessions/peakos.desktop`:
    ```ini
    [Desktop Entry]
    Name=PeakOS
    Comment=Peak Native Session
    Exec=/usr/local/bin/peak-session
    Type=Application
    ```

2.  **Create the Session Script** at `/usr/local/bin/peak-session`:
    ```bash
    #!/bin/sh
    # Launch labwc with Peak Native as the shell
    export MOZ_ENABLE_WAYLAND=1
    # labwc will launch peak-desktop via its autostart config
    exec labwc
    ```
    *(Note: This requires `labwc` installed).*

## Architecture
Peak Native is **NOT** a "rice".
- A "rice" is a configuration of existing tools (Polybar, Rofi, i3).
- **Peak Native** is a monolithic shell written in Rust. It draws its own panels, docks, launcher, and window controls using the GPU. It is built to replace GNOME Shell or Plasma Shell entirely.
