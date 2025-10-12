use worker_pool_demo::{TaskInput, WorkerPool};

#[tokio::main]
async fn main() {
    let pool = WorkerPool::new(4);
    let tasks = (0..8)
        .map(|i| TaskInput {
            id: i,
            payload: format!("task-{}", i),
        })
        .collect();

    let results = pool.run_tasks(tasks).await;
    println!("✅ Completed {} tasks:", results.len());
    for r in results {
        println!("  - {:?}", r);
    }
}
