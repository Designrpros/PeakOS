#!/bin/bash
set -e

# Set cargo PATH directly (rustup installs to /root/.cargo/bin)
export PATH="/root/.cargo/bin:$PATH"

echo "=== PeakOS Builder (Alpine) ==="
ARCH=$(uname -m)
echo "Architecture: $ARCH"

if [ "$ARCH" = "aarch64" ]; then
    GRUB_TARGET="arm64-efi"
    EFI_NAME="BOOTAA64.EFI"
    SERIAL_CONSOLE="ttyAMA0"
    TARGET="aarch64-unknown-linux-musl"
else
    GRUB_TARGET="x86_64-efi"
    EFI_NAME="BOOTX64.EFI"
    SERIAL_CONSOLE="ttyS0"
    TARGET="x86_64-unknown-linux-musl"
    EARLYCON=""
fi

# Export variables (System Rust on Alpine handles paths automatically now)
# export RUSTFLAGS="..." # REMOVED
# export PKG_CONFIG_PATH="..." # REMOVED

if [ "$ARCH" = "aarch64" ]; then
    # PL011 is common for QEMU/Virt ARM64
    EARLYCON="earlycon=pl011,mmio32,0x09000000"
fi

# Paths
rm -rf /build/rootfs /build/iso
mkdir -p /build/rootfs
mkdir -p /build/iso/boot/grub
mkdir -p /build/out

# 1. Compile Peak Desktop
echo "--- Compiling Peak Desktop ---"
cd /project/crates/modes/desktop
# Generate a lockfile if missing so we can pin dependencies
if [ ! -f Cargo.lock ]; then
    cargo generate-lockfile
fi
# Pin smithay-clipboard
cargo update -p smithay-clipboard --precise 0.7.2 || true
# Pin dlopen2_derive
cargo update -p dlopen2_derive --precise 0.4.1 || true
# Pin async-lock
cargo update -p async-lock --precise 3.4.1 || true
# Pin url to avoid zerovec issues
cargo update -p url --precise 2.5.4 || true
# Pin iced_graphics to match iced_renderer 0.13.0
cargo update -p iced_graphics --precise 0.13.0 || true

# Use architecture-specific build directory
# Use architecture-specific build directory
export CARGO_TARGET_DIR=/build/target/$ARCH
# Clean target dir to ensure llama-server is not cached
rm -rf "$CARGO_TARGET_DIR"

# Build with explicit target to avoid host confusion
# REVERT: Explicit target causes "can't find crate for core" on system rust. 
# We trust the host (Alpine) to build for itself (musl).
# peak-intelligence has default-features = false in peak-desktop/Cargo.toml to skip llama-server

# NOTE: GTK linker workarounds removed - no longer needed since wry/webkit2gtk were removed

cargo build --release --manifest-path /project/crates/modes/desktop/Cargo.toml

# Copy binary
echo "Searching for binary..."
# Without --target, it puts release in target/release, not target/$TARGET/release
BIN_PATH="/build/target/$ARCH/release/peak-desktop"
cp "$BIN_PATH" /build/rootfs/peak-desktop
chmod +x /build/rootfs/peak-desktop

# NOTE: peak-browser removed - using Firefox instead (avoids webkit2gtk static linking issues)
# Firefox is installed via apk in the rootfs setup below

# Copy assets directory for icons, fonts, etc.
echo "Copying assets..."
mkdir -p /build/rootfs/usr/share/peakos/assets
cp -r /project/assets/* /build/rootfs/usr/share/peakos/assets/
# Exclude legacy binaries
rm -rf /build/rootfs/usr/share/peakos/assets/bin

# Copy boot animation script
cp /build/boot_animation.sh /build/rootfs/boot_animation.sh
chmod +x /build/rootfs/boot_animation.sh

echo "Compilation success."

# 2. Build Minimal RootFS
echo "--- Building RootFS ---"
# Initialize apk database
mkdir -p /build/rootfs/etc/apk
cp /etc/apk/repositories /build/rootfs/etc/apk/repositories

# Map architecture names for APK
if [ "$ARCH" = "x86_64" ]; then
    APK_ARCH="x86_64"
elif [ "$ARCH" = "aarch64" ]; then
    APK_ARCH="aarch64"
else
    APK_ARCH="$ARCH"
fi

echo "   Updating crates.io index"

# Install Base System + Kernel + Input Devices
# wireless-tools/wpa_supplicant needed for wifi?
apk --root /build/rootfs --initdb add --arch "$APK_ARCH" --no-cache --allow-untrusted \
    alpine-base linux-lts linux-firmware-none \
    udev eudev libinput libinput-dev \
    alsa-lib wayland mesa-dri-gallium \
    labwc seatd \
    webkit2gtk-4.1 \
    adwaita-icon-theme \
    ttf-dejavu font-noto font-noto-cjk \
    dbus networkmanager networkmanager-cli networkmanager-wifi wpa_supplicant \
    firefox ca-certificates fbida-fbi \
    bluez bluez-tools bluez-deprecated \
    alsa-utils

# Configure doas and sudo compatibility
echo "permit nopass keepenv root" > /build/rootfs/etc/doas.d/doas.conf
ln -sf /usr/bin/doas /build/rootfs/usr/bin/sudo




# --- Configuration Phase ---
echo "--- Configuring Startup ---"

# 0. Regenerate initramfs with VirtIO drivers
echo "--- Regenerating Initramfs ---"
echo 'features="base keymap kms mmc network virtio"' > /build/rootfs/etc/mkinitfs/mkinitfs.conf
# We must chroot to run mkinitfs correctly or use the host tool targeting the directory
# On Alpine Docker, we can use the host tool:
mkinitfs -n -b /build/rootfs -c /build/rootfs/etc/mkinitfs/mkinitfs.conf -o /build/rootfs/boot/initramfs-lts $(ls /build/rootfs/lib/modules/)

# 1. labwc Configuration
mkdir -p /build/rootfs/etc/xdg/labwc
cat > /build/rootfs/etc/xdg/labwc/rc.xml <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<labwc_config>
  <core>
    <decoration>server</decoration>
  </core>
  <theme>
    <name>Adwaita</name>
    <cornerRadius>8</cornerRadius>
  </theme>
  <focus>
    <followMouse>no</followMouse>
  </focus>
  <keyboard>
    <keybind key="A-Tab">
      <action name="NextWindow"/>
    </keybind>
  </keyboard>
</labwc_config>
EOF

# labwc autostart (launches peak-desktop automatically)
cat > /build/rootfs/etc/xdg/labwc/autostart <<EOF
/peak-desktop &
EOF
chmod +x /build/rootfs/etc/xdg/labwc/autostart

# 2. Startup Script
cat > /build/rootfs/init_peak.sh <<EOF
#!/bin/sh
# PeakOS Startup Script

# Show boot animation
if [ -f /boot_animation.sh ]; then
    /boot_animation.sh
fi

# Setup Environment
export XDG_RUNTIME_DIR=/tmp/xdg
mkdir -p \$XDG_RUNTIME_DIR
chmod 700 \$XDG_RUNTIME_DIR
# labwc uses wlroots backend automatically

# Cursor Configuration
export XCURSOR_THEME=Adwaita
export XCURSOR_SIZE=24

# Start Seat Management Daemon
seatd &
# Wait for seatd and GPU drivers to stabilize
sleep 3

# Launch labwc (Wayland Compositor)
# labwc auto-launches peak-desktop via /etc/xdg/labwc/autostart
labwc >> /dev/tty1 2>&1 &

# Wait for Wayland socket to appear (Increased timeout for slow emulation)
echo "Waiting for Wayland socket..."
SOCKET_NAME=""
for i in \$(seq 1 20); do
    SOCKET_FOUND=\$(find \$XDG_RUNTIME_DIR -name "wayland-*" | head -n 1)
    if [ -n "\$SOCKET_FOUND" ]; then
        echo "Wayland socket found: \$SOCKET_FOUND"
        SOCKET_NAME=\$(basename "\$SOCKET_FOUND")
        break
    fi
    sleep 1
done

if [ -z "\$SOCKET_NAME" ]; then
    echo "Error: Wayland socket not found!"
    # Fallback just in case
    SOCKET_NAME="wayland-0"
fi

# PeakOS is launched by labwc autostart
echo "PeakOS launched via labwc autostart on \$SOCKET_NAME"
export WAYLAND_DISPLAY=\$SOCKET_NAME

# Prevent script from exiting so init doesn't respawn it immediately (if using respawn)
# Or just let it exit if OpenRC expects it to. OpenRC 'start' should exit.
# But here we are spawning background processes.
EOF
chmod +x /build/rootfs/init_peak.sh

# 3. Create OpenRC service for automatic graphical startup
cat > /build/rootfs/etc/init.d/peakos <<'SERVICE_EOF'
#!/sbin/openrc-run

description="PeakOS Graphical Environment"

depend() {
    need modules
    after *
}

start() {
    ebegin "Starting PeakOS graphical environment"
    /init_peak.sh &
    eend $?
}
SERVICE_EOF

chmod +x /build/rootfs/etc/init.d/peakos

# Enable the PeakOS service to run at boot (default runlevel)
ln -sf /etc/init.d/peakos /build/rootfs/etc/runlevels/default/peakos

# Enable udev for input device detection
ln -sf /etc/init.d/udev /build/rootfs/etc/runlevels/sysinit/udev
ln -sf /etc/init.d/udev-trigger /build/rootfs/etc/runlevels/sysinit/udev-trigger
ln -sf /etc/init.d/udev-settle /build/rootfs/etc/runlevels/sysinit/udev-settle

# Enable Networking Services (NetworkManager)
# DBus is required for NetworkManager
ln -sf /etc/init.d/dbus /build/rootfs/etc/runlevels/default/dbus
ln -sf /etc/init.d/networkmanager /build/rootfs/etc/runlevels/default/networkmanager
ln -sf /etc/init.d/wpa_supplicant /build/rootfs/etc/runlevels/default/wpa_supplicant

# Enable Bluetooth
ln -sf /etc/init.d/bluetooth /build/rootfs/etc/runlevels/default/bluetooth

# Keep serial console for debugging
echo "$SERIAL_CONSOLE::respawn:/sbin/getty -L 0 $SERIAL_CONSOLE vt100" >> /build/rootfs/etc/inittab

# Enable modules service to auto-load kernel modules on boot
ln -sf /etc/init.d/modules /build/rootfs/etc/runlevels/boot/modules

# 3. Pack Initramfs/Kernel?
# Actually, making a bootable ISO from scratch is complex.
# We will use 'mtools' to make an EFI partition image.

echo "--- Generating ISO (Experimental) ---"
# Copy kernel/initramfs from rootfs to ISO boot folder
cp /build/rootfs/boot/vmlinuz-lts /build/iso/boot/vmlinuz
cp /build/rootfs/boot/initramfs-lts /build/iso/boot/initrd

# DEBUG: Check kernel file type
echo "--- Kernel File Info ---"
if command -v file >/dev/null 2>&1; then
    file /build/iso/boot/vmlinuz
else
    echo "'file' command not found"
fi


# 2. Configure Modules Loading (OpenRC style)
# Ensure virtio modules are loaded by OpenRC modules service
# Clear any existing entries first to avoid duplicates
cat > /build/rootfs/etc/modules <<MODULES_EOF
af_packet
ipv6
virtio_pci
virtio-gpu
virtio-input
evdev
snd-dummy
MODULES_EOF

# Manual Depmod to ensure modules.dep is correct for our kernel
echo "--- Running Depmod ---"
KVER=$(ls /build/rootfs/lib/modules/ | head -n 1)
depmod -a -b /build/rootfs "$KVER"

# 3. Create Custom /init Script (PID 1)
# This replaces the stock Alpine initramfs logic.
# It acts as the bridge between Kernel and OpenRC.
cat > /build/rootfs/init <<EOF
#!/bin/sh
# Early Init Script
set -x
dmesg -n 8

mount -t proc -o noexec,nosuid,nodev proc /proc
mount -t sysfs -o noexec,nosuid,nodev sysfs /sys
mount -t devtmpfs -o exec,nosuid,mode=0755 devtmpfs /dev
mkdir -p /dev/pts
mount -t devpts -o gid=5,mode=620 devpts /dev/pts

echo "--- PeakOS Early Init ---"

# Load essential kernel modules
# VirtIO PCI must be loaded first, then GPU driver can attach
modprobe virtio_pci || echo "Failed to load virtio_pci"
modprobe snd-dummy || echo "Failed to load snd-dummy"

# Handover to OpenRC (System Init)
echo "--- Starting OpenRC ---"
exec /sbin/init
EOF
chmod +x /build/rootfs/init

# 4. Modify Inittab
# Remove standard gettys
# Enable standard login on tty1 and ttyS0
sed -i 's/^tty/#tty/g' /build/rootfs/etc/inittab
echo "ttyS0::respawn:/sbin/getty -L 0 ttyS0 vt100" >> /build/rootfs/etc/inittab
# Auto-login on the main serial console for easier debugging
echo "$SERIAL_CONSOLE::respawn:/sbin/getty -n -l /bin/sh -L 0 $SERIAL_CONSOLE vt100" >> /build/rootfs/etc/inittab
echo "tty1::respawn:/sbin/getty 38400 tty1" >> /build/rootfs/etc/inittab

# 5. Pack Initramfs (Pure RootFS)
echo "--- Generating Pure Initramfs ---"
# We do NOT use the stock initramfs. We pack our entire rootfs.
cd /build/rootfs
find . | cpio -H newc -o | gzip > /build/iso/boot/initrd

# 6. Copy Kernel
cp /build/rootfs/boot/vmlinuz-lts /build/iso/boot/vmlinuz

# 7. Create GRUB Config
# Note: rdinit=/init tells kernel to execute our script.
cat > /build/iso/boot/grub/grub.cfg <<EOF
set timeout=3
set gfxmode=1920x1080
set gfxpayload=keep
menuentry "PeakOS ($ARCH)" {
    search --no-floppy --set=root --label PeakOS
    linux /boot/vmlinuz root=/dev/ram0 rdinit=/init console=$SERIAL_CONSOLE console=tty0 $EARLYCON debug keep_bootcon video=1920x1080
    initrd /boot/initrd
}
EOF

# Generate BOOTAA64.EFI
mkdir -p /build/iso/EFI/BOOT
echo "--- Generarting EFI Image ---"
grub-mkimage \
    -p /boot/grub \
    -o /build/iso/EFI/BOOT/$EFI_NAME \
    -O $GRUB_TARGET \
    -d /usr/lib/grub/$GRUB_TARGET \
    boot linux normal configfile part_gpt part_msdos fat iso9660 \
    search search_fs_file search_fs_uuid search_label \
    efi_gop ext2 f2fs xfs gzio xzio zstd \
    all_video gfxterm gettext echo ls cat test help

# Generate ISO
cd /build
echo "--- Generating ISO (Manual Xorriso) ---"

# 1. Create EFI Partition Image
# We create it inside /build/iso so it is available for -e (El Torito)
dd if=/dev/zero of=/build/iso/efiboot.img bs=1M count=10
mkfs.vfat /build/iso/efiboot.img
mmd -i /build/iso/efiboot.img ::/EFI
mmd -i /build/iso/efiboot.img ::/EFI/BOOT
mcopy -i /build/iso/efiboot.img /build/iso/EFI/BOOT/$EFI_NAME ::/EFI/BOOT/$EFI_NAME
# Copy GRUB config to EFI partition as well just in case
mcopy -i /build/iso/efiboot.img /build/iso/boot/grub/grub.cfg ::/EFI/BOOT/grub.cfg

# 2. Run Xorriso
ISO_NAME="peakos-latest.iso"

# Clean previous ISOs to free space
rm -f /out/*.iso

# Build internally first to avoid "disk full" false positives on mounted volumes
xorriso -as mkisofs \
    -iso-level 3 \
    -full-iso9660-filenames \
    -volid "PeakOS" \
    -eltorito-alt-boot \
    -e efiboot.img \
    -no-emul-boot \
    -append_partition 2 0xEF /build/iso/efiboot.img \
    -output "/out/peakos-alpine-${TIMESTAMP}.iso" \
    /build/iso
    
echo "--- Done ---"
ls -lh /out/
