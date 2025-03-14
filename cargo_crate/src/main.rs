//use cargo_crate_extra::kinds::PrimaryColor;
//use cargo_crate_extra::utils::mix;
use cargo_crate_extra::mix;
use cargo_crate_extra::PrimaryColor;
///it uses the items of kinds and Utils defined on the 
fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}