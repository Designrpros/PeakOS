#!/bin/bash
set -e

# Configuration
# Configuration
IMAGE_NAME="peak-iso-builder"
# Directory containing this script (peak-deploy)
DEPLOY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Directory containing Dockerfile and iso config (peak-deploy/iso)
ISO_DIR="$DEPLOY_DIR/iso"
WORK_DIR="$ISO_DIR/work"
OUT_DIR="$ISO_DIR/out"

# Ensure directories exist
mkdir -p "$WORK_DIR" "$OUT_DIR"

# Build Docker Image
# We force linux/amd64 because we are building an x86_64 ISO for an Intel Mac
echo "Building Docker image (linux/amd64)..."
docker build --platform linux/amd64 -t $IMAGE_NAME "$ISO_DIR"

echo "Starting Build Process..."
# Archiso requires privileged mode to mount loop devices
docker run --rm --privileged --platform linux/amd64 \
    -v "$ISO_DIR:/build" \
    -v "$DEPLOY_DIR/../":/project \
    -v "$OUT_DIR:/out" \
    $IMAGE_NAME
