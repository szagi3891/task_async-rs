use std::boxed::FnBox;

pub type Callback<A> = Box<FnBox(A) + Send + Sync + 'static>;

pub fn exec<A>(func: Callback<A>, arg1: A)
	where
		A : Send + Sync + 'static {
    
    (func as Box<FnBox(A)>)(arg1);
}

pub struct CallbackBox<A>
	where
		A : Send + Sync + 'static {
    
    func : Callback<A>
}

impl<A> CallbackBox<A>
	where
		A : Send + Sync + 'static {
    
    pub fn exec(self, arg1: A) {
        exec(self.func, arg1);
    }
}

pub fn new<A>(func: Callback<A>) -> CallbackBox<A>
	where
		A : Send + Sync + 'static {
    
    CallbackBox {
        func : func,
    }
}
