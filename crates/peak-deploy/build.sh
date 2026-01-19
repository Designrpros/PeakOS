#!/bin/bash
set -e

# =============================================================================
# PeakOS Build Script
# =============================================================================
# Usage:
#   ./build.sh --arm          # ARM: Native on ARM device, Docker on x86
#   ./build.sh --intel        # Intel: Always uses Docker (x86_64 emulation)
#   ./build.sh --native       # Force native build (no Docker)
# =============================================================================

# Configuration
IMAGE_NAME="peak-alpine-builder"
DEPLOY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ISO_DIR="$DEPLOY_DIR/alpine"
OUT_DIR="$DEPLOY_DIR/out"
PROJECT_ROOT="$DEPLOY_DIR/../.."

# Ensure directories exist
mkdir -p "$OUT_DIR"

# Detect host architecture
HOST_ARCH=$(uname -m)
echo "Host architecture: $HOST_ARCH"

# Parse arguments
BUILD_MODE="auto"
TARGET_ARCH="arm"

for arg in "$@"; do
    case $arg in
        --arm)
            TARGET_ARCH="arm"
            ;;
        --intel|--x86)
            TARGET_ARCH="intel"
            ;;
        --native)
            BUILD_MODE="native"
            ;;
        --docker)
            BUILD_MODE="docker"
            ;;
        *)
            echo "Unknown argument: $arg"
            echo "Usage: ./build.sh [--arm|--intel] [--native|--docker]"
            exit 1
            ;;
    esac
done

# Determine build strategy
if [[ "$BUILD_MODE" == "auto" ]]; then
    if [[ "$TARGET_ARCH" == "arm" && "$HOST_ARCH" == "aarch64" ]]; then
        BUILD_MODE="native"
        echo "ARM on ARM host → Using native build (fast)"
    elif [[ "$TARGET_ARCH" == "intel" && "$HOST_ARCH" == "x86_64" ]]; then
        BUILD_MODE="native"
        echo "Intel on Intel host → Using native build (fast)"
    else
        BUILD_MODE="docker"
        echo "Cross-architecture → Using Docker build"
    fi
fi

echo "Target: $TARGET_ARCH | Mode: $BUILD_MODE"
echo "==========================================="

# =============================================================================
# Pre-Build Validation (Fail Fast)
# =============================================================================
echo ""
echo "Running pre-build checks..."
if ! bash "$DEPLOY_DIR/pre-build-check.sh"; then
    echo ""
    echo "❌ Pre-build validation failed. Fix errors above."
    exit 1
fi
echo ""

# =============================================================================
# Native Build (for running on actual ARM/Intel Linux servers)
# =============================================================================
native_build() {
    echo "Starting native build..."
    
    # Check if we're on Alpine
    if [[ -f /etc/alpine-release ]]; then
        echo "Detected Alpine Linux - installing dependencies..."
        apk add --no-cache \
            bash git make gcc g++ musl-dev pkgconf \
            clang clang-dev llvm-dev lld cmake perl linux-headers curl \
            xorriso grub-efi mtools doas \
            alsa-lib-dev wayland-dev wayland-protocols \
            libxkbcommon-dev fontconfig-dev freetype-dev \
            gtk+3.0-dev webkit2gtk-4.1-dev \
            openssl-dev openssl-libs-static glib-dev mesa-dev zlib-dev libpng-dev jpeg-dev \
            dosfstools libblkid file \
            alpine-base linux-lts linux-firmware-none \
            labwc seatd webkit2gtk-4.1 adwaita-icon-theme \
            ttf-dejavu font-noto font-noto-cjk \
            dbus networkmanager networkmanager-cli networkmanager-wifi wpa_supplicant \
            firefox ca-certificates bluez bluez-tools alsa-utils
    fi
    
    # Install Rust if not present
    if ! command -v cargo &> /dev/null; then
        echo "Installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        source "$HOME/.cargo/env"
    fi
    
    # Set environment
    export LIBCLANG_PATH="/usr/lib"
    export OPENSSL_STATIC=1
    export OPENSSL_LIB_DIR=/usr/lib
    export OPENSSL_INCLUDE_DIR=/usr/include
    
    # Run the ISO generation script
    cd "$ISO_DIR"
    ./gen_iso.sh
    
    echo "Native build complete!"
}

# =============================================================================
# Docker Build (for cross-compilation or macOS)
# =============================================================================
docker_build() {
    PLATFORM_FLAG=""
    if [[ "$TARGET_ARCH" == "intel" ]]; then
        echo "Building for Intel (x86_64) via Docker..."
        PLATFORM_FLAG="--platform linux/amd64"
    else
        echo "Building for ARM (aarch64) via Docker..."
        PLATFORM_FLAG="--platform linux/arm64"
    fi
    
    # Build Docker Image
    echo "Building Docker image..."
    docker build $PLATFORM_FLAG --load -t $IMAGE_NAME "$ISO_DIR"
    
    echo "Starting Docker build process..."
    docker run --rm --privileged $PLATFORM_FLAG \
        -v "$ISO_DIR:/build" \
        -v "$PROJECT_ROOT":/project \
        -v /project/target \
        -v "$OUT_DIR:/out" \
        $IMAGE_NAME
    
    # Fix permissions
    echo "Fixing permissions..."
    docker run --rm \
        -v "$OUT_DIR:/out" \
        alpine \
        chown -R $(id -u):$(id -g) /out
    
    echo "Docker build complete!"
}

# =============================================================================
# Main
# =============================================================================
if [[ "$BUILD_MODE" == "native" ]]; then
    native_build
else
    docker_build
fi

echo ""
echo "==========================================="
echo "Build Complete!"
echo "ISO is in: $OUT_DIR"
echo "==========================================="
ls -lh "$OUT_DIR"/*.iso 2>/dev/null || echo "(No ISO found - check logs above)"
