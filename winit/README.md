# Winit Test

This is a test application for the winit-based rendering in Plinth Core.

## Running the test

```bash
cd winit
cargo run
```

This will open a window displaying a colorful triangle rendered with WebGPU. The triangle should show red, green, and blue gradients.

## What it tests

- Winit window creation
- WebGPU context initialization
- WGSL shader compilation and execution
- Basic rendering pipeline
- Event loop integration

## Expected output

You should see a window with a colorful triangle that renders at 60 FPS. The console will show frame count updates every 60 frames.
