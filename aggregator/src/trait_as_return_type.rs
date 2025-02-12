// We can use the trait as the only criteria to get a type of data
// we can only return one type of trait
pub use crate::trait_impl;
pub use crate::estruct_used;
use self::trait_impl::Summary;

pub fn returns_summarizable() -> impl Summary {
    estruct_used::Tweet {
        username: String::from("Returner"),
        content: String::from(
            "this type has the Summary trait",
        ),
        reply: false,
        retweet: false,
    }
}