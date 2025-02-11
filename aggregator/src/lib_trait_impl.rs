//mod estruct_used;
pub use crate::estruct_used;
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for estruct_used::Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for estruct_used::NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

