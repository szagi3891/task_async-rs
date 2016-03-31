use counter::Counter;
use std::boxed::FnBox;
use std::sync::Arc;
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
    
    counter  : Arc<Counter>,
    complete : Callback2<Option<A>, Option<B>>,
    result1  : Option<A>,
    result2  : Option<B>,
}

impl<A,B> Drop for Result2<A,B>
    where
        A : Send + Sync + 'static,
        B : Send + Sync + 'static {

    fn drop(&mut self) {

        let empty_clouser = Box::new(|a:Option<A>, b:Option<B>|{});
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
    
    pub fn async_run<A,B>(&self, set1: Callback1<Callback1<A>>, set2: Callback1<Callback1<B>>, complete: Callback2<Option<A>, Option<B>>)
    where
        A : Send + Sync + 'static ,
        B : Send + Sync + 'static {
        
        let result = Arc::new(RwLock::new(Result2{
            counter  : self.counter.clone(),
            complete : complete,
            result1  : None,
            result2  : None,
        }));
        
        
    /*
    counter  : Arc<Counter>,
    complete : Callback2<Option<A>, Option<B>>,
    result1  : Option<A>,
    result2  : Option<B>,
    */
/*
pub fn get<F>(id: i32, job: F) where F: Fn(i32) + Send + 'static + Sync {

    let job_box = Box::new(job);

    struct ApiResult {
        job     : Option<Box<Fn(i32) + Send + 'static + Sync>>,
        result1 : Option<i32>,
        result2 : Option<i32>,
    }

    impl Drop for ApiResult {

        fn drop(&mut self) {
            let job = mem_replace(&mut self.job, None);
            job.unwrap()(100000 * self.result1.unwrap() + self.result2.unwrap());
        }

    }

    let result = ApiResult{job: Some(job_box), result1: None, result2: None};
    let result = Arc::new(RwLock::new(result));

    let result_copy = result.clone();

    api1::get(50, move |res_data:i32| {
        println!("{} wykonuję callbacka 1", thread::current().name().unwrap());
        result_copy.write().unwrap().result1 = Some(res_data);
    });

    api2::get(1000, move |res_data: i32| {
        println!("{} wykonuję callbacka 2", thread::current().name().unwrap());
        result.write().unwrap().result2 = Some(res_data);
    });
}
*/
    }
}