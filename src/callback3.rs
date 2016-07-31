use std::boxed::FnBox;

pub type Callback<A,B,C> = Box<FnBox(A,B,C) + Send + Sync + 'static>;

pub fn exec<A,B,C>(func: Callback<A,B,C>, arg1: A, arg2: B, arg3: C)
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
    
    (func as Box<FnBox(A,B,C)>)(arg1, arg2, arg3);
}

/*
pub struct CallbackBox<A,B,C>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
    
    func : Callback<A,B,C>
}

impl<A,B,C> CallbackBox<A,B,C>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
    
    pub fn exec(self, arg1: A, arg2: B, arg3: C) {
        exec(self.func, arg1, arg2, arg3);
    }
}

pub fn new<A,B,C>(func: Callback<A,B,C>) -> CallbackBox<A,B,C>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
    
    CallbackBox {
        func : func,
    }
}
*/