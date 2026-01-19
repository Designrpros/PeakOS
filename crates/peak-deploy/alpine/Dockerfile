FROM alpine:latest

# 2. Install Build Dependencies
RUN apk add --no-cache bash git make gcc g++ musl-dev pkgconf
RUN apk add --no-cache clang clang-dev llvm-dev lld cmake perl linux-headers curl
RUN apk add --no-cache xorriso grub-efi mtools doas
RUN apk add --no-cache alsa-lib-dev wayland-dev wayland-protocols libxkbcommon-dev
RUN apk add --no-cache fontconfig-dev freetype-dev gtk+3.0-dev webkit2gtk-4.1-dev \
    harfbuzz-dev cairo-dev gdk-pixbuf-dev pango-dev glib-dev
RUN apk add --no-cache openssl-dev openssl-libs-static glib-static mesa-dev zlib-dev libpng-dev jpeg-dev
RUN apk add --no-cache gobject-introspection-dev dosfstools libblkid file

# Install Rust via rustup for better compatibility
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

ENV LIBCLANG_PATH="/usr/lib"
# Use dynamic linking for OpenSSL to match system
ENV OPENSSL_STATIC=0
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

# Setup Build Directory
WORKDIR /build

# Copy Build Scripts
COPY gen_iso.sh /build/gen_iso.sh
RUN chmod +x /build/gen_iso.sh

# Default entrypoint
CMD ["/build/gen_iso.sh"]
