#!/usr/bin/env python3
"""
Script to start a Python HTTP server for the WASM test and open it in the browser.
This script will:
1. Kill any existing Python server on port 8000
2. Start a new Python HTTP server on port 8000
3. Open the browser to localhost:8000
"""

import http.server
import socketserver
import webbrowser
import subprocess
import sys
import os
import signal
import time
from pathlib import Path

PORT = 8000
HOST = 'localhost'

def kill_existing_server():
    """Kill any existing Python server on port 8000"""
    try:
        # Find processes using port 8000
        result = subprocess.run(['lsof', '-ti', f':{PORT}'], 
                              capture_output=True, text=True)
        if result.returncode == 0 and result.stdout.strip():
            pids = result.stdout.strip().split('\n')
            for pid in pids:
                if pid:
                    print(f"Killing existing process on port {PORT}: PID {pid}")
                    os.kill(int(pid), signal.SIGTERM)
                    time.sleep(1)  # Give it time to die
    except Exception as e:
        print(f"Note: Could not check for existing processes: {e}")

def start_server():
    """Start the Python HTTP server"""
    # Change to the wasm directory
    wasm_dir = Path(__file__).parent
    os.chdir(wasm_dir)
    
    # Kill any existing server
    kill_existing_server()
    
    # Create a custom handler that serves files with correct MIME types
    class CustomHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
        def end_headers(self):
            # Add CORS headers for WASM
            self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
            self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
            super().end_headers()
        
        def guess_type(self, path):
            """Override to set correct MIME types for WASM files"""
            mimetype = super().guess_type(path)
            if path.endswith('.wasm'):
                return 'application/wasm'
            elif path.endswith('.js'):
                return 'application/javascript'
            return mimetype
    
    try:
        with socketserver.TCPServer((HOST, PORT), CustomHTTPRequestHandler) as httpd:
            print(f"Starting Python HTTP server on {HOST}:{PORT}")
            print(f"Serving files from: {wasm_dir}")
            print(f"Open your browser to: http://{HOST}:{PORT}")
            print("Press Ctrl+C to stop the server")
            
            # Open browser
            webbrowser.open(f'http://{HOST}:{PORT}')
            
            # Start serving
            httpd.serve_forever()
            
    except KeyboardInterrupt:
        print("\nServer stopped by user")
    except OSError as e:
        if e.errno == 48:  # Address already in use
            print(f"Port {PORT} is still in use. Trying to kill processes and retry...")
            kill_existing_server()
            time.sleep(2)
            start_server()  # Retry once
        else:
            print(f"Error starting server: {e}")
            sys.exit(1)

if __name__ == "__main__":
    start_server()
