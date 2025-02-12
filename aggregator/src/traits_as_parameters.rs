pub use crate::trait_impl;
// only items with a trait that implements Sumarry will be acepted and executed
pub fn notify(item: &impl trait_impl::Summary) {
    println!("Breaking news!\n{}", item.summarize());
}