use std::sync::Arc;

extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::Counter;


fn run_timeout(inst: Arc<Counter>, name: String, timeout: u64) {

    task_async::spawn(name.clone(), move||{
        
        let _ = inst.clone();
        
        task_async::sleep(timeout);
        
        println!("end task -> {}", name);
    });
}


fn main() {
    

    let (down_producer, donw_consumer) = channel();

    {
        let varible = "some value".to_owned();
        
        let inst = Counter::new(Box::new(move||{

            println!("counter is 0 -> {}", varible);
            down_producer.send(()).unwrap();
        }));


        run_timeout(inst.clone(), "task1".to_owned(), 1000);
        run_timeout(inst.clone(), "task2".to_owned(), 2000);
        run_timeout(inst.clone(), "task3".to_owned(), 3000);
    }

    let _ = donw_consumer.get();

    println!("end program");
}

