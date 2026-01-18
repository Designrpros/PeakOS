FROM alpine:edge

# 2. Install Build Dependencies
RUN apk update && apk add --no-cache \
    bash git make gcc g++ musl-dev pkgconf \
    clang clang-dev llvm-dev lld cmake perl linux-headers curl \
    xorriso grub-efi mtools doas \
    alsa-lib-dev wayland-dev wayland-protocols \
    libxkbcommon-dev fontconfig-dev freetype-dev \
    gtk+3.0-dev webkit2gtk-4.1-dev \
    openssl-dev openssl-libs-static glib-dev mesa-dev zlib-dev libpng-dev jpeg-dev \
    dosfstools libblkid file \
    rust cargo

ENV PATH="/root/.cargo/bin:${PATH}"
ENV LIBCLANG_PATH="/usr/lib"
# Use static OpenSSL linking
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

# Setup Build Directory
WORKDIR /build

# Copy Build Scripts
COPY gen_iso.sh /build/gen_iso.sh
RUN chmod +x /build/gen_iso.sh

# Default entrypoint
CMD ["/build/gen_iso.sh"]
