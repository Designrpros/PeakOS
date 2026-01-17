#!/bin/bash
set -e

# Configuration
IMAGE_NAME="peak-alpine-builder"
# Directory containing this script (peak-deploy)
DEPLOY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Directory containing Dockerfile and alpine config
ISO_DIR="$DEPLOY_DIR/alpine"
# Output directory
OUT_DIR="$DEPLOY_DIR/out"

# Ensure directories exist
mkdir -p "$OUT_DIR"

# Parse arguments
PLATFORM_FLAG=""
if [[ "$1" == "--intel" ]] || [[ "$1" == "--x86" ]]; then
    echo "Building for Intel (x86_64)..."
    PLATFORM_FLAG="--platform linux/amd64"
fi

# Build Docker Image
echo "Building Docker image..."
docker build $PLATFORM_FLAG -t $IMAGE_NAME "$ISO_DIR"

echo "Starting Alpine Build Process..."
docker run --rm --privileged $PLATFORM_FLAG \
    -v "$ISO_DIR:/build" \
    -v "$DEPLOY_DIR/../../":/project \
    -v "$OUT_DIR:/out" \
    $IMAGE_NAME

# Fix permissions
echo "Fixing permissions..."
docker run --rm \
    -v "$OUT_DIR:/out" \
    alpine \
    chown -R $(id -u):$(id -g) /out

echo "Build Complete! ISO is in $OUT_DIR"
