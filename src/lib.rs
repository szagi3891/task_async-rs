#![feature(fnbox)]

mod counter;
mod task;
mod task_pool;
mod result2;
mod utils;

mod callback0;
mod callback1;
mod callback2;
mod callback3;
mod callback4;
mod callback5;

mod thread_pool;

pub use counter::Counter;
pub use task::Task;

pub use task_pool::TaskPool;
pub use thread_pool::ThreadPool;


                                //TODO - do usunięcia
pub use utils::*;



//TODO - sprawdzić clippy
        //dodać dev zależność




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


