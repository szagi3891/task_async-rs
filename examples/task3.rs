extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::{TaskManager, Task};


fn process_task(task: Task<String>) {
    
    let (set_resp1, set_resp2, set_resp3) = task.async3(Box::new(move|task: Task<String>, response1: Option<String>, response2: Option<String>, response3: Option<String>|{
        
        match (response1, response2, response3) {
            
            (Some(resp1), Some(resp2), Some(resp3)) => {
                
                task.result(format!("Response end {}, {}, {}", resp1, resp2, resp3));
            }, 
            _ => {
                //nic
            }
        }
    }));
        
    task_async::spawn("task1".to_owned(), move||{

        task_async::sleep(3000);
        println!("task1");
        
        set_resp1.result("resp1".to_owned());
    });

    task_async::spawn("task2".to_owned(), move||{

        task_async::sleep(2000);
        println!("task2");

        set_resp2.result("resp2".to_owned());
    });

    task_async::spawn("task3".to_owned(), move||{

        task_async::sleep(1000);
        println!("task3");

        set_resp3.result("resp3".to_owned());
    });
}



fn main() {
    
    println!("Task start (3)");
    
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

