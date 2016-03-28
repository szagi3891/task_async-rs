use std::sync::Arc;
use std::mem;
use std::boxed::FnBox;


pub struct Counter {
    func : Box<FnBox() + Send + Sync + 'static>,
}

impl Counter {

    pub fn new(func: Box<FnBox() + Send + Sync + 'static>) -> Arc<Counter> {

        Arc::new(Counter {
            func : func
        })
    }
}

impl Drop for Counter {
    
    fn drop(&mut self) {
        
        let empty_clouser = Box::new(||{});
        let func = mem::replace(&mut self.func, empty_clouser);
        
        (func as Box<FnBox()>)();
    }
}

