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

#[cfg(target_arch = "wasm32")]
mod wasm {
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
    pub fn main() {
        // Можно добавить panic hook для отладки
        // console_error_panic_hook::set_once();
    }
}
