use std::sync::Arc;
use std::boxed::FnBox;

use counter::Counter;
use types::Callback1;
use task::Task;

pub struct TaskManager {
    counter : Arc<Counter>,
}

impl TaskManager {
    
    pub fn new(func: Box<FnBox() + Send + Sync + 'static>) -> TaskManager {
        
        TaskManager {
            counter : Counter::new(func)
        }
    }
    
    
    pub fn task<A>(&self, func: Callback1<Option<A>>) -> Task<A>
        where A : Send + Sync + 'static {
        
        Task::new(self.counter.clone(), func)
    }   
}
