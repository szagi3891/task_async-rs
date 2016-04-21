#![feature(fnbox)]


extern crate channels_async;

mod counter;
mod task;
mod result2;
mod utils;
mod callback;



pub use counter::Counter;
pub use task::Task;

pub use utils::*;

pub use callback::callback0;
pub use callback::callback1;
pub use callback::callback2;
pub use callback::callback3;
pub use callback::callback4;
pub use callback::callback5;


//TODO - sprawdzić clippy
        //dodać dev zależność




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


