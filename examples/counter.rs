use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::sync::Arc;

extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::Counter;


fn run_timeout(inst: Arc<Counter>, name: String, timeout: u64) {

    thread::spawn(move||{
        
        let _ = inst.clone();
        
        sleep(Duration::from_millis(timeout));
        
        println!("kończę {}", name);
    });
}


fn main() {
    

    let (down_producer, donw_consumer) = channel();

    {
        let inst = Counter::new(Box::new(move||{

            println!("counter is 0");
            down_producer.send(()).unwrap();
        }));


        run_timeout(inst.clone(), "task1".to_owned(), 1000);
        run_timeout(inst.clone(), "task2".to_owned(), 2000);
        run_timeout(inst.clone(), "task3".to_owned(), 3000);
    }

    let _ = donw_consumer.get();

    println!("kończę program");
}

