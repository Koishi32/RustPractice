// The weak reference will be dropped even if it's count
//is >0 as long as the strong references involved
// reach 0
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, //This node doesn't own their parent
    //The weak count of the variable above won't affect when this node
    //will be dropped
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn weak_example() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}