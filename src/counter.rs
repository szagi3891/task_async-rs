use std::sync::Arc;
use std::mem;

use types::{Callback0, CallbackBox0};

pub struct Counter {
    func : CallbackBox0,
}

impl Counter {

    pub fn new(func: Callback0) -> Arc<Counter> {

        Arc::new(Counter {
            func : CallbackBox0::new(func)
        })
    }
}

impl Drop for Counter {
    
    fn drop(&mut self) {
        
        let empty_clouser = CallbackBox0::new(Box::new(||{}));
        let func = mem::replace(&mut self.func, empty_clouser);
        
		func.exec();
    }
}

