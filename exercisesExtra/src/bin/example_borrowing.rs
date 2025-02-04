fn main() {
    let mut s = String::from("hello");

   // change(s);
	change(&mut s);
}

//fn change(mut some_string:String) {
fn change(some_string:&mut String){
    some_string.push_str(", world"); //error if we are borrwing
	println!("{some_string}");
}
