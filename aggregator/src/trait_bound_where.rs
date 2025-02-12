
pub use crate::trait_impl;

use std::fmt::{Display,Debug};
/*
Simplifying the next funtion
*/
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    unimplemented!()
}
// TO
fn some_function_improv<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    unimplemented!()
}