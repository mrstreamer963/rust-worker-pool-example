use super::*;
use js_sys::Promise;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub fn run_task(input_js: JsValue) -> Promise {
    future_to_promise(async move {
        let input: TaskInput = from_value(input_js)
            .map_err(|e| JsValue::from_str(&format!("Deserialize error: {:?}", e)))?;
        let output = process_task(input);
        Ok(to_value(&output).unwrap())
    })
}

#[wasm_bindgen(start)]
pub fn main() {}

#[wasm_bindgen]
pub struct WebWorkerPoolWrapper {
    pool: crate::worker_pool::WorkerPool,
}

#[wasm_bindgen]
impl WebWorkerPoolWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Self {
        Self {
            pool: crate::worker_pool::WorkerPool::new(size),
        }
    }

    pub fn run_tasks(&self, inputs: JsValue) -> Promise {
        let pool = self.pool.clone();
        future_to_promise(async move {
            let inputs: Vec<TaskInput> = from_value(inputs)
                .map_err(|e| JsValue::from_str(&format!("Deserialize error: {:?}", e)))?;
            let results = pool.run_tasks(inputs).await;
            Ok(to_value(&results).unwrap())
        })
    }
}
