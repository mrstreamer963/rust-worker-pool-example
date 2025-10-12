use crate::{process_task, TaskInput, TaskOutput};
use std::future::Future;
use std::pin::Pin;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

pub struct WorkerPool {
    #[cfg(feature = "native")]
    _size: usize, // rayon управляет пулом сам

    #[cfg(feature = "web")]
    js_pool: std::sync::Arc<wasm_bindgen::JsValue>,
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        #[cfg(feature = "native")]
        {
            // Убеждаемся, что rayon инициализирован с нужным числом потоков
            rayon::ThreadPoolBuilder::new()
                .num_threads(size)
                .build_global()
                .ok(); // игнорируем, если уже инициализирован
            Self { _size: size }
        }

        #[cfg(feature = "web")]
        {
            use js_sys::Function;
            use wasm_bindgen::JsCast;

            let worker_url = "./worker.js";
            // Импортируем конструктор из модуля worker_pool.js
            let module: &wasm_bindgen::JsValue = &wasm_bindgen::module_raw!("../worker_pool.js");
            let constructor = js_sys::Reflect::get(module, &"WorkerPool".into())
                .expect("WorkerPool not found in worker_pool.js");
            let pool = Function::from(constructor)
                .new_with_args(&[worker_url.into(), &(size as u32).into()])
                .expect("Failed to create WorkerPool");

            Self {
                js_pool: std::sync::Arc::new(pool),
            }
        }
    }

    /// Запускает задачи параллельно и возвращает результаты в том же порядке.
    pub async fn run_tasks(&self, inputs: Vec<TaskInput>) -> Vec<TaskOutput> {
        #[cfg(feature = "native")]
        {
            use tokio::task;

            let handles: Vec<_> = inputs
                .into_iter()
                .map(|input| task::spawn_blocking(|| process_task(input)))
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

            let method = js_sys::Reflect::get(&self.js_pool, &"runTasksBatch".into())
                .expect("runTasksBatch not found");
            let func = js_sys::Function::from(method);
            let promise = func.call1(&self.js_pool, &js_inputs).unwrap();

            let js_result = JsFuture::from(promise).await.unwrap();
            serde_wasm_bindgen::from_value(js_result).expect("Failed to deserialize results")
        }
    }
}
