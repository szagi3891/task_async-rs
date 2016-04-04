use counter::Counter;
use std::sync::{Arc, RwLock};
use std::mem;

use result2::Result2;
use types::{Callback1, callback1_exec, Callback3, callback3_exec};


pub struct Task<A>
    where A : Send + Sync + 'static {
    
    counter : Arc<Counter>,
    func    : Callback1<Option<A>>,
    is_exec : bool,
}


impl<A> Drop for Task<A>
    where
        A : Send + Sync + 'static {

    fn drop(&mut self) {
        
        if self.is_exec == false {

            let empty_clouser = Box::new(|_:Option<A>|{});
            let complete      = mem::replace(&mut self.func, empty_clouser);

            callback1_exec(complete, None);
        }
    }
}


impl<A> Task<A> where A : Send + Sync + 'static {
    
    pub fn new(counter: Arc<Counter>, func: Callback1<Option<A>>) -> Task<A> {
        
        Task {
            counter : counter,
            func    : func,
            is_exec : false,
        }
    }
    
    pub fn result(mut self, value: A) {
        
        let empty_clouser = Box::new(|_:Option<A>|{});
        let complete      = mem::replace(&mut self.func, empty_clouser);
        
        self.is_exec = true;
        
        callback1_exec(complete, Some(value));
    }
    
    
    pub fn async<B, C>(self, complete: Callback3<Task<A>, Option<B>, Option<C>>) -> (Task<B>, Task<C>)
    
    where
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
        
        
        let counter1 = self.counter.clone();
        let counter2 = self.counter.clone();
        let counter3 = self.counter.clone();
        
                                                        //TODO - upewnić się że licznik zbiorczego zadania prawidłowo się przenosi
        
        let new_complete = Box::new(move|result1 : Option<B>, result2 : Option<C>|{
            
            callback3_exec(complete, self, result1, result2);
        });
        
        
        let result = Arc::new(RwLock::new(Result2{
            _counter : counter1,
            complete : new_complete,
            result1  : None,
            result2  : None,
        }));
        
        let result_copy = result.clone();
        
            
        let func2 = Box::new(move |data: Option<B>| {
                                                                    //TODO - dobrze byłoby się pozbyć tego unwrapa
            result_copy.write().unwrap().result1 = data;
        });
        
        let func3 = Box::new(move |data: Option<C>| {
                                                                    //TODO - dobrze byłoby się pozbyć tego unwrapa
            result.write().unwrap().result2 = data;
        });
        
        (Task::new(counter2, func2), Task::new(counter3, func3))
    }
}

