use std::fmt;
//mod estruct_used;
pub use crate::estruct_used;
pub trait Summary {
    fn summarize(&self) -> String;
}

//We define what will the trait do for each of these estructs
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

impl Summary for estruct_used::Blog {
    fn summarize(&self) -> String {
        format!("{}\n   {}\n        \"{}\"", self.tittle, self.sub_tittle, self.content)
    }
}

impl fmt::Display for estruct_used::Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"User: {}\nRetweet: {} Reply: {}",self.username,self.retweet,self.reply)
    }
    
}

