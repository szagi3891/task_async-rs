#![feature(fnbox)]


extern crate channels_async;

mod counter;
mod task;
mod result2;
mod types;
mod taskmanager;


pub use counter::Counter;
pub use task::Task;
pub use taskmanager::TaskManager;


//TODO - sprawdzić clippy
        //dodać dev zależność




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


