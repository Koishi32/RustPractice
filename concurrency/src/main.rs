use shared_concurrency_mutexs::dead_lock_example;

mod simultaneous_code;
mod thread_transfer_data;
mod shared_concurrency_mutexs;
fn main() {
    /*
    println!("Thread Examples");
    simultaneous_code::thread_example::thread_example();
    simultaneous_code::move_var_thread::move_var_to_thread();
    simultaneous_code::drop_value::drop_value_out_of_thread();
    println!("Messages between threads examples:");
    thread_transfer_data::chanel_example::example_channel();
    thread_transfer_data::chanel_example::main_thread_communication();
    thread_transfer_data::chanel_example::multiple_threads();
    println!("Managging access and Sharing among threads");
    */
    //shared_concurrency_mutexs::mutex_example::mutex_lock();
    //shared_concurrency_mutexs::mutex_example::share_value_amog_multiple_threads();
    dead_lock_example::deadlock();
}