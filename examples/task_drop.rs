extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::TaskManager;



fn main() {
    
    println!("Task start (drop)");
    
    let (down_producer, donw_consumer) = channel();
    
    {
        
        let varible = "some value".to_owned();
        
        
        let task_manager = TaskManager::new(Box::new(move||{

            println!("runung task is 0 -> {}", varible);
            down_producer.send(()).unwrap();
        }));
        
        
        
        let _ = task_manager.task(Box::new(move|result : Option<String>|{
            
            match result {
                Some(value) => println!("{}", value),
                None => println!("dane niekompletne"),
            };
        }));
        
        
    }
    
    
    let _ = donw_consumer.get();

    println!("end program");
}

