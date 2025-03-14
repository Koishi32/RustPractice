/*
Convert strings to pig latin. The first consonant of each word is 
moved to the end of the word and ay is added, so first becomes irst-fay. 
Words that start with a vowel have hay added to the end instead 
(apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
*/
use std::io;

pub fn test_pig_latin(){
	println!("Insert string to turn to pig latin: ");
    let mut user_string = String::new();
    io::stdin()
    .read_line(&mut user_string)
    .expect("Failure To read line");
    let user_string=user_string.trim();
    println!("{}",actual_pig_latin(user_string));
}
fn actual_pig_latin(my_string : &str)-> String{
	let list_vowel = ['a','e','i','o','u'];
	let mut final_string = String::new();
	for word in my_string.split_whitespace(){
		match get_first_char(word){
			Some(char) => {println!("First character is: {}",char);
			if list_vowel.contains(&char){
					final_string.push_str(&word);
					final_string.push_str("-hay");
				}
			else{
					let new_end ="-".to_owned()+ &char.to_string() + "ay";
					//final_string.push_str(&word[1..]);
					final_string.push_str(&string_no_first_element(word));
					final_string.push_str(&new_end);
				}
			},
			None => println!("Non valid string"),
		}
		final_string.push_str(" ");
	}
	final_string
}
fn get_first_char(my_string:&str)->Option<char>{
	my_string.chars().next()
}
fn string_no_first_element(my_string:&str)->String{
	let rest_string = my_string.chars().skip(1).collect();
	rest_string
}