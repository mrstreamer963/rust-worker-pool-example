#[cfg(not(feature = "web"))]
pub use crate::native_worker_pool::NativeWorkerPool as WorkerPool;

#[cfg(feature = "web")]
pub use crate::web_worker_pool::WebWorkerPool as WorkerPool;
