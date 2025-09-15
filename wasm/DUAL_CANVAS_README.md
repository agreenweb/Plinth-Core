# Dual Canvas WASM Test

This enhanced WASM test now supports dual canvas rendering to test whether the same WASM module can render to multiple canvases simultaneously.

## Features Added

### 1. Python Server Script
- **File**: `start_server.py`
- **Purpose**: Launches a Python HTTP server on port 8000 and opens the browser
- **Features**:
  - Automatically kills any existing server on port 8000
  - Sets proper CORS headers for WASM files
  - Opens browser automatically
  - Handles MIME types correctly for .wasm and .js files

### 2. Shell Script Alternative
- **File**: `start_server.sh`
- **Purpose**: Alternative shell script for launching the server
- **Features**: Same as Python script but uses shell commands

### 3. Dual Canvas Rendering
- **HTML**: Modified `index.html` to show two canvases side by side
- **JavaScript**: Updated to create two separate WASM app instances
- **Canvas Size**: Each canvas is 400x300 pixels (reduced from 800x600 to fit both)
- **Rendering**: Both canvases render the same content simultaneously

## Usage

### Option 1: Python Script
```bash
cd wasm
python3 start_server.py
```

### Option 2: Shell Script
```bash
cd wasm
./start_server.sh
```

### Option 3: Manual Python Server
```bash
cd wasm
python3 -m http.server 8000
# Then open http://localhost:8000 in your browser
```

## What You Should See

1. **Two identical canvases** showing colorful triangles with red, green, and blue gradients
2. **Synchronized rendering** - both canvases should update at the same time
3. **Independent WASM instances** - each canvas has its own WASM app instance
4. **Same content** - both canvases render identical content from the same WASM module

## Technical Details

- **Canvas Size**: 400x300 pixels each (2 canvases total)
- **Rendering**: WebGPU with WGSL shaders
- **WASM**: Rust compiled to WebAssembly
- **Frame Rate**: 60 FPS (browser dependent)
- **Dual Rendering**: Same WASM module rendering to both canvases simultaneously

## Testing Purpose

This setup tests whether:
1. The same WASM module can be instantiated multiple times
2. Multiple WebGPU contexts can coexist
3. The rendering pipeline works correctly with multiple canvases
4. There are any resource conflicts or performance issues with dual rendering

## Stopping the Server

Press `Ctrl+C` in the terminal where the server is running to stop it.
