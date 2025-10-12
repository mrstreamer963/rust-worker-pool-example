use crate::{process_task, TaskInput, TaskOutput};
use rayon::prelude::*;

pub fn run_tasks_parallel(inputs: Vec<TaskInput>) -> Vec<TaskOutput> {
    inputs.into_par_iter().map(process_task).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_pool() {
        let tasks = (0..4)
            .map(|i| TaskInput {
                id: i,
                payload: format!("task-{}", i),
            })
            .collect();
        let results = run_tasks_parallel(tasks);
        assert_eq!(results.len(), 4);
        println!("Native test passed: {:?}", results);
    }
}
