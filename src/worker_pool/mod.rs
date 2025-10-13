// Используем cfg_attr для условного указания путей к модулям
#[cfg_attr(not(target_arch = "wasm32"), path = "native_worker/mod.rs")]
#[cfg_attr(target_arch = "wasm32", path = "web_worker/mod.rs")]
mod worker_pool_impl;

#[cfg(not(target_arch = "wasm32"))]
pub use worker_pool_impl::NativeWorkerPool as WorkerPool;

#[cfg(target_arch = "wasm32")]
pub use worker_pool_impl::WebWorkerPool as WorkerPool;
