use thread_pool::{ThreadPool};
use task::{Task, new_task};
use callback0;
use callback1;

#[derive(Clone)]
pub struct TaskPool {
    thread_pool: ThreadPool<TaskPoolParam>
}

enum TaskPoolParam {
    Exec(callback0::CallbackBox),
}

impl TaskPool {

    pub fn new(workers: u16) -> TaskPool {
        
        let thread_pool = ThreadPool::new(workers, Box::new(move||{
            
            Box::new(move|param: TaskPoolParam|{

                exec(param);
            })
        }));
    
        TaskPool{
            thread_pool: thread_pool
        }
    }
    
    pub fn task<TaskParam: Send + Sync + 'static>(&self, func: callback1::Callback<Option<TaskParam>>) -> Task<TaskParam> {
        new_task(func, self.clone())
    }
    
    pub fn run(&self, func : callback0::CallbackBox) {
        self.thread_pool.run(TaskPoolParam::Exec(func));
    }
}

fn exec(param: TaskPoolParam) {
    
    match param {
        TaskPoolParam::Exec(callback_box) => {
            callback_box.exec();
        }
    }
}
