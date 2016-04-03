#![feature(fnbox)]


extern crate channels_async;

mod counter;
mod task;

pub use counter::Counter;
pub use task::{TaskManager, Task, Callback1, Callback2};




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


//TODO - zaimplementować drop dla taska, ma wykonać wtedy callbacka z wartością none ...
        //też trzeba sprawdzić czy działa wysyłanie None
 
    
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