# Unified Worker Pool (Rust)

Кросс-платформенный пул задач с единым async API:

- **Хост**: `tokio + rayon` → настоящие потоки.
- **Браузер**: пул Web Workers с очередью.

## Сборка

### WASM (браузер):
```bash
wasm-pack build --target web --out-dir pkg --no-default-features --features web
python3 -m http.server 8080