// using ? operator to get Option<T>
//Some(val) -> there is a last char
use std::error::Error;
use std::fs::File;

pub fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

//This function won't compile since ? doesn't have anywhere compatible to return to
// main only returns ()
/*
pub fn main(){
    let greeting_file = File::open("hello.txt")?;
}
*/
// Box<dyn Error> is any kind of error
pub fn main_example() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}
/*
When a main function returns a Result<(), E>, the executable will 
exit with a value of 0 if main returns Ok(()) and will exit with 
a nonzero value if main returns an Err value.
*/
use std::net::IpAddr;
pub fn result_always_ok(){
    let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
    println!("Hard coded Ip Addres {:?} ",home);
}
// since IP is hardcoded there is no posibility of
// Result returning an Err variant

