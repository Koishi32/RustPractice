mod lifetime_examples;
mod functions;
mod lifetime_with_estructs;
mod lifetime_elison;
mod lifetime_methods;
fn main() {
    lifetime_examples::borrow_str();
    lifetime_examples::scope1();
    lifetime_examples::scope2_error();
    lifetime_with_estructs::estruct_example();
    lifetime_elison::example_elison();
    lifetime_methods::lifetime_with_methods();
}
