use std::sync::{Arc, Mutex};
use std::thread;

pub fn deadlock() {
    // Create two mutexes
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    // Clone the mutexes so they can be accessed in multiple threads
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);

    // Spawn the first thread
    let thread1 = thread::spawn(move || {
        // Acquire the lock on the first mutex
        let lock1 = m1.lock().unwrap();
        println!("Thread 1: Acquired lock on mutex1");

        // Simulate some work
        println!("-------------------------------");
        thread::sleep(std::time::Duration::from_millis(3000));
        println!("-------------------------------");

        // Attempt to acquire the lock on the second mutex
        println!("Thread 1: Trying to acquire lock on mutex2");
        let lock2 = m2.lock().unwrap();

        // Access the data
        println!("Thread 1: Acquired lock on mutex2");
        println!("Thread 1: mutex1: {}, mutex2: {}", lock1, lock2);
    });

    // Clone the mutexes again for the second thread
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);

    // Spawn the second thread
    let thread2 = thread::spawn(move || {

        // Attempt to acquire the lock on the first mutex
        println!("Thread 2: Trying to acquire lock on mutex1");
        let lock1 = m1.lock().unwrap();
        println!("Thread 2: Acquired lock on mutex1");

        // Simulate some work
        println!("-------------------------------");
        thread::sleep(std::time::Duration::from_millis(3000));
        println!("-------------------------------");

        // Acquire the lock on the second mutex
        println!("Thread 2: Trying to acquire lock on mutex2");
        let lock2 = m2.lock().unwrap();
        println!("Thread 2: Acquired lock on mutex2");
        
        // Access the data
        println!("Thread 2: mutex1: {}, mutex2: {}", lock1, lock2);
    });

    // Wait for the threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
}
