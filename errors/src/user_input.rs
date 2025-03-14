use std::io;

pub fn string_input()->String{
	let mut user_string = String::new();
    io::stdin()
    .read_line(&mut user_string)
    .expect("Failure To read line");
    let user_string=user_string.trim();
    user_string.to_string()
}

pub fn wait_for_enter(){
    let mut user_string = String::new();
    println!("Press Enter To continue");
    io::stdin()
    .read_line(&mut user_string)
    .expect("Failure To read line");
    //let user_string=user_string.trim();
    //user_string.to_string()
}