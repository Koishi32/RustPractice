use std::fmt::Display;

pub use crate::trait_impl;

use self::trait_impl::Summary;

// Trait bound syntax, it does the same but it makes use of 
//generic Types
pub fn notify_generic<T: trait_impl::Summary>(item: &T) {
    println!("Notification!: {}", item.summarize());
}

// will take any traits that implement Summary
pub fn notify_any_summary_type(item1: &impl trait_impl::Summary, item2: &impl trait_impl::Summary) {
    println!("\n2 Notification gotten:\n1.-\n{}\n2.-\n{}", item1.summarize(),item2.summarize());
}
// Will only take traits of the same type that implement the Summary
pub fn notify_specific_summary_type<T: trait_impl::Summary>(item1: &T, item2: &T) {
    println!("\n2 Updates:\n1.-\n{}\n2.-\n{}", item1.summarize(),item2.summarize());
}

pub fn notify_with_multiple_traits(item1: &(impl trait_impl::Summary+ Display)){
    println!("\nTweet info with display trait: {}",item1);
    println!("\nTweet summary with summary trait: {}",item1.summarize())
}
pub fn notify_with_multiple_traits_generic<T: trait_impl::Summary+Display>(item1:&T){
    println!("\nTweet info with display trait:\n{}",item1);
    println!("\nTweet summary with summary trait:\n{}",item1.summarize()) 
}
pub fn notify_with_multiple_traits_generic_where<T>(item:&T)
where T:Summary+Display
{
    println!("\nTweet info with display trait:\n{}",item);
    println!("\nTweet summary with summary trait:\n{}",item.summarize()) 
}