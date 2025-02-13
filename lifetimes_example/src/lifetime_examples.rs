use crate::functions::longest;

pub fn borrow_str(){
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

pub fn scope1() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}

pub fn scope2_error() {
    let string1 = String::from("long string is long");
    let result;
    {
        let _string2 = String::from("xyz");
        // Produces error since string2 is dropped before result is used
        //result = longest(string1.as_str(), string2.as_str());
    }
    result = longest(string1.as_str(), "abc");
    println!("The longest string is {result}");
}
