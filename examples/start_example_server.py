from http.server import BaseHTTPRequestHandler, HTTPServer

import os
import sys
import logging
import webbrowser

PORT = 8080
STATIC_RESOURCES = {"/wasm.js", "/wasm_bg.wasm", "/favicon.ico"}
DEFAULT_INDEX_HTML = b"""
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Yew example</title>
        <script type="module">
            import init from "./wasm.js"
            init()
        </script>
    </head>
    <body></body>
</html>
"""


def build_content_type(mime: str, charset: str = None) -> str:
    if not charset and mime.startswith("text/"):
        charset = "UTF-8"

    if charset:
        return f"{mime};charset={charset}"
    else:
        return mime


class S(BaseHTTPRequestHandler):
    def _set_response(self, code: int, mime: str, charset: str = None) -> None:
        self.send_response(code)
        self.send_header(
            "Content-type", build_content_type(mime, charset=charset))
        self.send_header(
            "Cache-Control", "private, max-age=0, must-revalidate")
        self.end_headers()

    def do_GET(self):
        path = self.path
        if path in ("", "/"):
            path = "/index.html"
        relative_path = f".{path}"

        if os.path.isfile(relative_path):
            logging.info("is a file: %s", relative_path)
            with open(relative_path, "rb") as f:
                mime = "text/plain"
                if relative_path.endswith(".html"):
                    mime = "text/html"
                elif relative_path.endswith(".wasm"):
                    mime = "application/wasm"
                elif relative_path.endswith(".js"):
                    mime = "text/javascript"
                elif relative_path.endswith(".json"):
                    mime = "application/json"
                elif relative_path.endswith(".css"):
                    mime = "text/css"
                elif relative_path.endswith(".toml"):
                    mime = "application/toml"
                self._set_response(200, mime)
                self.wfile.write(f.read())
        elif path == "/index.html":
            self._set_response(200, "text/html")
            self.wfile.write(DEFAULT_INDEX_HTML)
        else:
            self._set_response(404, mime="text/plain")
            self.wfile.write(b"404 file not found")


def run(server_class=HTTPServer, handler_class=S):
    logging.basicConfig(level=logging.INFO)
    server_address = ("", PORT)
    httpd = server_class(server_address, handler_class)
    url = f"http://localhost:{PORT}"
    logging.info("Starting web server at %s", url)
    logging.info("Use CTRL+C to stop\n")
    webbrowser.open(url)
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass
    httpd.server_close()
    logging.info("Stopping web server...\n")


if __name__ == "__main__":
    from sys import argv

    run()
