use std::sync::Arc;
use std::mem;


pub struct Counter {
    func : Option<Box<Fn() + Send + Sync + 'static>>,
}

impl Counter {

    pub fn new(func: Box<Fn() + Send + Sync + 'static>) -> Arc<Counter> {

        Arc::new(Counter {
            func : Some(func)
        })
    }
}

impl Drop for Counter {
    
    fn drop(&mut self) {
        
        let func = mem::replace(&mut self.func, None);
        func.unwrap()();
    }
}
