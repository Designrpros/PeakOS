FROM alpine:edge

# 2. Install Build Dependencies
RUN apk update && apk add --no-cache \
    bash git make gcc g++ musl-dev pkgconf \
    clang lld cmake perl linux-headers curl \
    xorriso grub-efi mtools doas \
    alsa-lib-dev wayland-dev wayland-protocols \
    libxkbcommon-dev fontconfig-dev freetype-dev \
    gtk+3.0-dev webkit2gtk-4.1-dev \
    openssl-dev glib-dev mesa-dev zlib-dev libpng-dev jpeg-dev \
    dosfstools libblkid file \
    rust cargo
ENV PATH="/root/.cargo/bin:${PATH}"

# Setup Build Directory
WORKDIR /build

# Copy Build Scripts
COPY gen_iso.sh /build/gen_iso.sh
RUN chmod +x /build/gen_iso.sh

# Default entrypoint
CMD ["/build/gen_iso.sh"]
