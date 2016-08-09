use std::sync::{Arc, RwLock};
use std::mem;

use result2::Result2;
use callback0;
use callback1;
use callback2;
use callback3;
use callback4;
use callback5;

use task_pool::TaskPool;

pub struct Task<A>
    where A : Send + Sync + 'static {
    
    func : Option<callback1::CallbackBox<Option<A>>>,
    task_pool: TaskPool
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


pub fn new_task<A>(func: callback1::Callback<Option<A>>, task_pool: TaskPool) -> Task<A>
    where A : Send + Sync + 'static {

    Task {
        func : Some(callback1::new(func)),
        task_pool: task_pool
    }
}


impl<A> Task<A> where A : Send + Sync + 'static {

    pub fn result(mut self, value: A) {
        
        match mem::replace(&mut self.func, None) {
            
            Some(complete) => {
                
                                            //TODO - przesyłać do puli callbacka z parametrami do wykonania

                self.task_pool.run(callback0::new(Box::new(move||{
                    complete.exec(Some(value));
                })));
            },
            None => unreachable!(),
        }
    }
    
    
    pub fn async1<B>(self, complete: callback2::Callback<Task<A>, Option<B>>) -> Task<B>
    
    where
        B : Send + Sync + 'static {
        
        let pool1 = self.task_pool.clone();
        let pool2 = self.task_pool.clone();
        
                                        //TODO - przesyłać do puli callbacka z parametrami do wykonania
            
        let func = Box::new(move |result: Option<B>| {
            pool1.run(callback0::new(Box::new(move||{
                
                callback2::exec(complete, self, result);
            })));
        });
        
        new_task(func, pool2)
    }
    
    
    
    pub fn async2<B, C>(self, complete: callback3::Callback<Task<A>, Option<B>, Option<C>>) -> (Task<B>, Task<C>)
    
    where
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
        
            
        let pool1 = self.task_pool.clone();
        let pool2 = self.task_pool.clone();
        let pool3 = self.task_pool.clone();
        
                                                    //TODO - przesyłać do puli callbacka z parametrami do wykonania
            
        let new_complete = Box::new(move|result1 : Option<B>, result2 : Option<C>|{
            
            pool3.run(callback0::new(Box::new(move||{
            
                callback3::exec(complete, self, result1, result2);
            })));
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
        
        (new_task(func1, pool1), new_task(func2, pool2))
    }
    
    
    pub fn async3<B, C, D>(self, complete: callback4::Callback<Task<A>, Option<B>, Option<C>, Option<D>>) -> (Task<B>, Task<C>, Task<D>)
    
    where
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static {
         
        
        let (set1, set2) = self.async2(Box::new(move|task: Task<A>, response1: Option<(Option<B>, Option<C>)>, response2: Option<D>|{
            
            let (res1, res2) = opt_to_tuple(response1);
            
            callback4::exec(complete, task, res1, res2, response2);
        }));
        
        let (set11, set12) = set1.async2(Box::new(move|task: Task<(Option<B>, Option<C>)>, response1: Option<B>, response2:Option<C>|{
            
            task.result((response1, response2));
            
        }));
        
        (set11, set12, set2)
    }
    
    
    
    pub fn async4<B, C, D, E>(self, complete: callback5::Callback<Task<A>, Option<B>, Option<C>, Option<D>, Option<E>>) -> (Task<B>, Task<C>, Task<D>, Task<E>)
    
    where
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static ,
        E : Send + Sync + 'static {
         
        
        let (set1, set2) = self.async2(Box::new(move|task: Task<A>, response1: Option<(Option<B>, Option<C>)>, response2: Option<(Option<D>, Option<E>)>|{
            
            let (res1, res2) = opt_to_tuple(response1);
            let (res3, res4) = opt_to_tuple(response2);
            
            callback5::exec(complete, task, res1, res2, res3, res4);
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
    