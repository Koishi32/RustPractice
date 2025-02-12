mod traits_example1;
mod traits_example2;
mod traits_example3;
mod traits_example4;
fn main() {
    println!("\nImplementation of Traits");
    traits_example1::example1();
    println!("\nTraits default implementation");
    traits_example2::example2();
    println!("\nTraits implementation with methods");
    traits_example3::example3();
    println!("\nTraits as parameters");
    traits_example4::example4();
}