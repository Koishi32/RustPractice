pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::box_smart_pointer::List::{Cons, Nil};
use crate::custom_smart_pointer;
pub fn smart_pointer_with_box(){
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let m = custom_smart_pointer::MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
}
fn hello(name: &str) {
    println!("Hello, {name}!");
}