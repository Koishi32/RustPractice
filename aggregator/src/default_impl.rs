pub use crate::estruct_used;

impl Summary for estruct_used::NewsArticle {}
impl Summary for estruct_used::Blog {}

pub trait Summary {
    fn summarize(&self) -> String {
        println!("This a default implementation of summary for NewsArticle");
        String::from("(Read more...)")
    }
}
impl Summary for estruct_used::Tweet {
    fn summarize(&self) -> String {
        println!("This an overrride implementation of summary for Tweet");
        format!("{}: {}", self.username, self.content)
    }
}