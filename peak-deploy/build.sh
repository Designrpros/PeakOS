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

# Build Docker Image
# We DO NOT force platform here. We let it build for the HOST architecture (ARM64 on M1).
echo "Building Docker image (Native Architecture)..."
docker build -t $IMAGE_NAME "$ISO_DIR"

echo "Starting Alpine Build Process..."
docker run --rm --privileged \
    -v "$ISO_DIR:/build" \
    -v "$DEPLOY_DIR/../":/project \
    -v "$OUT_DIR:/out" \
    -v "peak-cargo-cache-alpine:/root/.cargo" \
    $IMAGE_NAME

# Fix permissions
echo "Fixing permissions..."
docker run --rm \
    -v "$OUT_DIR:/out" \
    alpine \
    chown -R $(id -u):$(id -g) /out

echo "Build Complete! ISO is in $OUT_DIR"
