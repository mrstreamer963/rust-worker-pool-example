use std::future::Future;

pub trait WorkerPoolTrait {
    fn new(size: usize) -> Self;
    fn run_tasks<'a>(
        &'a self,
        inputs: Vec<crate::TaskInput>,
    ) -> impl Future<Output = Vec<crate::TaskOutput>> + Send + 'a;
}
