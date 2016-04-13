use std::sync::Arc;

use counter::Counter;
use task::Task;
use callback0;
use callback1;

pub struct TaskManager {
    counter : Arc<Counter>,
}

impl TaskManager {
    
    pub fn new(func: callback0::Callback) -> TaskManager {
        
        TaskManager {
            counter : Counter::new(func)
        }
    }
    
    
    pub fn task<A>(&self, func: callback1::Callback<Option<A>>) -> Task<A>
        where A : Send + Sync + 'static {
        
        Task::new(self.counter.clone(), func)
    }   
}
