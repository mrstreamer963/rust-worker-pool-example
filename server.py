#!/usr/bin/env python3
"""
HTTP-сервер с поддержкой правильных MIME-типов для WebAssembly
"""
import http.server
import socketserver
from functools import partial
import os

class CustomHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
    
    def guess_type(self, path):
        if path.endswith('.wasm'):
            return 'application/wasm'
        if path.endswith('.js'):
            return 'application/javascript'
        if path.endswith('.html'):
            return 'text/html'
        if path.endswith('.css'):
            return 'text/css'
        if path.endswith('.json'):
            return 'application/json'
        return super().guess_type(path)
    
    def end_headers(self):
        # Добавляем заголовки для разрешения импорта WASM как модуля
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        super().end_headers()

if __name__ == "__main__":
    PORT = 8000
    
    # Используем текущую директорию как корень сервера
    handler = partial(CustomHTTPRequestHandler, directory='.')
    
    with socketserver.TCPServer(("", PORT), handler) as httpd:
        print(f"Сервер запущен на http://localhost:{PORT}")
        print("Для остановки сервера нажмите Ctrl+C")
        httpd.serve_forever()