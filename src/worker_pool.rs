#[cfg(not(target_arch = "wasm32"))]
pub use crate::native_worker_pool::NativeWorkerPool as WorkerPool;

#[cfg(target_arch = "wasm32")]
pub use crate::web_worker_pool::WebWorkerPool as WorkerPool;
