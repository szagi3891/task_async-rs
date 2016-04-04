use std::thread;
use std::thread::sleep;
use std::time::Duration;

extern crate task_async;
extern crate channels_async;

use channels_async::channel;
use task_async::{TaskManager, Task};

