use std::boxed::FnBox;

pub type Callback = Box<FnBox() + Send + Sync + 'static>;

pub fn exec(func: Callback) {
    (func as Box<FnBox()>)();
}

pub struct CallbackBox {
    func : Callback
}

impl CallbackBox {
    pub fn exec(self) {
        exec(self.func);
    }
}

pub fn new(func: Callback) -> CallbackBox {
    CallbackBox {
        func : func,
    }
}
