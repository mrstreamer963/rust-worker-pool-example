# Rust Worker Pool (Native + WASM)

Единый асинхронный пул задач для хоста и браузера.

## Сборка

### Для браузера (WASM):
```bash
wasm-pack build --target web --out-dir pkg
python3 -m http.server 8080


### расширение

посмотреть вот эту ветку обсуждения:

https://users.rust-lang.org/t/creating-a-web-worker-from-rust-wasm-using-a-bundler/104866/7
