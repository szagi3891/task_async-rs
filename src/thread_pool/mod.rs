use std::sync::{Arc, Mutex};

mod sender_id;
mod receiver_id;
mod types;
mod inner;
mod autoid;

use thread_pool::types::{CounterType, WorkerBuilderType};
use thread_pool::inner::{Inner};

pub struct ThreadPool<Param: Send + Sync + 'static> {
    inner: Arc<Mutex<Inner<Param>>>,
}

impl<Param> ThreadPool<Param> where Param: Send + Sync + 'static {
        
    pub fn new(count: CounterType, worker_builder: WorkerBuilderType<Param>) -> ThreadPool<Param> {

        let inner = Inner::new();

        let pool = ThreadPool {
            inner: Arc::new(Mutex::new(inner))
        };
        
        pool.new_workers(count, worker_builder);
        
        pool
    }

    fn new_workers(&self, count: CounterType, worker_builder: WorkerBuilderType<Param>) {

        let inner_weak = Arc::downgrade(&self.inner);
        let mut guard = self.inner.lock().unwrap();
        
        for _ in 0..count {
            let worker_function = (worker_builder)();
            guard.create_worker(inner_weak.clone(), worker_function);
        }
    }

    pub fn run(&self, param: Param) {
        let mut guard = self.inner.lock().unwrap();
        guard.run(param);
    }
}

impl<Param> Clone for ThreadPool<Param> where Param: Send + Sync + 'static {
    
    fn clone(&self) -> ThreadPool<Param> {
        
        ThreadPool {
            inner : self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &ThreadPool<Param>) {
        self.inner = source.inner.clone();
    }
}
