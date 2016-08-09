use thread_pool::{ThreadPool};
use task::{Task, new_task};
use callback1;

#[derive(Clone)]
pub struct TaskPool {
    thread_pool: ThreadPool<TaskPoolParam>
}

#[derive(Clone)]
pub enum TaskPoolParam {
    Exec,
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
}

fn exec(param: TaskPoolParam) {
    //dane do wykonania w wątku
}

/*
use std::io::prelude::Read;
use std::fs::{self, File};
use std::path::Path;
use std::io;

use task_async::{self, Task, ThreadPool};

pub type FilesData = Result<Vec<u8>, io::Error>;

enum ParamFile {
    Get(String, Task<FilesData>),
}

pub struct ApiFile {
    thread_pool: ThreadPool<ParamFile>
}

impl ApiFile {
    pub fn get_file(&self, path_src: String, task: Task<FilesData>) {

        let param = ParamFile::Get(path_src, task);
        self.thread_pool.run(param);
    }
}

fn exec(param_file: ParamFile) {
    
    match param_file {
        ParamFile::Get(path, task) => {
            get_inner_file(path, task);
        }
    }
}


*/