#!/bin/bash

# Build script for WASM target
set -e

echo "Building WASM target..."

# Build the WASM module
wasm-pack build --target web --out-dir pkg

echo "Build complete! You can now serve the wasm directory with a web server."
echo "For example:"
echo "  cd wasm && python3 -m http.server 8000"
echo "  or"
echo "  cd wasm && npx serve ."
echo ""
echo "Then open http://localhost:8000 in your browser"
