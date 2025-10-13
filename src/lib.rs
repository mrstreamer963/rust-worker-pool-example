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
mod wasm_exports;

pub mod worker_pool;
pub use worker_pool::WorkerPool;
