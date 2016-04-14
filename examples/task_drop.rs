extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::Task;



fn main() {
    
    println!("Task start (drop)");
    
    let (down_producer, donw_consumer) = channel();
    


    let _ = Task::new(Box::new(move|result : Option<String>|{

        match result {
            Some(value) => println!("{}", value),
            None => println!("dane niekompletne"),
        };

        down_producer.send(()).unwrap();
    }));
        
    
    
    let _ = donw_consumer.get();

    println!("end program");
}

