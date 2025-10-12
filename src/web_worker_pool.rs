#[cfg(target_arch = "wasm32")]
use crate::{TaskInput, TaskOutput};

#[cfg(target_arch = "wasm32")]
use js_sys;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/worker_pool.js")]
extern "C" {
    type WorkerPoolJs;

    #[wasm_bindgen(constructor)]
    fn new(worker_url: &str, size: u32) -> WorkerPoolJs;

    #[wasm_bindgen(method, js_name = runTasksBatch)]
    fn run_tasks_batch(this: &WorkerPoolJs, payloads: &js_sys::Array) -> js_sys::Promise;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "URL"], js_name = createObjectURL)]
    fn create_object_url(blob: &web_sys::Blob) -> String;

    #[wasm_bindgen(js_namespace = ["window", "Blob"])]
    fn new_blob(parts: &js_sys::Array, options: &js_sys::Object) -> web_sys::Blob;
}

#[cfg(target_arch = "wasm32")]
pub struct WebWorkerPool {
    js_pool: std::sync::Arc<WorkerPoolJs>,
}

#[cfg(target_arch = "wasm32")]
impl WebWorkerPool {
    pub fn new(size: usize) -> Self {
        // Загрузка содержимого worker.js из файла
        let worker_js_content = include_str!("./worker.js");

        // Создание Blob из строки JavaScript
        let js_array = js_sys::Array::new();
        js_array.push(&worker_js_content.into());

        let blob_options = js_sys::Object::new();
        js_sys::Reflect::set(
            &blob_options,
            &"type".into(),
            &"application/javascript".into(),
        )
        .unwrap();

        let blob = new_blob(&js_array, &blob_options);
        let worker_url = create_object_url(&blob);

        let pool = WorkerPoolJs::new(&worker_url, size as u32);
        Self {
            js_pool: std::sync::Arc::new(pool),
        }
    }

    pub async fn run_tasks(&self, inputs: Vec<TaskInput>) -> Vec<TaskOutput> {
        use js_sys::Array;
        use serde_wasm_bindgen::to_value;

        let js_inputs = Array::new();
        for input in inputs {
            js_inputs.push(&to_value(&input).unwrap());
        }

        let promise = self.js_pool.run_tasks_batch(&js_inputs);
        let js_result = JsFuture::from(promise).await.unwrap();
        serde_wasm_bindgen::from_value(js_result).expect("Failed to deserialize results")
    }
}
