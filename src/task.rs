use counter::Counter;
use std::boxed::FnBox;
use std::sync::{Arc, RwLock};
use std::mem;


pub type Callback1<A>   = Box<FnBox(A)   + Send + Sync + 'static>;
pub type Callback2<A,B> = Box<FnBox(A,B) + Send + Sync + 'static>;


pub struct TaskManager {
    counter : Arc<Counter>,
}



struct Result2<A,B>
    where
        A : Send + Sync + 'static,
        B : Send + Sync + 'static {
    
    _counter : Arc<Counter>,                        //ta zmienne jest tylko do przetrzymywania referencji
    complete : Callback2<Option<A>, Option<B>>,
    result1  : Option<A>,
    result2  : Option<B>,
}


impl<A,B> Drop for Result2<A,B>
    where
        A : Send + Sync + 'static,
        B : Send + Sync + 'static {

    fn drop(&mut self) {

        let empty_clouser = Box::new(|_:Option<A>, _:Option<B>|{});
        let complete      = mem::replace(&mut self.complete, empty_clouser);
        let result1       = mem::replace(&mut self.result1, None);
        let result2       = mem::replace(&mut self.result2, None);
        
        (complete as Box<FnBox(Option<A>,Option<B>)>)(result1, result2);
    }
}



impl TaskManager {
    
    pub fn new(func: Box<FnBox() + Send + Sync + 'static>) -> TaskManager {
        
        TaskManager {
            counter : Counter::new(func)
        }
    }
    
    pub fn async_run<A,B>(&self, complete: Callback2<Option<A>, Option<B>>) -> (Callback1<A>, Callback1<B>)
    where
        A : Send + Sync + 'static ,
        B : Send + Sync + 'static {
        
        let result = Arc::new(RwLock::new(Result2{
            _counter : self.counter.clone(),
            complete : complete,
            result1  : None,
            result2  : None,
        }));
        
        let result_copy = result.clone();

        let set1 = Box::new(move |data: A| {
                                                                        //TODO - dobrze byłoby się pozbyć tego unwrapa
            result_copy.write().unwrap().result1 = Some(data);
        });
        
        let set2 = Box::new(move |data: B| {
                                                                        //TODO - dobrze byłoby się pozbyć tego unwrapa
            result.write().unwrap().result2 = Some(data);
        });
            
        (set1, set2)
    }
    
    pub fn clone(&self) -> TaskManager {
        
        TaskManager {
            counter : self.counter.clone()
        }
    }
}
