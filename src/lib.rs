use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskInput {
    pub id: u32,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskOutput {
    pub id: u32,
    pub result: String,
}

/// Чистая бизнес-логика — работает везде
pub fn process_task(input: TaskInput) -> TaskOutput {
    let result = format!("processed: {} (len={})", input.payload, input.payload.len());
    TaskOutput {
        id: input.id,
        result,
    }
}

// Экспорт для WASM (вызов из JS)
#[cfg(target_arch = "wasm32")]
mod wasm_exports {
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
        pool: crate::web_worker_pool::WebWorkerPool,
    }

    #[wasm_bindgen]
    impl WebWorkerPoolWrapper {
        #[wasm_bindgen(constructor)]
        pub fn new(size: usize) -> Self {
            Self {
                pool: crate::web_worker_pool::WebWorkerPool::new(size),
            }
        }

        pub async fn run_tasks(&self, inputs: JsValue) -> JsValue {
            let inputs: Vec<TaskInput> = from_value(inputs).unwrap();
            let results = self.pool.run_tasks(inputs).await;
            to_value(&results).unwrap()
        }
    }
}

pub mod native_worker_pool;
pub mod web_worker_pool;
pub mod worker_pool;
pub use worker_pool::WorkerPool;
