#![allow(unused)]
//Allows the affected reference to live for the entire duration of the program.
pub fn static_test() {
let s: &'static str = "I have a static lifetime.";
}