# Rust Worker Pool (Native + WASM)

Единый асинхронный пул задач для хоста и браузера.

## Сборка

### Для браузера (WASM):
```bash
wasm-pack build --target web --out-dir pkg --no-default-features --features web
python3 -m http.server 8080