use std::boxed::FnBox;

pub type Callback0            = Box<FnBox()          + Send + Sync + 'static>;
pub type Callback1<A>         = Box<FnBox(A)         + Send + Sync + 'static>;
pub type Callback2<A,B>       = Box<FnBox(A,B)       + Send + Sync + 'static>;
pub type Callback3<A,B,C>     = Box<FnBox(A,B,C)     + Send + Sync + 'static>;
pub type Callback4<A,B,C,D>   = Box<FnBox(A,B,C,D)   + Send + Sync + 'static>;
pub type Callback5<A,B,C,D,E> = Box<FnBox(A,B,C,D,E) + Send + Sync + 'static>;

//Callback0::box_new()  tworzy nowy CallbackBox
//Callback0::Box -> box, z metodą exec
//Callback0::Callback       -> typ
//Callback0::exec(Callback0::Callback)

pub struct CallbackBox0 {
    func : Callback0
}

impl CallbackBox0 {
    
	pub fn new(func: Callback0) -> CallbackBox0 {
		CallbackBox0 {
			func : func,
		}
	}
    pub fn exec_func(func: Callback0) {
        (func as Box<FnBox()>)();
    }
    
    pub fn exec(self) {
        CallbackBox0::exec_func(self.func);
    }
}

pub struct CallbackBox1<A>
	where
		A : Send + Sync + 'static {
	
    func : Callback1<A>
}

impl<A> CallbackBox1<A>
	where
		A : Send + Sync + 'static {

	pub fn new(func: Callback1<A>) -> CallbackBox1<A> {
		CallbackBox1 {
			func : func,
		}
	}
	
    pub fn exec_func(func: Callback1<A>, arg1: A) {
        (func as Box<FnBox(A)>)(arg1);
    }
                
	pub fn exec(self, arg1: A) {
        CallbackBox1::exec_func(self.func, arg1);
	}
}


pub struct CallbackBox2<A,B>
	where
		A : Send + Sync + 'static ,
		B : Send + Sync + 'static {
	
    func : Callback2<A,B>
}

impl<A,B> CallbackBox2<A,B>
	where
		A : Send + Sync + 'static ,
		B : Send + Sync + 'static {

	pub fn new(func: Callback2<A,B>) -> CallbackBox2<A,B> {
		CallbackBox2 {
			func : func,
		}
	}
	
    pub fn exec_func(func: Callback2<A,B>, arg1: A, arg2: B) {
        (func as Box<FnBox(A,B)>)(arg1, arg2);
    }
            
	pub fn exec(self, arg1: A, arg2: B) {
		CallbackBox2::exec_func(self.func, arg1, arg2);
	}
}


pub struct CallbackBox3<A,B,C>
	where
		A : Send + Sync + 'static ,
		B : Send + Sync + 'static ,
        C : Send + Sync + 'static {
	
    func : Callback3<A,B,C>
}

impl<A,B,C> CallbackBox3<A,B,C>
	where
		A : Send + Sync + 'static ,
		B : Send + Sync + 'static ,
        C : Send + Sync + 'static {

	pub fn new(func: Callback3<A,B,C>) -> CallbackBox3<A,B,C> {
		CallbackBox3 {
			func : func,
		}
	}
	
    pub fn exec_func(func: Callback3<A,B,C>, arg1: A, arg2: B, arg3: C) {
        (func as Box<FnBox(A,B,C)>)(arg1, arg2, arg3);
    }
            
	pub fn exec(self, arg1: A, arg2: B, arg3: C) {
		CallbackBox3::exec_func(self.func, arg1, arg2, arg3);
	}
}



pub struct CallbackBox4<A,B,C,D>
	where
		A : Send + Sync + 'static ,
		B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static {
	
    func : Callback4<A,B,C,D>
}

impl<A,B,C,D> CallbackBox4<A,B,C,D>
	where
		A : Send + Sync + 'static ,
		B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static {

	pub fn new(func: Callback4<A,B,C,D>) -> CallbackBox4<A,B,C,D> {
		CallbackBox4 {
			func : func,
		}
	}
	
    pub fn exec_func(func: Callback4<A,B,C,D>, arg1: A, arg2: B, arg3: C, arg4: D) {
        (func as Box<FnBox(A,B,C,D)>)(arg1, arg2, arg3, arg4);
    }
            
	pub fn exec(self, arg1: A, arg2: B, arg3: C, arg4: D) {
		CallbackBox4::exec_func(self.func, arg1, arg2, arg3, arg4);
	}
}



pub struct CallbackBox5<A,B,C,D,E>
	where
		A : Send + Sync + 'static ,
		B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static ,
        E : Send + Sync + 'static {
	
    func : Callback5<A,B,C,D,E>
}

impl<A,B,C,D,E> CallbackBox5<A,B,C,D,E>
	where
		A : Send + Sync + 'static ,
		B : Send + Sync + 'static ,
        C : Send + Sync + 'static ,
        D : Send + Sync + 'static ,
        E : Send + Sync + 'static {

	pub fn new(func: Callback5<A,B,C,D,E>) -> CallbackBox5<A,B,C,D,E> {
		CallbackBox5 {
			func : func,
		}
	}
	
    pub fn exec_func(func: Callback5<A,B,C,D,E>, arg1: A, arg2: B, arg3: C, arg4: D, arg5: E) {
        (func as Box<FnBox(A,B,C,D,E)>)(arg1, arg2, arg3, arg4, arg5);
    }
            
	pub fn exec(self, arg1: A, arg2: B, arg3: C, arg4: D, arg5: E) {
		CallbackBox5::exec_func(self.func, arg1, arg2, arg3, arg4, arg5);
	}
}


