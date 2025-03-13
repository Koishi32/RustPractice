use std::sync::mpsc;
use crate::simultaneous_code::prelude::*;

//multiple producer, single consumer
pub fn example_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //line below doesn't work since val was already sent
        //down the channel
        //println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

pub fn main_thread_communication(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got message from thread: {received}");
    }
}

pub fn multiple_threads(){
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move ||{
        let last_var = String::from("Last Message for you");
        thread::sleep(Duration::from_secs(8));
        tx2.send(last_var).unwrap();
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}