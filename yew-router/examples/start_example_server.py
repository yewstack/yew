#!/usr/bin/env python3
from http.server import BaseHTTPRequestHandler, HTTPServer

import os
import sys
import logging
import webbrowser

PORT=8080
STATIC_RESOURCES = { "/wasm.js", "/wasm_bg.wasm", "/favicon.ico" }

class S(BaseHTTPRequestHandler):
    def _set_response(self, code, mime='text/html; charset=utf-8'):
        self.send_response(code)
        self.send_header('Content-type', mime)
        self.send_header('Cache-Control', 'private, max-age=0, must-revalidate')
        self.end_headers()

    def do_GET(self):
        if self.path in STATIC_RESOURCES:
            resolved_path = "." + self.path
            if os.path.isfile(resolved_path):
                with open(resolved_path, "rb") as f:
                    mime='text/html; charset=utf-8'
                    if resolved_path.endswith('.wasm'):
                        mime = 'application/wasm'
                    elif resolved_path.endswith('.js'):
                        mime = 'text/javascript'
                    self._set_response(200, mime)
                    self.wfile.write(f.read())
            else:
                self._set_response(404)
                self.wfile.write(b"404 file not found")
        else:
            self._set_response(200)
            with open("index.html", "rb") as f:
                self.wfile.write(f.read())

def run(server_class=HTTPServer, handler_class=S, initial_url=''):
    logging.basicConfig(level=logging.INFO)
    server_address = ('', PORT)
    httpd = server_class(server_address, handler_class)
    webbrowser.open(f"http://localhost:{PORT}/{initial_url}")
    logging.info('Starting web server...\n')
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass
    httpd.server_close()
    logging.info('Stopping web server...\n')

if __name__ == '__main__':
    from sys import argv

    run(initial_url=sys.argv[1] if len(sys.argv) > 1 else '')
