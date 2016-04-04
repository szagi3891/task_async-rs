use std::boxed::FnBox;

pub type Callback0        = Box<FnBox()      + Send + Sync + 'static>;
pub type Callback1<A>     = Box<FnBox(A)     + Send + Sync + 'static>;
pub type Callback2<A,B>   = Box<FnBox(A,B)   + Send + Sync + 'static>;
pub type Callback3<A,B,C> = Box<FnBox(A,B,C) + Send + Sync + 'static>;

