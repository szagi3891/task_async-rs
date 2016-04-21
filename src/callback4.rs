use std::boxed::FnBox;

pub type Callback<A,B,C,D> = Box<FnBox(A,B,C,D) + Send + Sync + 'static>;

pub fn exec<A,B,C,D>(func: Callback<A,B,C,D>, arg1: A, arg2: B, arg3: C, arg4: D)
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static {
    
    (func as Box<FnBox(A,B,C,D)>)(arg1, arg2, arg3, arg4);
}

pub struct CallbackBox<A,B,C,D>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static {
    
    func : Callback<A,B,C,D>
}

impl<A,B,C,D> CallbackBox<A,B,C,D>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static {
    
    pub fn exec(self, arg1: A, arg2: B, arg3: C, arg4: D) {
        exec(self.func, arg1, arg2, arg3, arg4);
    }
}

pub fn new<A,B,C,D>(func: Callback<A,B,C,D>) -> CallbackBox<A,B,C,D>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static {
    
    CallbackBox {
        func : func,
    }
}
