use std::fs::File;
use std::io::ErrorKind;
pub fn verify_for_errors_with_match(){
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            //Only creates a new files if the error was
            // NotFound
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

pub fn use_closure_unwrape_or_else() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

pub fn unwrap_panic_call(){
    let greeting_file = File::open("hello.txt").unwrap();
    // on result ok -> value inside of Ok is returned
    // in result err -> panic! macro is called
}

pub fn panic_call() {
    panic!("crash and burn");
}
pub fn expect_panic_call() { //lets you provide an error message for panic
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}