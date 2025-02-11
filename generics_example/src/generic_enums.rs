use core::fmt;
use std::{io::ErrorKind, num::IntErrorKind};
use crate::generic_enums::fmt::Error;
// Example of generic enumerator
// Some can hold a value of any type
#[derive(Debug)]
enum SecretOption<T> {
    Some(T),
    None,
}
// Example of multiple generic enumerator
// In this example, secret_Result, holds two different values
enum SecretResult<T, E> {
    Ok(T),
    MyErr(E),
}
impl <T:fmt::Display, E:fmt::Display> fmt::Display for SecretResult<T,E>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SecretResult::Ok(v) => write!(f, "{}", v),
            SecretResult::MyErr(v) => write!(f, "{}", v),
        }
    }
}
impl<T:std::fmt::Debug,E:std::fmt::Debug> SecretResult<T,E> {
    fn check_myself(&self){
        match self{
            SecretResult::Ok(value)=>{
                println!("The current value is: {:?}",value);
            }
            SecretResult::MyErr(err)=>{
                println!("The current error is: {:?}",err);
            }
        }
    }
    
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    /*fn xy(&self) -> &(T,T){
        let tupple = (&self.x.clone(), &self.y.clone());
        &tupple
    }*/
    fn xy(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
    
}

pub fn do_stuff() {
    let p = Point { x: 5, y: 10 };

    println!("just p.x = {}", p.x());
    
    let p = Point { x: 10, y: 20 };
    let (x, y) = p.xy();
    println!("x: {}, y: {}", x, y);

    let secret_opt = SecretOption::Some(20);
    println!("Secret Option: {:?}",secret_opt);
    let secret_res:SecretResult<i32, ErrorKind>= SecretResult::MyErr(ErrorKind::AlreadyExists);
    println!("Secret Result format print: {}",secret_res);
    secret_res.check_myself();
}