fn main() {
    let reference_to_nothing = dangle();
	println!("{reference_to_nothing}");
}
fn dangle() -> String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

   // &s // we return a reference to the String, s, the pointer generates an error
	s // ownership is passed to where it was called
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger! s is out of scope
