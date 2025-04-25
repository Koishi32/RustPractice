
unsafe extern "C" {
    safe fn abs(input: i32) -> i32; //promising this fucntion will comply with memory safety
    fn pow(input: f64,power:f64) -> f64;
}

pub fn abs_c_call() {
    let my_val= abs(-43);
    println!("Absolute value of -3 according to C: {}", abs(-3));
    println!("Another: {}",my_val);

    unsafe { //Marking this block as unsafe
        println!("Power value of -3 to square according to C: {}", pow(-3.0,2.0));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
