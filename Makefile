# Makefile для проекта Rust worker-pool-demo

# Запуск примера для нативной реализации
.PHONY: examples
examples:
	cargo run --example native_main

# Сборка проекта
.PHONY: build
build:
	cargo build

# Сборка релиза
.PHONY: release
release:
	cargo build --release

# Запуск тестов
.PHONY: test
test:
	cargo test

# Очистка
.PHONY: clean
clean:
	cargo clean

# Документация
.PHONY: docs
docs:
	cargo doc --open

# Форматирование кода
.PHONY: fmt
fmt:
	cargo fmt

# Проверка кода с помощью clippy
.PHONY: lint
lint:
	cargo clippy

# Сборка для WebAssembly
.PHONY: wasm
wasm:
	wasm-pack build --target web --out-dir pkg

# Запуск HTTP-сервера для демонстрации WebAssembly
.PHONY: serve
serve: wasm
	python3 -m http.server 8081
