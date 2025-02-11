pub mod estruct_used;
pub mod lib_trait_impl;
pub mod lib_default_impl;
pub mod default_impl_methods;
pub trait Summary {
    fn summarize(&self) -> String;
}