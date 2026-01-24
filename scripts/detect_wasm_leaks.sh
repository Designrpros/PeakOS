#!/bin/bash

# Script to detect potential WASM 'static lifetime leaks in PeakUI
# These are captures that prevent closures from being 'static

SEARCH_DIR="/Users/vegarberentsen/Documents/PeakOS/crates/peak-ui/src"

echo "Checking for potential WASM compatibility issues (lifetime leaks)..."
echo "=================================================================="

# 1. Look for direct use of 'context.' or 'theme.' inside style closures
echo "1. Style Closures with direct Theme/Context access:"
grep -rE "\.style\(move \|.*\|.*(theme|context)\.[a-zA-Z0-9_\.]+" "$SEARCH_DIR" | grep -vE "let [a-zA-Z0-9_]+ = (theme|context)"

# 2. Look for .map closures which often capture self or references
echo -e "\n2. .map() closures (potential reference captures):"
grep -rE "\.map\(move \|.*\|" "$SEARCH_DIR" | grep -v "'static"

# 3. Reference Captures
echo -e "\n3. Reference Captures (look for &theme or &context being stored):"
grep -rE "let [a-zA-Z0-9_]+ = &(theme|context)" "$SEARCH_DIR"

# 4. WASM-Sensitive Widgets
echo -e "\n4. WASM-Sensitive Widgets (Canvas/Stack/Shadow):"
grep -rE "Canvas::new|stack!\[|Shadow" "$SEARCH_DIR"

# 4. Check for lifetime markers that might indicate non-'static usage
echo -e "\n4. Explicit lifetimes in UI elements (suspicious for WASM):"
grep -rE "Element<'a,|View<'a," "$SEARCH_DIR"

echo -e "\n=================================================================="
echo "WASM compatibility check complete."
