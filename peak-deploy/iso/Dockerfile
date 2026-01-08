FROM archlinux:latest

# Update system and install archiso + build tools
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm archiso git make base-devel \
    rust cargo nodejs npm wget \
    webkit2gtk-4.1 openssl glib2 libsoup3 appmenu-gtk-module libappindicator-gtk3 librsvg \
    grub syslinux efibootmgr dosfstools mtools

# Set working directory
WORKDIR /build

# Default command
CMD ["/bin/bash", "/build/build_internal.sh"]
