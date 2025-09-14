# WASM Test

This is a test application for the WebAssembly-based rendering in Plinth Core.

## Building the WASM module

```bash
cd wasm
./build.sh
```

This will compile the Rust code to WebAssembly using `wasm-pack`.

## Running the test

After building, serve the directory with a web server:

```bash
# Option 1: Python HTTP server
python3 -m http.server 8000

# Option 2: Node.js serve
npx serve .

# Option 3: Any other static file server
```

Then open http://localhost:8000 in your browser.

## What it tests

- WebAssembly compilation
- WebGPU context initialization in the browser
- Canvas element integration
- WGSL shader compilation and execution
- Web-based rendering pipeline
- Animation frame loop

## Expected output

You should see a web page with a canvas displaying a colorful triangle. The triangle should render with red, green, and blue gradients using WebGPU.

## Browser requirements

- Modern browser with WebGPU support (Chrome 113+, Firefox 110+)
- WebAssembly support
- Canvas 2D context support

## Troubleshooting

If you see errors:
1. Check browser console for error messages
2. Ensure your browser supports WebGPU
3. Try a different browser if issues persist
4. Check that the WASM module was built successfully
