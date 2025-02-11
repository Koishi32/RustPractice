pub use crate::estruct_used;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}


impl Summary for estruct_used::Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}