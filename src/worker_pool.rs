use crate::{TaskInput, TaskOutput};

pub struct WorkerPool {
    #[cfg(feature = "native")]
    _size: usize,

    #[cfg(feature = "web")]
    js_pool: std::sync::Arc<wasm_js::WorkerPoolJs>,
}

#[cfg(feature = "web")]
mod wasm_js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "../worker_pool.js")]
    extern "C" {
        #[wasm_bindgen(js_name = WorkerPool)]
        pub type WorkerPoolJs;

        #[wasm_bindgen(constructor)]
        pub fn new(worker_url: &str, size: u32) -> WorkerPoolJs;

        #[wasm_bindgen(method, js_name = runTasksBatch)]
        pub fn run_tasks_batch(this: &WorkerPoolJs, payloads: &js_sys::Array) -> js_sys::Promise;
    }
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        #[cfg(feature = "native")]
        {
            rayon::ThreadPoolBuilder::new()
                .num_threads(size)
                .build_global()
                .ok();
            Self { _size: size }
        }

        #[cfg(feature = "web")]
        {
            let worker_url = "./worker.js";
            let pool = wasm_js::WorkerPoolJs::new(worker_url, size as u32);
            Self {
                js_pool: std::sync::Arc::new(pool),
            }
        }
    }

    pub async fn run_tasks(&self, inputs: Vec<TaskInput>) -> Vec<TaskOutput> {
        #[cfg(feature = "native")]
        {
            use tokio::task;
            let handles: Vec<_> = inputs
                .into_iter()
                .map(|input| task::spawn_blocking(|| crate::process_task(input)))
                .collect();

            let mut results = Vec::with_capacity(handles.len());
            for handle in handles {
                results.push(handle.await.expect("Task panicked"));
            }
            results
        }

        #[cfg(feature = "web")]
        {
            use js_sys::Array;
            use serde_wasm_bindgen::to_value;
            use wasm_bindgen_futures::JsFuture;

            let js_inputs = Array::new();
            for input in inputs {
                js_inputs.push(&to_value(&input).unwrap());
            }

            let promise = self.js_pool.run_tasks_batch(&js_inputs);
            let js_result = JsFuture::from(promise).await.unwrap();
            serde_wasm_bindgen::from_value(js_result).expect("Failed to deserialize results")
        }
    }
}
