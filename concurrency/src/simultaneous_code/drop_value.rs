use crate::simultaneous_code::prelude::*;

pub fn drop_value_out_of_thread(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move|| {
        println!("Here's a vector: {v:?}");
    });

    //below line, dropping the v value will cause the thread above to
    // to try to acceses to dealocated memory incurring an error
    //drop(v);
    handle.join().unwrap();
}