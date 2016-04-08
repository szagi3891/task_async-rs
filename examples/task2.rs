use std::thread;
use std::thread::sleep;
use std::time::Duration;

extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::{TaskManager, Task};


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
    thread::spawn(move||{

        sleep(Duration::from_millis(2000));
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
    thread::spawn(move||{

        sleep(Duration::from_millis(3000));
        println!("wykonało się trzecie asynchroniczne zapytanie");

        set_resp3.result("odpowiedź trzecia".to_owned());
    }); 


                        //asynchroniczne zapytanie
    thread::spawn(move||{


        sleep(Duration::from_millis(4000));
        println!("wykonało się czwarte asynchroniczne zapytanie");

        set_resp4.result("odpowiedź czwarta".to_owned());
    });
}


fn main() {
    
    println!("Task start (2)");
    
    let (down_producer, donw_consumer) = channel();
    
    {
        
        let varible = "some value".to_owned();
        
        
        let task_manager = TaskManager::new(Box::new(move||{

            println!("runung task is 0 -> {}", varible);
            down_producer.send(()).unwrap();
        }));
        
        
        
        let task = task_manager.task(Box::new(move|result : Option<String>|{
            
            match result {
                Some(value) => println!("{}", value),
                None => println!("dane niekompletne"),
            };
        }));
        
        process_task(task);
        
        
    }
    
    
    let _ = donw_consumer.get();

    println!("end program");
}

