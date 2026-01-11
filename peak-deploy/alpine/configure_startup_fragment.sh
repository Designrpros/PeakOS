
# --- Configuration Phase ---
echo "--- Configuring Startup ---"

# 1. Weston Configuration (Auto-launch Peak)
mkdir -p /build/rootfs/etc/xdg/weston
cat > /build/rootfs/etc/xdg/weston/weston.ini <<EOF
[core]
backend=drm-backend.so
shell=kiosk-shell.so
idle-time=0

[shell]
locking=false
panel-position=none

[autolaunch]
path=/bin/peak-native
EOF

# 2. Startup Script
cat > /build/rootfs/init_peak.sh <<EOF
#!/bin/sh
echo "--- PeakOS Starting ---"
# Load Drivers
modprobe virtio-gpu
modprobe virtio-input
modprobe evdev

# Setup Environment
export XDG_RUNTIME_DIR=/tmp/xdg
mkdir -p \$XDG_RUNTIME_DIR
chmod 0700 \$XDG_RUNTIME_DIR

# Start Hardware Daemon (Seat management)
seatd -d

# Start Composition
echo "Starting Weston..."
# We run as root for now (simplicity)
weston --tty=1 --log=/var/log/weston.log
EOF
chmod +x /build/rootfs/init_peak.sh

# 3. Modify Inittab (Auto-login/start)
# Remove standard gettys
sed -i 's/^tty/#tty/g' /build/rootfs/etc/inittab
# Add our startup
echo "tty1::respawn:/init_peak.sh" >> /build/rootfs/etc/inittab
