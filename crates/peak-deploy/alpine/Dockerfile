FROM alpine:3.21

# Install Build Tools and Dependencies
RUN apk update && apk add --no-cache \
    # Build System
    bash git make gcc g++ musl-dev pkgconf \
    clang lld cmake perl linux-headers curl \
    xorriso grub-efi mtools doas \
    # Runtime Deps (for compilation linking)
    alsa-lib-dev wayland-dev wayland-protocols \
    libxkbcommon-dev fontconfig-dev freetype-dev \
    gtk+3.0-dev webkit2gtk-4.1-dev \
    openssl-dev glib-dev mesa-dev zlib-dev libpng-dev jpeg-dev \
    # ISO Creation Tools
    dosfstools libblkid file

# Install Rust via rustup for latest stable version
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# Setup Build Directory
WORKDIR /build

# Copy Build Scripts
COPY gen_iso.sh /build/gen_iso.sh
RUN chmod +x /build/gen_iso.sh

# Default entrypoint
CMD ["/build/gen_iso.sh"]
