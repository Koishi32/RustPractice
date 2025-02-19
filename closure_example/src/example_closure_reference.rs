use std::thread;

pub fn example2_borrowing_inmutable() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    //cariable list is still usuable after being used by the closure since it 
    // can also handle it by borrowing it
}
pub fn example3_borrow_mutuble_reference(){
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    //Next line triggers an error since the closure still owns list and borrowed it as a mutable borrow
    //println!("Check if borrow capture is still effective {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");
}


pub fn example4_closure_take_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    //Erasing the move keyword will cause an error 
    thread::spawn( move|| println!("From thread: {list:?}"))
    //since the compiler detects a possible dangling reference , since list is owned by the thread
    // and the thread can be outlived by the clusure function
    //therefore is necesary to transfer ownership of list from the thread (function)
    //to the closure
        .join()
        .unwrap();
    //next line will cause an error since the previous closure took ownership of
    // list
    //println!("After using closure {list:?}");
}