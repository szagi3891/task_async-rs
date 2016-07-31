use std::boxed::FnBox;

pub type Callback<A,B,C,D,E> = Box<FnBox(A,B,C,D,E) + Send + Sync + 'static>;

pub fn exec<A,B,C,D,E>(func: Callback<A,B,C,D,E>, arg1: A, arg2: B, arg3: C, arg4: D, arg5: E)
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static ,
        E : Send + Sync + 'static {
    
    (func as Box<FnBox(A,B,C,D,E)>)(arg1, arg2, arg3, arg4, arg5);
}
/*
pub struct CallbackBox<A,B,C,D,E>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static ,
        E : Send + Sync + 'static {
    
    func : Callback<A,B,C,D,E>
}

impl<A,B,C,D,E> CallbackBox<A,B,C,D,E>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static ,
        E : Send + Sync + 'static {
    
    pub fn exec(self, arg1: A, arg2: B, arg3: C, arg4: D, arg5: E) {
        exec(self.func, arg1, arg2, arg3, arg4, arg5);
    }
}

pub fn new<A,B,C,D,E>(func: Callback<A,B,C,D,E>) -> CallbackBox<A,B,C,D,E>
	where
		A : Send + Sync + 'static ,
        B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static ,
        E : Send + Sync + 'static {
    
    CallbackBox {
        func : func,
    }
}
*/