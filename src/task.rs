use counter::Counter;
use std::sync::{Arc, RwLock};
use std::mem;

use result2::Result2;
use types::{Callback1, callback1_exec, Callback3, callback3_exec};




pub struct Task<A>
    where A : Send + Sync + 'static {
    
    counter : Arc<Counter>,
    func    : Option<Callback1<Option<A>>>,
}


impl<A> Drop for Task<A>
    where
        A : Send + Sync + 'static {

    fn drop(&mut self) {
        
        match mem::replace(&mut self.func, None) {

            Some(complete) => callback1_exec(complete, None),

            None => {
                //nic nie robimy, prawidłowy przebieg
            }
        }
    }
}


impl<A> Task<A> where A : Send + Sync + 'static {
    
    pub fn new(counter: Arc<Counter>, func: Callback1<Option<A>>) -> Task<A> {
        
        Task {
            counter : counter,
            func    : Some(func),
        }
    }
    
    pub fn result(mut self, value: A) {
        
        
        match mem::replace(&mut self.func, None) {
            
            Some(complete) => callback1_exec(complete, Some(value)),
            None => unreachable!(),
        }
    }
    
    
    pub fn async<B, C>(self, complete: Callback3<Task<A>, Option<B>, Option<C>>) -> (Task<B>, Task<C>)
    
    where
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
        
        
        let counter1 = self.counter.clone();
        let counter2 = self.counter.clone();
        
                                                        //TODO - upewnić się że licznik zbiorczego zadania prawidłowo się przenosi
        
        let new_complete = Box::new(move|result1 : Option<B>, result2 : Option<C>|{
            
            callback3_exec(complete, self, result1, result2);
        });
        
        
        let result = Arc::new(RwLock::new(Result2{
            complete : new_complete,
            result1  : None,
            result2  : None,
        }));
        
        let result_copy = result.clone();
        
            
        let func1 = Box::new(move |data: Option<B>| {
                                                                    //TODO - dobrze byłoby się pozbyć tego unwrapa
            result_copy.write().unwrap().result1 = data;
        });
        
        let func2 = Box::new(move |data: Option<C>| {
                                                                    //TODO - dobrze byłoby się pozbyć tego unwrapa
            result.write().unwrap().result2 = data;
        });
        
        (Task::new(counter1, func1), Task::new(counter2, func2))
    }
}

