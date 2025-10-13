#[cfg(not(target_arch = "wasm32"))]
pub mod native_worker_pool;
#[cfg(not(target_arch = "wasm32"))]
pub use native_worker_pool::NativeWorkerPool;
