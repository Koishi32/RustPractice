use std::fmt;

#[derive(Debug)]
pub struct Asparagus {}
#[derive(Debug)]
pub struct Potato{pub name:String,pub size:u32}

impl fmt::Display for Potato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\nSize: {}", self.name, self.size)
    }
}