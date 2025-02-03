mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
/*
use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}*/
//Fixes 1

/*mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}*/
//Fixes 2
/*
mod customer {
    use super::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
*/
//Example idomatic use paths 
/*
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
*/
// Idiomatic way to bring full path
/*
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
*/
//Bringing item with the same Name
// specifying their parents on every call
/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
*/
// Giving them new names
/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
*/
// Re exporting a function with pub to make it avaible on a new scope
/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
*/

// Nested paths
//FROM
/*
use std::cmp::Ordering;
use std::io;
*/
//TO
/*
use std::{cmp::Ordering, io};
*/
//Combaining submodules and their subparts
//FROM
/*
use std::io;
use std::io::Write;
*/
//TO
/*
use std::io::{self, Write};
*/
//Using glob operator to bring all items defined in a path to your scope
//use std::collections::*;
