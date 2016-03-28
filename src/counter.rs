use std::sync::Arc;
use std::mem;


pub struct Counter {
    func : Box<Fn() + Send + Sync + 'static>,
}

impl Counter {

    pub fn new(func: Box<Fn() + Send + Sync + 'static>) -> Arc<Counter> {

        Arc::new(Counter {
            func : func
        })
    }
}

impl Drop for Counter {
    
    fn drop(&mut self) {
        
        let empty_clouser = Box::new(||{});
        let func = mem::replace(&mut self.func, empty_clouser);
        func();
    }
}


/*
use std::boxed::FnBox;
Box<FnBox(T) + Send + Sync + 'static>;
*/

