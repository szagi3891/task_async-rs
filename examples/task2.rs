extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::Task;


fn process_task(task: Task<String>) {
    
    let (set_resp1, set_resp2) = task.async2(Box::new(move|task: Task<String>, response1: Option<String>, response2: Option<String>|{

                                        //zagregowanie obu odpowiedzi
        match (response1, response2) {

            (Some(dat1), Some(dat2)) => {

                let mess: String = format!("zbiorczy callback '{}', '{}'", dat1, dat2);
                
                task.result(mess);
            },

            _ => {}
        }

    }));

                        //asynchroniczne zapytanie
    task_async::spawn("async_query".to_owned(), move||{

        task_async::sleep(2000);
        println!("wykonało się pierwsze asynchroniczne zapytanie");

        set_resp1.result("odpowiedź pierwsza".to_owned());
    });




    let (set_resp3, set_resp4) = set_resp2.async2(Box::new(move|main_task: Task<String>, response1: Option<String>, response2: Option<String>|{

        match (response1, response2) {
            (Some(resp1), Some(resp2)) => {

                main_task.result(format!("zbiorcza druga : {}, {}", resp1, resp2));
            },
            _ => {}
        }

    }));



                        //asynchroniczne zapytanie
    task_async::spawn("async3".to_owned(), move||{

        task_async::sleep(3000);
        println!("wykonało się trzecie asynchroniczne zapytanie");

        set_resp3.result("odpowiedź trzecia".to_owned());
    }); 


                        //asynchroniczne zapytanie
    task_async::spawn("async4".to_owned(), move||{


        task_async::sleep(4000);
        println!("wykonało się czwarte asynchroniczne zapytanie");

        set_resp4.result("odpowiedź czwarta".to_owned());
    });
}


fn main() {
    
    println!("Task start (2)");
    
    let (down_producer, donw_consumer) = channel();
    
    
    
    let varible = "some value".to_owned();


    let task = Task::new(Box::new(move|result : Option<String>|{

        match result {
            Some(value) => println!("complete: {} -> {}", value, varible),
            None => println!("dane niekompletne"),
        };

        down_producer.send(()).unwrap();
    }));

    process_task(task);
    
    
    
    let _ = donw_consumer.get();

    println!("end program");
}

