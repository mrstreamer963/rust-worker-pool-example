use tokio::runtime::Runtime;
use worker_pool_demo::{TaskInput, WorkerPool};

fn main() {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let pool = WorkerPool::new(4);
    let tasks = (0..6)
        .map(|i| TaskInput {
            id: i,
            payload: format!("native-task-{}", i),
        })
        .collect();

    let results = rt.block_on(async { pool.run_tasks(tasks).await });

    println!("✅ Completed {} tasks:", results.len());
    for r in results {
        println!("  - {:?}", r);
    }
}
