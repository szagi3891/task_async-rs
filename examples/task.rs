#![feature(fnbox)]

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::sync::Arc;
use std::boxed::FnBox;

extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::{TaskManager, Callback1};


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
        
        
        task_manager.async_run(Box::new(move|set_resp: Callback1<String>|{
            
                                //asynchroniczne zapytanie
            thread::spawn(move||{
                
                sleep(Duration::from_millis(2000));
                println!("wykonało się pierwsze asynchroniczne zapytanie");
                
                //set_resp("odpowiedź pierwsza".to_owned());
                
                (set_resp as Box<FnBox(String)>)("odpowiedź pierwsza".to_owned());
            });
            
        }), Box::new(move|set_resp: Callback1<String>|{
            
                                //asynchroniczne zapytanie
            thread::spawn(move||{
        
                sleep(Duration::from_millis(2000));
                println!("wykonało się drugie asynchroniczne zapytanie");
                
                //set_resp("odpowiedź druga".to_owned());
                
                (set_resp as Box<FnBox(String)>)("odpowiedź druga".to_owned());
            });
            
        }), Box::new(move|response1, response2|{
            
            //zagregowanie obu odpowiedzi i zwrot na poziom wyżej
            
            
            match (response1, response2) {
                
                (Some(dat1), Some(dat2)) => {
            
                    println!("zbiorczy callback {}, {}", dat1, dat2);
                },
                
                _ => {
                    
                    println!("dane niekompletne");
                }
            }
                    
        }));
    }
    
    
    let _ = donw_consumer.get();

    println!("end program");
}

