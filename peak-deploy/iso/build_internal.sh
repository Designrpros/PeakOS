#!/bin/bash
set -e

echo "=== [1/4] Building Peak Intelligence (Sidecar) ==="
cd /project/peak-intelligence
cargo build --release
TRIPLE="x86_64-unknown-linux-gnu"
SIDECAR_NAME="peak-intelligence-$TRIPLE"

echo "=== [2/4] Building Peak Desktop ==="
cd /project/peak-desktop

# Ensure sidecar is built and placed correctly
echo "Ensuring Sidecar is built..."
cd /project/peak-intelligence
cargo build --release
cd /project/peak-desktop

mkdir -p src-tauri/bin
# Check both local and workspace-level target dirs for the sidecar
if [ -f "/project/peak-intelligence/target/release/peak-intelligence" ]; then
    cp "/project/peak-intelligence/target/release/peak-intelligence" src-tauri/bin/peak-intelligence-x86_64-unknown-linux-gnu
elif [ -f "/project/target/release/peak-intelligence" ]; then
    cp "/project/target/release/peak-intelligence" src-tauri/bin/peak-intelligence-x86_64-unknown-linux-gnu
else
    echo "ERROR: Sidecar binary not found!"
    ls -R /project/target /project/peak-intelligence/target
    exit 1
fi
chmod +x src-tauri/bin/peak-intelligence-x86_64-unknown-linux-gnu

echo "Installing NPM Dependencies..."
npm install
echo "Building Frontend..."
npm run build

# Run production Tauri build
echo "Running Production Tauri Build..."
npx tauri build --no-bundle

cd .. # Return to /project

echo "=== [3/4] Preparing ISO Profile (Releng Base) ==="
# Use the official releng profile as a base to ensure bootloaders works
PROFILE_DIR="/tmp/peak-profile"
rm -rf "$PROFILE_DIR"
cp -r /usr/share/archiso/configs/releng "$PROFILE_DIR"
# Apply our custom profile on top of releng
cp -rv /build/profile/* "$PROFILE_DIR/"

echo "=== [Branding] Rebranding to PeakOS ==="
# Replace branding in bootloader configs (target specific directories)
find "$PROFILE_DIR/syslinux" "$PROFILE_DIR/efiboot" -type f -exec sed -i 's/Arch Linux/PeakOS/g' {} +
find "$PROFILE_DIR/syslinux" "$PROFILE_DIR/efiboot" -type f -exec sed -i 's/archlinux/peakos/g' {} +

# Ensure pacman.conf is in the profile (required by mkarchiso)
if [ ! -f "$PROFILE_DIR/pacman.conf" ]; then
    cp /etc/pacman.conf "$PROFILE_DIR/pacman.conf"
fi

# Ensure mirrorlist exists in airootfs/etc/pacman.d (required by archiso hooks)
mkdir -p "$PROFILE_DIR/airootfs/etc/pacman.d"
if [ ! -f "$PROFILE_DIR/airootfs/etc/pacman.d/mirrorlist" ]; then
    cp /etc/pacman.d/mirrorlist "$PROFILE_DIR/airootfs/etc/pacman.d/mirrorlist"
fi
# Ensure syslinux/grub entries use the new install_dir
sed -i "s/install_dir=arch/install_dir=peakos/g" "$PROFILE_DIR/syslinux/archiso_sys.cfg" 2>/dev/null || true
sed -i "s/install_dir=arch/install_dir=peakos/g" "$PROFILE_DIR/efiboot/loader/entries/archiso-x86_64-linux.conf" 2>/dev/null || true

echo "=== [Branding] OS Identity ==="
# Provide custom os-release and issue files
mkdir -p "$PROFILE_DIR/airootfs/etc"
cat <<EOF > "$PROFILE_DIR/airootfs/etc/os-release"
NAME="PeakOS"
PRETTY_NAME="PeakOS Live"
ID=peakos
ID_LIKE=arch
BUILD_ID=rolling
ANSI_COLOR="38;2;255;255;255"
HOME_URL="https://peakos.org/"
DOCUMENTATION_URL="https://wiki.peakos.org/"
SUPPORT_URL="https://peakos.org/support"
BUG_REPORT_URL="https://github.com/peakos/peakos/issues"
LOGO=peak-logo
EOF

cat <<EOF > "$PROFILE_DIR/airootfs/etc/issue"
Welcome to PeakOS (\l) - \D %t
EOF

# Enable seatd service for hardware access (fixes "failed to open seat" error)
mkdir -p "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants"
ln -sf /usr/lib/systemd/system/seatd.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/seatd.service"

echo "=== [4/4] Installing PeakOS & Configuring Start ==="
DEST_BIN="$PROFILE_DIR/airootfs/usr/bin"
mkdir -p "$DEST_BIN"

# Install Sidecar
cp "/project/peak-desktop/src-tauri/bin/peak-intelligence-x86_64-unknown-linux-gnu" "$DEST_BIN/$SIDECAR_NAME"

# Install Desktop APP
if [ -f "/project/peak-desktop/src-tauri/target/release/peak-desktop" ]; then
    cp "/project/peak-desktop/src-tauri/target/release/peak-desktop" "$DEST_BIN/peak-desktop"
elif [ -f "/project/target/release/peak-desktop" ]; then
    cp "/project/target/release/peak-desktop" "$DEST_BIN/peak-desktop"
else
    echo "ERROR: Desktop binary not found!"
    ls -R /project/target /project/peak-desktop/src-tauri/target
    exit 1
fi

# Ensure executable (Force 755 to be sure)
chmod 755 "$DEST_BIN/$SIDECAR_NAME"
chmod 755 "$DEST_BIN/peak-desktop"

# Configure Auto-Start (zlogin for root)
mkdir -p "$PROFILE_DIR/airootfs/root"
cat <<EOF > "$PROFILE_DIR/airootfs/root/.zlogin"
if [[ -z \$DISPLAY ]] && [[ \$(tty) = /dev/tty1 ]]; then
  echo "Welcome to PeakOS!"
  
  # Setup Wayland Runtime Directory
  export XDG_RUNTIME_DIR=/tmp/runtime-root
  mkdir -p \$XDG_RUNTIME_DIR
  chmod 700 \$XDG_RUNTIME_DIR

  # Force Software Rendering for stability on old GPUs
  export WLR_RENDERER=pixman
  export GDK_BACKEND=wayland
  
  # WebKitGTK Stability & Security Bypass
  export WEBKIT_DISABLE_COMPOSITING_MODE=1
  export WEBKIT_DISABLE_SANDBOX_FAKE_XDISPLAY=1
  export WEBKIT_FORCE_SANDBOX=0
  export WEBKIT_DISABLE_SANDBOX_THIS_IS_DANGEROUS=1
  export WEBKIT_DISABLE_DMABUF_RENDERER=1
  export WEBKIT_USE_GLIB_NETWORKING=0

  # Debug Info
  echo "--- Environment Check ---"
  id
  ls -l /usr/bin/peak-desktop
  file /usr/bin/peak-desktop
  echo "------------------------"

  # Start Cage
  echo "Starting Peak Desktop (Software Mode)..."
  # Redirect output to log file for debugging if screen stays blank
  if ! cage -- /usr/bin/peak-desktop > /tmp/peak.log 2>&1; then
      echo "---------------------------------------------------"
      echo "PeakOS Desktop failed to launch. Checking logs..."
      echo "---------------------------------------------------"
      cat /tmp/peak.log
      journalctl -t cage -t peak-desktop --no-pager -n 50
      exec zsh
  fi
fi
EOF

echo "Cleaning up any old build artifacts..."
# Fix permissions from previous run (Docker on Mac can be fussy with setuid files)
# chmod -R 777 /build/work 2>/dev/null || true
rm -rf /tmp/work /build/out
mkdir -p /tmp/work /build/out

echo "=== Generating ISO ==="
# Run mkarchiso against our modified temporary profile
# Output goes to the mounted /build/out
mkarchiso -v -w /tmp/work -o /build/out "$PROFILE_DIR"
