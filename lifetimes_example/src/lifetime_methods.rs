struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

///It could al be defined in a single impl block
/*
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
*/
pub fn lifetime_with_methods() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let mesg = "Testing the announce method\nLevel:".to_string() + &i.level().to_string();
    // &i.level().to_string() is borrowed as a &str, 
    //The lifetime of the reference returned 
    //by the method is the same as the lifetime of self, 
    //because we declared the lifetime of the 
    //return value and the lifetime of self to be the same 
    //in the definition of the ImportantExcerpt struct.
    i.announce_and_return_part(&mesg);

    //Improved version
    let mesg_improved = format!("Testing the announce method\nLevel:{}", i.level());
    i.announce_and_return_part(&mesg_improved);
}