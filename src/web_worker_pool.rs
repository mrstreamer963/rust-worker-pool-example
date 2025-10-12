use crate::{TaskInput, TaskOutput};

#[cfg(feature = "web")]
mod wasm_js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/worker_pool.js")]
    extern "C" {
        #[wasm_bindgen(js_name = WorkerPool)]
        pub type WorkerPoolJs;

        #[wasm_bindgen(constructor)]
        pub fn new(worker_url: &str, size: u32) -> WorkerPoolJs;

        #[wasm_bindgen(method, js_name = runTasksBatch)]
        pub fn run_tasks_batch(this: &WorkerPoolJs, payloads: &js_sys::Array) -> js_sys::Promise;
    }
}

#[cfg(feature = "web")]
pub struct WebWorkerPool {
    js_pool: std::sync::Arc<wasm_js::WorkerPoolJs>,
}

#[cfg(feature = "web")]
impl WebWorkerPool {
    pub fn new(size: usize) -> Self {
        let worker_url = "./worker.js";
        let pool = wasm_js::WorkerPoolJs::new(worker_url, size as u32);
        Self {
            js_pool: std::sync::Arc::new(pool),
        }
    }

    pub async fn run_tasks(&self, inputs: Vec<TaskInput>) -> Vec<TaskOutput> {
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
