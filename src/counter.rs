use std::sync::Arc;
use std::mem;

use callback::callback0;

pub struct Counter {
    func : callback0::CallbackBox,
}

impl Counter {

    pub fn new(func: callback0::Callback) -> Arc<Counter> {

        Arc::new(Counter {
            func : callback0::new(func)
        })
    }
}

impl Drop for Counter {
    
    fn drop(&mut self) {
        
        let empty_clouser = callback0::new(Box::new(||{}));
        let func = mem::replace(&mut self.func, empty_clouser);
        
		func.exec();
    }
}

