#[cfg(not(target_arch = "wasm32"))]
pub struct NativeWorkerPool {
    _size: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeWorkerPool {
    pub fn new(size: usize) -> Self {
        rayon::ThreadPoolBuilder::new()
            .num_threads(size)
            .build_global()
            .ok();
        Self { _size: size }
    }

    pub async fn run_tasks(&self, inputs: Vec<crate::TaskInput>) -> Vec<crate::TaskOutput> {
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
}
