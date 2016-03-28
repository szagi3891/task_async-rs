#![feature(fnbox)]


extern crate channels_async;

mod counter;

pub use counter::Counter;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}



//TODO ...

/*
    taskManager = create(||{

        //gdy licznik spadnie do zera, to wykonywany jest callback
        //samozjadanie down, usunięcie markera

        //w tym momencie można zrobić group.off();          //wyłączenie wszystkich pracujących wątków

        //sygnał dla świata na zewnątrz przez kanał, że grupa wątków dokonała swego żywota
    });


    //jakieś operacje asynchroniczne używające taskManager-a ...

*/


/*


let mut taskManager = createTaskManager();


let (set_res1, set_res2) = taskManager.create(move|resp1, resp2|{

    //gdy odpowiedź z obu odpowiedzi, to wtedy wykonaj tego callbacka ...
});

let (set_res1, set_res2, set_res3) = taskManager.create3(move|resp1, resp2, resp3|{
    //...
});


taskManager.create(move|set_resp|{

    asynchroniczne zapytanie

}, move|set_resp|{

    asynchroniczne zapytanie

}, move|response1, response2|{

    zagregowanie obu odpowiedzi i zwrot na poziom wyżej
});


licznik instancji, będzie liczył ilość tasków
gdy watroś spadnie do zera, to znaczy że ma się wykonać callback kończący
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