#![feature(fnbox)]

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::boxed::FnBox;

extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::{TaskManager, Task};


fn main() {
    
    println!("Task");
    
    let (down_producer, donw_consumer) = channel();
    
    {
        
        let varible = "some value".to_owned();
        
        let end_fun : Box<FnBox() + Send + Sync> = Box::new(move||{
            
            println!("cleanig state -> {}", varible);
        });
        
        
        let task_manager = TaskManager::new(Box::new(move||{

            println!("runung task is 0");
            (end_fun as Box<FnBox()>)();
            down_producer.send(()).unwrap();
        }));
        
        
        
        let main_task = task_manager.task(Box::new(move|result : Option<String>|{
            
            match result {
                Some(value) => println!("{}", value),
                None => println!("dane niekompletne"),
            };
        }));
        
        
        
        let (set_resp1, set_resp2) = main_task.async(Box::new(move|main_task: Task<String>, response1: Option<String>, response2: Option<String>|{
            
                                            //zagregowanie obu odpowiedzi
            match (response1, response2) {
                
                (Some(dat1), Some(dat2)) => {
                    
                    let mess: String = format!("zbiorczy callback '{}', '{}'", dat1, dat2);
                    
                    main_task.result(mess);
                },
                
                _ => {
                    
                    main_task.result("dane niekompletne".to_owned());
                }
            }
            
        }));
        
                            //asynchroniczne zapytanie
        thread::spawn(move||{
            
            sleep(Duration::from_millis(2000));
            println!("wykonało się pierwsze asynchroniczne zapytanie");
            
            set_resp1.result("odpowiedź pierwsza".to_owned());
        });
        
        
        
        
                                    //TODO - testowo
        thread::spawn(move||{
            
            sleep(Duration::from_millis(3000));
            println!("wykonało się drugie asynchroniczne zapytanie");
            
            set_resp2.result("odpowiedź druga".to_owned());
        });
        
        
        
        /*
        let (set_resp3, set_resp4) = task_manager.async_run(Box::new(move|response1, response2|{
            
            match (response1, response2) {
                (Some(resp1), Some(resp2)) => {
                    
                    (set_resp2 as Box<FnBox(String)>)(format!("zbiorcza druga : {}, {}", resp1, resp2));
                },
                _ => {
                    (set_resp2 as Box<FnBox(String)>)(format!("zbiorcza druga : None"));
                }
            }
            
        }));
        
            
        
                            //asynchroniczne zapytanie
        thread::spawn(move||{
            
            sleep(Duration::from_millis(3000));
            println!("wykonało się trzecie asynchroniczne zapytanie");
            
            (set_resp3 as Box<FnBox(String)>)("odpowiedź trzecia".to_owned());
        }); 
            
            
                            //asynchroniczne zapytanie
        thread::spawn(move||{
            
            
            sleep(Duration::from_millis(4000));
            println!("wykonało się czwarte asynchroniczne zapytanie");
            
            (set_resp4 as Box<FnBox(String)>)("odpowiedź czwarta".to_owned());
        });
        */
    }
    
    
    let _ = donw_consumer.get();

    println!("end program");
}

