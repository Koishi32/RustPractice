// Any instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl <'a>  ImportantExcerpt <'a> {
    fn print_value(&self){
        println!("The value is: {}", self.part);
    }
}

pub fn estruct_example() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    i.print_value();
}