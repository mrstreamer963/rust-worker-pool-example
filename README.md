# Worker Pool Demo (Rust + WASM + Web Workers)

Кросс-платформенный пул задач:
- **На хосте**: использует `rayon` (настоящие потоки).
- **В браузере**: использует пул Web Workers с очередью задач.

## Сборка

### Для WASM (браузер):
```bash
cargo install wasm-pack
wasm-pack build --target web --out-dir pkg
python3 -m http.server 8080
