use std::boxed::FnBox;

pub type Callback<A,B> = Box<FnBox(A,B) + Send + Sync + 'static>;

pub fn exec<A,B>(func: Callback<A,B>, arg1: A, arg2: B)
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static {
    
    (func as Box<FnBox(A,B)>)(arg1, arg2);
}

pub struct CallbackBox<A,B>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static {
    
    func : Callback<A,B>
}

impl<A,B> CallbackBox<A,B>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static {
    
    pub fn exec(self, arg1: A, arg2: B) {
        exec(self.func, arg1, arg2);
    }
}

pub fn new<A,B>(func: Callback<A,B>) -> CallbackBox<A,B>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static {
    
    CallbackBox {
        func : func,
    }
}
