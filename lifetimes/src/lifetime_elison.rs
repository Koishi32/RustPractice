/* 
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
*/

//The code above is equivalent to 

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

pub fn example_elison() {
    let string = String::from("Hello World");
    let word = first_word(&string);
    println!("The first word is: {}", word);
}