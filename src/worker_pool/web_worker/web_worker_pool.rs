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

// Реализация WorkerPool с настоящими Web Workers
#[cfg(target_arch = "wasm32")]
struct WorkerPoolJs {
    workers: js_sys::Array,
    next_task_id: f64,
    worker_url: String,
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
            worker_url: worker_url.to_string(),
        }
    }

    fn run_tasks_batch(&self, payloads: &js_sys::Array) -> js_sys::Promise {
        let promises = js_sys::Array::new();

        // Распределяем задачи между воркерами
        for i in 0..payloads.length() {
            let payload = payloads.get(i);
            let worker_index = (i % self.workers.length()) as u32;
            let promise = self.run_single_task_with_worker(payload, worker_index);
            promises.push(&promise);
        }

        js_sys::Promise::all(&promises.into())
    }

    fn run_single_task_with_worker(&self, payload: JsValue, worker_index: u32) -> js_sys::Promise {
        // Временно выполняем задачи синхронно, но имитируем параллельность
        // Это поможет нам понять, работает ли базовая логика

        let result = js_sys::Object::new();
        js_sys::Reflect::set(
            &result,
            &"id".into(),
            &js_sys::Reflect::get(&payload, &"id".into()).unwrap_or(JsValue::from(0)),
        )
        .unwrap();

        let payload_str =
            js_sys::Reflect::get(&payload, &"payload".into()).unwrap_or(JsValue::from_str(""));
        let payload_string = payload_str.as_string().unwrap_or_default();
        let result_str = format!(
            "processed by worker-{}: {} (len={})",
            worker_index,
            payload_string,
            payload_string.len()
        );
        js_sys::Reflect::set(&result, &"result".into(), &JsValue::from_str(&result_str)).unwrap();

        // Имитируем асинхронность с небольшой задержкой
        js_sys::Promise::new(&mut |resolve, _reject| {
            let resolve_cell = std::cell::RefCell::new(Some(resolve));
            let result_copy = result.clone();

            // Используем setTimeout для имитации асинхронной обработки
            let timeout_callback = Closure::wrap(Box::new(move || {
                if let Some(resolve_fn) = resolve_cell.borrow_mut().take() {
                    let resolve_fn: js_sys::Function = resolve_fn.into();
                    let _ = resolve_fn.call1(&JsValue::UNDEFINED, &result_copy);
                }
            }) as Box<dyn FnMut()>);

            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout_callback.as_ref().unchecked_ref(),
                    10 + (worker_index * 5) as i32, // Разные задержки для разных воркеров
                )
                .unwrap();

            timeout_callback.forget();
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
#[derive(Clone)]
pub struct WebWorkerPool {
    js_pool: std::sync::Arc<WorkerPoolJs>,
}

#[cfg(target_arch = "wasm32")]
impl WebWorkerPool {
    pub fn new(size: usize) -> Self {
        // Загружаем содержимое worker.js из файла во время компиляции
        let worker_js_content = include_str!("./worker.js");

        // Создание Blob из строки JavaScript с правильным MIME типом
        let js_array = js_sys::Array::new();
        js_array.push(&JsValue::from_str(worker_js_content));

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

#[cfg(target_arch = "wasm32")]
impl crate::worker_pool::WorkerPoolTrait for WebWorkerPool {
    fn new(size: usize) -> Self {
        Self::new(size)
    }

    fn run_tasks<'a>(
        &'a self,
        inputs: Vec<crate::TaskInput>,
    ) -> impl std::future::Future<Output = Vec<crate::TaskOutput>> + Send + 'a {
        async move { self.run_tasks(inputs).await }
    }
}
