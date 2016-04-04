use std::boxed::FnBox;

pub type Callback0        = Box<FnBox()      + Send + Sync + 'static>;
pub type Callback1<A>     = Box<FnBox(A)     + Send + Sync + 'static>;
pub type Callback2<A,B>   = Box<FnBox(A,B)   + Send + Sync + 'static>;
pub type Callback3<A,B,C> = Box<FnBox(A,B,C) + Send + Sync + 'static>;


pub fn callback0_exec(func: Callback0) {
    
    (func as Box<FnBox()>)();
}

pub fn callback1_exec<A>(func: Callback1<A>, arg1: A) 
    where
        A : Send + Sync + 'static {
    
    (func as Box<FnBox(A)>)(arg1);
}

pub fn callback2_exec<A,B>(func: Callback2<A,B>, arg1: A, arg2: B) 
    where
        A : Send + Sync + 'static,
        B : Send + Sync + 'static {
    
    (func as Box<FnBox(A,B)>)(arg1, arg2);
}


pub fn callback3_exec<A,B,C>(func: Callback3<A,B,C>, arg1: A, arg2: B, arg3: C) 
    where
        A : Send + Sync + 'static,
        B : Send + Sync + 'static,
        C : Send + Sync + 'static {
    
    (func as Box<FnBox(A,B,C)>)(arg1, arg2, arg3);
}