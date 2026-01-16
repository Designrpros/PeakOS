#!/bin/bash
set -e

echo "=== PeakOS Builder (Alpine) ==="
ARCH=$(uname -m)
echo "Architecture: $ARCH"

if [ "$ARCH" = "aarch64" ]; then
    GRUB_TARGET="arm64-efi"
    EFI_NAME="BOOTAA64.EFI"
    SERIAL_CONSOLE="ttyAMA0"
else
    GRUB_TARGET="x86_64-efi"
    EFI_NAME="BOOTX64.EFI"
    SERIAL_CONSOLE="ttyS0"
    # x86_64 typically uses standard VGA or defaults. 
    # For earlycon on x86, typically uart8250,io,0x3f8 but often not needed if console=ttyS0 is set.
    EARLYCON="" 
fi

if [ "$ARCH" = "aarch64" ]; then
    # PL011 is common for QEMU/Virt ARM64
    EARLYCON="earlycon=pl011,mmio32,0x09000000"
fi

# Paths
# Paths
rm -rf /build/rootfs /build/iso
mkdir -p /build/rootfs
mkdir -p /build/iso/boot/grub
mkdir -p /build/out

# 1. Compile Peak Native
echo "--- Compiling Peak Native ---"
cd /project/peak-native
# Force MUSL target if needed, but on Alpine it's default
# Generate a lockfile if missing so we can pin dependencies
if [ ! -f Cargo.lock ]; then
    cargo generate-lockfile
fi
# Pin smithay-clipboard to 0.7.2 to avoid edition2024 error in 0.7.3 (broken on stable)
cargo update -p smithay-clipboard --precise 0.7.2 || true
# Pin dlopen2_derive to 0.4.1 (0.4.2+ seems to have edition 2024)
cargo update -p dlopen2_derive --precise 0.4.1 || true
# Pin async-lock to 3.4.1 to avoid rust 1.85 requirement in 3.4.2
cargo update -p async-lock --precise 3.4.1 || true
# Use architecture-specific build directory to avoid conflicts
export CARGO_TARGET_DIR=/build/target/$ARCH
cargo build --release
echo "Searching for binary..."
find $CARGO_TARGET_DIR -name "peak-native" -type f
BIN_PATH=$(find $CARGO_TARGET_DIR -name "peak-native" -type f | head -n 1)
cp "$BIN_PATH" /build/rootfs/peak-native
chmod +x /build/rootfs/peak-native

# Copy assets directory for icons, fonts, etc.
echo "Copying assets..."
mkdir -p /build/rootfs/usr/share/peakos/assets
cp -r /project/peak-native/assets/* /build/rootfs/usr/share/peakos/assets/
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

# Install Base System + Kernel + Input Devices
# wireless-tools/wpa_supplicant needed for wifi?
apk --root /build/rootfs --initdb add --arch "$APK_ARCH" --no-cache --allow-untrusted \
    alpine-base linux-lts linux-firmware-none \
    udev eudev libinput libinput-dev \
    alsa-lib wayland mesa-dri-gallium \
    weston weston-backend-drm seatd \
    webkit2gtk-4.1 \
    adwaita-icon-theme \
    ttf-dejavu font-noto font-noto-cjk \
    dbus networkmanager networkmanager-cli networkmanager-wifi wpa_supplicant \
    firefox ca-certificates fbida-fbi




# --- Configuration Phase ---
echo "--- Configuring Startup ---"

# 0. Regenerate initramfs with VirtIO drivers
echo "--- Regenerating Initramfs ---"
echo 'features="base keymap kms mmc network virtio"' > /build/rootfs/etc/mkinitfs/mkinitfs.conf
# We must chroot to run mkinitfs correctly or use the host tool targeting the directory
# On Alpine Docker, we can use the host tool:
mkinitfs -n -b /build/rootfs -c /build/rootfs/etc/mkinitfs/mkinitfs.conf -o /build/rootfs/boot/initramfs-lts $(ls /build/rootfs/lib/modules/)

# 1. Weston Configuration (Auto-launch Peak)
mkdir -p /build/rootfs/etc/xdg/weston
cat > /build/rootfs/etc/xdg/weston/weston.ini <<EOF
[core]
backend=drm-backend.so
shell=desktop-shell.so
idle-time=0

[shell]
locking=false
panel-position=none
background-color=0xff000000
cursor-theme=Adwaita
cursor-size=24

# [autolaunch] is not supported by desktop-shell, we launch manually in init_peak.sh
EOF

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
export WESTON_BACKEND=drm-backend.so

# Cursor Configuration
export XCURSOR_THEME=Adwaita
export XCURSOR_SIZE=24

# Start Seat Management Daemon
seatd &
# Wait for seatd and GPU drivers to stabilize
sleep 3

# Launch Weston (Wayland Compositor)
# We run as root for now (simplicity)
# Run in background, log to file
weston --continue-without-input --log=/var/log/weston.log &

# Wait for Wayland socket to appear
echo "Waiting for Wayland socket..."
for i in \$(seq 1 10); do
    if [ -e \$XDG_RUNTIME_DIR/wayland-0 ]; then
        echo "Wayland socket found!"
        break
    fi
    sleep 1
done

# Launch PeakOS Native App
echo "Launching PeakOS..."
# Export display explicitly just in case
export WAYLAND_DISPLAY=wayland-0
/peak-native &

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
menuentry "PeakOS ($ARCH)" {
    search --no-floppy --set=root --label PeakOS
    linux /boot/vmlinuz root=/dev/ram0 rdinit=/init console=$SERIAL_CONSOLE console=tty0 $EARLYCON debug keep_bootcon
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
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

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
