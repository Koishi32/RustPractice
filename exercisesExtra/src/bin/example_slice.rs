fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    println!("Example with : etc 323 323");
    let my_string = String::from("etc 323 323");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    println!("{_word}");
    let _word = first_word(&my_string[..]);
    println!("{_word}");
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);
    println!("{_word}");

    println!("Example with : this is a literal");
    let my_string_literal = "this is a literal";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    println!("{_word}");
    let _word = first_word(&my_string_literal[..]);
    println!("{_word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);
    println!("{_word}");
}