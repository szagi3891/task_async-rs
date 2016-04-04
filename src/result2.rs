use std::sync::Arc;
use std::mem;
use std::boxed::FnBox;

use counter::Counter;
use types::Callback2;

pub struct Result2<A,B>
    where
        A : Send + Sync + 'static,
        B : Send + Sync + 'static {
    
    pub _counter : Arc<Counter>,                        //ta zmienne jest tylko do przetrzymywania referencji
    pub complete : Callback2<Option<A>, Option<B>>,
    pub result1  : Option<A>,
    pub result2  : Option<B>,
}


impl<A,B> Drop for Result2<A,B>
    where
        A : Send + Sync + 'static,
        B : Send + Sync + 'static {

    fn drop(&mut self) {

        let empty_clouser = Box::new(|_:Option<A>, _:Option<B>|{});
        let complete      = mem::replace(&mut self.complete, empty_clouser);
        let result1       = mem::replace(&mut self.result1, None);
        let result2       = mem::replace(&mut self.result2, None);
        
        (complete as Box<FnBox(Option<A>,Option<B>)>)(result1, result2);
    }
}
