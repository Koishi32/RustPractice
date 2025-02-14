mod lifetime_examples;
mod functions;
mod lifetime_with_estructs;
mod lifetime_elison;
mod lifetime_methods;
mod lifetime_static;
mod lifetime_generic_trait;
fn main() {
    lifetime_examples::borrow_str();
    lifetime_examples::scope1();
    lifetime_examples::scope2_error();
    lifetime_with_estructs::estruct_example();
    lifetime_elison::example_elison();
    lifetime_methods::lifetime_with_methods();
    lifetime_static::static_test();
    lifetime_generic_trait::test_all_three();
}
