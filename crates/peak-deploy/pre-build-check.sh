#!/bin/bash
# Pre-Build Validation Script
# Runs fast local checks BEFORE expensive Docker builds
# Fails fast if code has issues

set -e

echo "=========================================="
echo "🔍 PeakOS Pre-Build Validation"
echo "=========================================="

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$PROJECT_ROOT"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track failures
FAILED=0

# Test 1: Cargo Check (syntax & type errors)
echo ""
echo "📋 Test 1/4: Checking for syntax and type errors..."
if cargo check --all-targets 2>&1 | tee /tmp/cargo-check.log; then
    echo -e "${GREEN}✅ No syntax or type errors${NC}"
else
    echo -e "${RED}❌ Compilation errors found!${NC}"
    echo "See /tmp/cargo-check.log for details"
    FAILED=1
fi

# Test 2: Clippy Lints (code quality)
echo ""
echo "📋 Test 2/4: Running clippy linter..."
if cargo clippy --all-targets -- -D warnings 2>&1 | tee /tmp/cargo-clippy.log; then
    echo -e "${GREEN}✅ No linting issues${NC}"
else
    echo -e "${YELLOW}⚠️  Clippy warnings found (non-fatal)${NC}"
    # Don't fail on clippy warnings, just inform
fi

# Test 3: Test Compilation (verify tests build)
echo ""
echo "📋 Test 3/4: Verifying tests compile..."
if cargo test --no-run 2>&1 | tee /tmp/cargo-test.log; then
    echo -e "${GREEN}✅ Tests compile successfully${NC}"
else
    echo -e "${RED}❌ Test compilation failed!${NC}"
    FAILED=1
fi

# Test 4: Dependency Check
echo ""
echo "📋 Test 4/4: Checking for unused dependencies..."
if command -v cargo-udeps &> /dev/null; then
    if cargo +nightly udeps --all-targets 2>&1 | grep -q "unused"; then
        echo -e "${YELLOW}⚠️  Unused dependencies detected (consider cleanup)${NC}"
    else
        echo -e "${GREEN}✅ No unused dependencies${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  cargo-udeps not installed (skipping)${NC}"
    echo "   Install with: cargo install cargo-udeps"
fi

# Summary
echo ""
echo "=========================================="
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All pre-build checks passed!${NC}"
    echo "=========================================="
    echo ""
    echo "Proceeding with ISO build..."
    exit 0
else
    echo -e "${RED}❌ Pre-build validation FAILED${NC}"
    echo "=========================================="
    echo ""
    echo "Fix the errors above before building ISO."
    echo "This saved you ~20 minutes of Docker build time! 🎯"
    exit 1
fi
