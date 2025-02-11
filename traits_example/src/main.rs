mod traits_example1;
mod traits_example2;
mod traits_example3;
fn main() {
    println!("Implementation of Traits");
    traits_example1::example1();
    println!("Traits default implementation");
    traits_example2::example2();
    println!("Traits implementation with methods");
    traits_example3::example3();
}