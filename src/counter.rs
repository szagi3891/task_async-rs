use std::sync::Arc;
use std::mem;

use types::{Callback0, callback0_exec};

pub struct Counter {
    func : Callback0,
}

impl Counter {

    pub fn new(func: Callback0) -> Arc<Counter> {

        Arc::new(Counter {
            func : func
        })
    }
}

impl Drop for Counter {
    
    fn drop(&mut self) {
        
        let empty_clouser = Box::new(||{});
        let func = mem::replace(&mut self.func, empty_clouser);
        
        callback0_exec(func);
    }
}

