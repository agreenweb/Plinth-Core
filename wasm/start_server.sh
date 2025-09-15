#!/bin/bash

# Script to start a Python HTTP server for the WASM test and open it in the browser.
# This script will:
# 1. Kill any existing Python server on port 8000
# 2. Start a new Python HTTP server on port 8000
# 3. Open the browser to localhost:8000

PORT=8000
HOST=localhost

echo "Starting WASM test server..."

# Kill any existing processes on port 8000
echo "Checking for existing processes on port $PORT..."
if lsof -ti :$PORT > /dev/null 2>&1; then
    echo "Killing existing processes on port $PORT..."
    lsof -ti :$PORT | xargs kill -9 2>/dev/null || true
    sleep 2
fi

# Change to the wasm directory
cd "$(dirname "$0")"

# Start the Python server
echo "Starting Python HTTP server on $HOST:$PORT..."
echo "Serving files from: $(pwd)"
echo "Open your browser to: http://$HOST:$PORT"
echo "Press Ctrl+C to stop the server"

# Open browser in background
if command -v open >/dev/null 2>&1; then
    # macOS
    open "http://$HOST:$PORT" &
elif command -v xdg-open >/dev/null 2>&1; then
    # Linux
    xdg-open "http://$HOST:$PORT" &
elif command -v start >/dev/null 2>&1; then
    # Windows
    start "http://$HOST:$PORT" &
fi

# Start the server with proper CORS headers for WASM
python3 -c "
import http.server
import socketserver
import os

class CustomHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add CORS headers for WASM
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        super().end_headers()
    
    def guess_type(self, path):
        mimetype = super().guess_type(path)
        if path.endswith('.wasm'):
            return 'application/wasm'
        elif path.endswith('.js'):
            return 'application/javascript'
        return mimetype

with socketserver.TCPServer(('$HOST', $PORT), CustomHTTPRequestHandler) as httpd:
    httpd.serve_forever()
"
