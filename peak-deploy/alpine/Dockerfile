FROM alpine:edge

# Install Build Tools and Dependencies
RUN apk update && apk add --no-cache \
    # Build System
    bash git make gcc musl-dev pkgconf \
    rust cargo \
    xorriso grub-efi mtools doas \
    # Runtime Deps (for compilation linking)
    alsa-lib-dev wayland-dev wayland-protocols \
    libxkbcommon-dev fontconfig-dev \
    gtk+3.0-dev webkit2gtk-4.1-dev \
    openssl-dev glib-dev \
    # ISO Creation Tools
    dosfstools libblkid

# Setup Build Directory
WORKDIR /build

# Copy Build Scripts
COPY gen_iso.sh /build/gen_iso.sh
RUN chmod +x /build/gen_iso.sh

# Default entrypoint
CMD ["/build/gen_iso.sh"]
