#!/bin/bash
# Dockerfile Testing Script
# Tests that the Dockerfile builds successfully and produces expected outputs

set -e

echo "=== Dockerfile Build Tests ==="

# Test 1: Dockerfile exists and is valid syntax
test_dockerfile_exists() {
    echo "Test 1: Checking Dockerfile exists..."
    if [ -f "Dockerfile" ]; then
        echo "✅ Dockerfile exists"
    else
        echo "❌ Dockerfile not found"
        exit 1
    fi
}

# Test 2: Dockerfile linting (using hadolint if available)
test_dockerfile_lint() {
    echo "Test 2: Linting Dockerfile..."
    if command -v hadolint &> /dev/null; then
        hadolint Dockerfile || echo "⚠️  Linting warnings (non-fatal)"
    else
        echo "⚠️  hadolint not installed (run: brew install hadolint)"
    fi
}

# Test 3: Build the image
test_dockerfile_build() {
    echo "Test 3: Building Docker image..."
    docker build --no-cache -t peak-alpine-builder-test . || {
        echo "❌ Docker build failed"
        exit 1
    }
    echo "✅ Docker image built successfully"
}

# Test 4: Verify Rust is installed
test_rust_installed() {
    echo "Test 4: Verifying Rust installation..."
    docker run --rm peak-alpine-builder-test rustc --version || {
        echo "❌ Rust not installed in image"
        exit 1
    }
    echo "✅ Rust installed and working"
}

# Test 5: Verify required tools
test_required_tools() {
    echo "Test 5: Checking required build tools..."
    TOOLS=("gcc" "clang" "xorriso" "grub-mkrescue" "git")
    
    for tool in "${TOOLS[@]}"; do
        docker run --rm peak-alpine-builder-test which "$tool" > /dev/null || {
            echo "❌ Missing tool: $tool"
            exit 1
        }
    done
    echo "✅ All required tools present"
}

# Test 6: Check image size
test_image_size() {
    echo "Test 6: Checking image size..."
    SIZE=$(docker images peak-alpine-builder-test --format "{{.Size}}")
    echo "📊 Image size: $SIZE"
    
    # Warn if image is suspiciously large (>3GB suggests bloat)
    SIZE_MB=$(docker images peak-alpine-builder-test --format "{{.Size}}" | sed 's/GB/*1024/;s/MB//' | bc 2>/dev/null || echo "0")
    if (( $(echo "$SIZE_MB > 3072" | bc -l 2>/dev/null || echo 0) )); then
        echo "⚠️  Image is large (>3GB), consider optimization"
    else
        echo "✅ Image size reasonable"
    fi
}

# Test 7: Layer count
test_layer_count() {
    echo "Test 7: Checking layer count..."
    LAYERS=$(docker history peak-alpine-builder-test | wc -l)
    echo "📊 Layer count: $LAYERS"
    
    if [ "$LAYERS" -gt 20 ]; then
        echo "⚠️  High layer count ($LAYERS), consider combining RUN commands"
    else
        echo "✅ Layer count reasonable"
    fi
}

# Run all tests
test_dockerfile_exists
test_dockerfile_lint
test_dockerfile_build
test_rust_installed
test_required_tools
test_image_size
test_layer_count

echo ""
echo "=== All Tests Passed! ==="
echo "Cleanup: docker rmi peak-alpine-builder-test"
