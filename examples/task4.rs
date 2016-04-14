extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::Task;


fn process_task(task: Task<String>) {
    
    let (set_resp1, set_resp2, set_resp3, set_resp4) = task.async4(Box::new(move|task: Task<String>, response1: Option<String>, response2: Option<String>, response3: Option<String>, response4: Option<String>|{
        
        match (response1, response2, response3, response4) {
            
            (Some(resp1), Some(resp2), Some(resp3), Some(resp4)) => {
                
                task.result(format!("Response end {}, {}, {}, {}", resp1, resp2, resp3, resp4));
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

    task_async::spawn("task4".to_owned(), move||{

        task_async::sleep(4000);
        println!("task4");

        set_resp4.result("resp4".to_owned());
    });
}



fn main() {
    
    println!("Task start (4)");
    
    let (down_producer, donw_consumer) = channel();
    
        

    let task = Task::new(Box::new(move|result : Option<String>|{

        match result {
            Some(value) => println!("{}", value),
            None => println!("dane niekompletne"),
        };

        down_producer.send(()).unwrap();
    }));

    process_task(task);
    
    
    
    let _ = donw_consumer.get();

    println!("end program");
}

