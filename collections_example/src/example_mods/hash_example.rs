use std::collections::HashMap;

pub fn hash_examples(){
	println!("HASH EXAMPLE\n");
	hash_creating_iteration();
	hash_with_string();
	println!("updating a hash map cases: ");
	overwrite_value();
	only_update_if_no_key_present();
	update_based_on_existing_value();
	}

fn hash_creating_iteration() {

    println!("Creating a hash map and accesing one of it's values: ");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}",score);

    println!("Iterating: ");
    scores.insert(String::from("Blue"), 2);
    scores.insert(String::from("Yellow"), 3);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

}

fn hash_with_string(){

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    for (key,value) in &map{
    	println!("key: {key}\nvalue: {value}");
    }
    println!("the string fields used for the map are no longer accesible by this point")
}

fn overwrite_value(){

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("Old: {scores:?}");
    scores.insert(String::from("Blue"), 25);
    println!("New: {scores:?}");
}

fn only_update_if_no_key_present(){

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("Org map: {scores:?}");
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("Map updated with entry\nif key was not present: {scores:?}");
    println!("{scores:?}");

}

fn update_based_on_existing_value(){
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
    	// inserts the words of text and rises the count
    	//
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

}