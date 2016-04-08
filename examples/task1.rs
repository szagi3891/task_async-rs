use std::thread;
use std::thread::sleep;
use std::time::Duration;

extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::{TaskManager, Task};


fn process_task(task: Task<String>) {
    
    let set_resp = task.async1(Box::new(move|task: Task<String>, response: Option<String>|{

                                        //zagregowanie obu odpowiedzi
        match response {

            Some(dat) => {

                let mess: String = format!("zbiorczy callback '{}'", dat);
                
                task.result(mess);
            },

            _ => {}
        }

    }));
    
                        //asynchroniczne zapytanie
    thread::spawn(move||{

        sleep(Duration::from_millis(2000));
        println!("wykonało się asynchroniczne zapytanie");
        
        set_resp.result("odpowiedź tegoż zapytania".to_owned());
    });
}


fn main() {
    
    println!("Task start (1)");
    
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

