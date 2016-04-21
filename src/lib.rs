#![feature(fnbox)]


extern crate channels_async;

mod counter;
mod task;
mod result2;
mod utils;

pub mod callback0;
pub mod callback1;
pub mod callback2;
pub mod callback3;
pub mod callback4;
pub mod callback5;

pub use counter::Counter;
pub use task::Task;

pub use utils::*;



//TODO - sprawdzić clippy
        //dodać dev zależność




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


