use counter::Counter;
use std::boxed::FnBox;
use std::sync::{Arc, RwLock};
use std::mem;


pub type Callback0        = Box<FnBox()      + Send + Sync + 'static>;
pub type Callback1<A>     = Box<FnBox(A)     + Send + Sync + 'static>;
pub type Callback2<A,B>   = Box<FnBox(A,B)   + Send + Sync + 'static>;
pub type Callback3<A,B,C> = Box<FnBox(A,B,C) + Send + Sync + 'static>;


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



pub struct Task<A>
    where A : Send + Sync + 'static {
    
    counter : Arc<Counter>,
    func    : Callback1<Option<A>>,
}

/*
impl<A,B> Drop for Result2<A,B>
    where
        A : Send + Sync + 'static,
        B : Send + Sync + 'static {

    fn drop(&mut self) {
        
    }
}
*/

impl<A> Task<A> where A : Send + Sync + 'static {
    
    fn new(counter: Arc<Counter>, func: Callback1<Option<A>>) -> Task<A> {
        
        Task {
            counter : counter,
            func    : func
        }
    }
    
    pub fn result(mut self, value: A) {
        
        let empty_clouser = Box::new(|_:Option<A>|{});
        let complete      = mem::replace(&mut self.func, empty_clouser);
        
        (complete as Box<FnBox(Option<A>)>)(Some(value));
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
            
            (complete as Box<FnBox(Task<A>, Option<B>, Option<C>)>)(self, result1, result2);
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
