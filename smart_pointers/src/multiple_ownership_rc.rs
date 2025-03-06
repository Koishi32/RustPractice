//rc: reference counting
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::multiple_ownership_rc::List::{Cons, Nil};
use std::rc::Rc;

pub fn rc_use_multiple_reference() {
    //Rc::clone isn't a deep copy
    //instead it increases the count of references to an multiple owned value
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

pub fn rc_dropping_increasing(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}