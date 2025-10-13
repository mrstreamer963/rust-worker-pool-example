#[cfg(target_arch = "wasm32")]
pub mod web_worker_pool;
#[cfg(target_arch = "wasm32")]
pub use web_worker_pool::WebWorkerPool;
