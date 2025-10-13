#[cfg(target_arch = "wasm32")]
use crate::{TaskInput, TaskOutput};

#[cfg(target_arch = "wasm32")]
use js_sys;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys;

// Максимально простая реализация WorkerPool
#[cfg(target_arch = "wasm32")]
struct WorkerPoolJs {
    workers: js_sys::Array,
    next_task_id: f64,
}

#[cfg(target_arch = "wasm32")]
impl WorkerPoolJs {
    fn new(worker_url: &str, size: u32) -> Self {
        let workers = js_sys::Array::new();
        
        // Создаем воркеров
        for _ in 0..size {
            if let Ok(worker) = web_sys::Worker::new(worker_url) {
                workers.push(&worker);
            }
        }

        Self {
            workers,
            next_task_id: 0.0,
        }
    }

    fn run_tasks_batch(&self, payloads: &js_sys::Array) -> js_sys::Promise {
        // Простая реализация - выполняем задачи последовательно
        let promises = js_sys::Array::new();
        
        for i in 0..payloads.length() {
            let payload = payloads.get(i);
            let promise = self.run_single_task(payload);
            promises.push(&promise);
        }

        js_sys::Promise::all(&promises.into())
    }

    fn run_single_task(&self, payload: JsValue) -> js_sys::Promise {
        // Берем первого доступного воркера
        if self.workers.length() == 0 {
            return js_sys::Promise::reject(&JsValue::from_str("No workers available"));
        }
        
        let worker = self.workers.get(0);
        let worker: web_sys::Worker = worker.into();
        
        let task_id = self.next_task_id;
        
        // Создаем простой Promise
        js_sys::Promise::new(&mut |resolve, reject| {
            let resolve_cell = std::cell::RefCell::new(Some(resolve));
            let reject_cell = std::cell::RefCell::new(Some(reject));
            
            let onmessage = Closure::wrap(Box::new(move |event: JsValue| {
                let data = js_sys::Reflect::get(&event, &"data".into()).unwrap_or(JsValue::UNDEFINED);
                let result_type = js_sys::Reflect::get(&data, &"type".into()).unwrap_or(JsValue::UNDEFINED);
                
                if result_type.as_string().unwrap_or_default() == "result" {
                    let result = js_sys::Reflect::get(&data, &"result".into()).unwrap_or(JsValue::UNDEFINED);
                    if let Some(resolve_fn) = resolve_cell.borrow_mut().take() {
                        let resolve_fn: js_sys::Function = resolve_fn.into();
                        let _ = resolve_fn.call1(&JsValue::UNDEFINED, &result);
                    }
                } else {
                    let error = js_sys::Reflect::get(&data, &"error".into()).unwrap_or(JsValue::from_str("Unknown error"));
                    if let Some(reject_fn) = reject_cell.borrow_mut().take() {
                        let reject_fn: js_sys::Function = reject_fn.into();
                        let _ = reject_fn.call1(&JsValue::UNDEFINED, &error);
                    }
                }
            }) as Box<dyn FnMut(JsValue)>);
            
            worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            onmessage.forget();
            
            // Отправляем задачу
            let message = js_sys::Object::new();
            js_sys::Reflect::set(&message, &"taskId".into(), &task_id.into()).unwrap_or_default();
            js_sys::Reflect::set(&message, &"payload".into(), &payload).unwrap_or_default();
            let _ = worker.post_message(&message);
        })
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "URL"], js_name = createObjectURL)]
    fn create_object_url(blob: &web_sys::Blob) -> String;
}

#[cfg(target_arch = "wasm32")]
pub struct WebWorkerPool {
    js_pool: std::sync::Arc<WorkerPoolJs>,
}

#[cfg(target_arch = "wasm32")]
impl WebWorkerPool {
    pub fn new(size: usize) -> Self {
        // Загрузка содержимого worker.js из файла
        let worker_js_content = include_str!("worker.js");

        // Создание Blob из строки JavaScript с правильным MIME типом
        let js_array = js_sys::Array::new();
        js_array.push(&worker_js_content.into());

        // Создаем BlobPropertyBag с правильным типом для ES6 модулей
        let blob_options = js_sys::Object::new();
        js_sys::Reflect::set(
            &blob_options,
            &"type".into(),
            &"application/javascript".into(),
        )
        .unwrap();

        let blob = web_sys::Blob::new_with_str_sequence(&js_array).unwrap();
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
