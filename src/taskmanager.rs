use std::sync::Arc;

use counter::Counter;
use types::{Callback0, Callback1};
use task::Task;

pub struct TaskManager {
    counter : Arc<Counter>,
}

impl TaskManager {
    
    pub fn new(func: Callback0) -> TaskManager {
        
        TaskManager {
            counter : Counter::new(func)
        }
    }
    
    
    pub fn task<A>(&self, func: Callback1<Option<A>>) -> Task<A>
        where A : Send + Sync + 'static {
        
        Task::new(self.counter.clone(), func)
    }   
}
