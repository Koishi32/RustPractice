pub use crate::trait_impl;

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
    println!("\n2 Blog updates:\n1.-\n{}\n2.-\n{}", item1.summarize(),item2.summarize());
}