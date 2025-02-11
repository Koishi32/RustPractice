pub use crate::estruct_used;

impl Summary for estruct_used::NewsArticle {}

pub trait Summary {
    fn summarize(&self) -> String {
        println!("This a default implementation of summary");
        String::from("(Read more...)")
    }
}

impl Summary for estruct_used::Tweet {
    fn summarize(&self) -> String {
        println!("This an overrride implementation of summary");
        format!("{}: {}", self.username, self.content)
    }
}