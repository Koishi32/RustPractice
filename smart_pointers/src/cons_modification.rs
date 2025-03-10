#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<String>>, Rc<List>),
    Nil,
}
use crate::cons_modification::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn cons_mod() {
    let value = Rc::new(RefCell::new(String::from("5")));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(String::from("3"))), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(String::from("4"))), Rc::clone(&a));

    *value.borrow_mut() += "10";
    println!("Having Multiple Owners of Mutable Data ");
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}