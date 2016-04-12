use counter::Counter;
use std::sync::{Arc, RwLock};
use std::mem;

use result2::Result2;
use types::{Callback1, CallbackBox1, Callback2, CallbackBox2, Callback3, callback3_exec, Callback4, callback4_exec, Callback5, callback5_exec};




pub struct Task<A>
    where A : Send + Sync + 'static {
    
    counter : Arc<Counter>,
    func    : Option<CallbackBox1<Option<A>>>,
}


impl<A> Drop for Task<A>
    where
        A : Send + Sync + 'static {

    fn drop(&mut self) {
        
        match mem::replace(&mut self.func, None) {

            Some(complete) => complete.exec(None),

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
            func    : Some(CallbackBox1::new(func)),
        }
    }
    
    pub fn result(mut self, value: A) {
        
        
        match mem::replace(&mut self.func, None) {
            
            Some(complete) => complete.exec(Some(value)),
            None => unreachable!(),
        }
    }
    
    
    pub fn async1<B>(self, complete: Callback2<Task<A>, Option<B>>) -> Task<B>
    
    where
        B : Send + Sync + 'static {
        
        let counter = self.counter.clone();
        
        let func = Box::new(move |result: Option<B>| {
            
            callback2_exec(complete, self, result);
        });
        
        Task::new(counter, func)
    }
    
    
    
    pub fn async2<B, C>(self, complete: Callback3<Task<A>, Option<B>, Option<C>>) -> (Task<B>, Task<C>)
    
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
    
    
    pub fn async3<B, C, D>(self, complete: Callback4<Task<A>, Option<B>, Option<C>, Option<D>>) -> (Task<B>, Task<C>, Task<D>)
    
    where
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static {
         
        
        let (set1, set2) = self.async2(Box::new(move|task: Task<A>, response1: Option<(Option<B>, Option<C>)>, response2: Option<D>|{
            
            let (res1, res2) = opt_to_tuple(response1);
            
            callback4_exec(complete, task, res1, res2, response2);
        }));
        
        let (set11, set12) = set1.async2(Box::new(move|task: Task<(Option<B>, Option<C>)>, response1: Option<B>, response2:Option<C>|{
            
            task.result((response1, response2));
            
        }));
        
        (set11, set12, set2)
    }
    
    
    
    pub fn async4<B, C, D, E>(self, complete: Callback5<Task<A>, Option<B>, Option<C>, Option<D>, Option<E>>) -> (Task<B>, Task<C>, Task<D>, Task<E>)
    
    where
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static ,
        E : Send + Sync + 'static {
         
        
        let (set1, set2) = self.async2(Box::new(move|task: Task<A>, response1: Option<(Option<B>, Option<C>)>, response2: Option<(Option<D>, Option<E>)>|{
            
            let (res1, res2) = opt_to_tuple(response1);
            let (res3, res4) = opt_to_tuple(response2);
            
            callback5_exec(complete, task, res1, res2, res3, res4);
        }));
        
        let (set11, set12) = set1.async2(Box::new(move|task: Task<(Option<B>, Option<C>)>, response1: Option<B>, response2:Option<C>|{
            
            task.result((response1, response2));
            
        }));
        
        let (set21, set22) = set2.async2(Box::new(move|task: Task<(Option<D>, Option<E>)>, response1: Option<D>, response2:Option<E>|{
            
            task.result((response1, response2));
        }));
        
        
        (set11, set12, set21, set22)
    }
}


    
//opcję z dwójki zamieniać na dwójkę opcji ....

fn opt_to_tuple<A,B>(opt: Option<(Option<A>,Option<B>)>) -> (Option<A>, Option<B>)
where
    A : Send + Sync + 'static ,
    B : Send + Sync + 'static {

    match opt {
        Some((a,b)) => (a,b),
        None => (None, None),
    }
}
    