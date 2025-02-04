/*
Using a hash map and vectors, create a text interface to allow a 
user to add employee names to a department in a company; for example, 
“Add Sally to Engineering” or “Add Amir to Sales.” Then let the user 
retrieve a list of all people in a department or all people in the 
company by department, sorted alphabetically. (;_;)
*/
use std::collections::HashMap;
use std::io;
pub fn text_interface(){
	let global_map = HashMap::<&str,&str>::new();
	println!("Welcome\nChoose an option:");
	choose_option();
}

fn choose_option(){
	println!("1) Add employee to deparment / Update employee deparment");
	println!("2) List all employees");
	println!("3) List employees in deparment");
	println!("4) Exit");
	let user_string = string_input();
    match user_string{
    	val if val == "1".to_string() => add_employee_to_department(),
    	val if val == "2".to_string() => list_all_alphabetic(),
    	val if val == "3".to_string() => list_by_department_alphabetic("test".to_string()),
    	val if val == "4".to_string() => {println!("Exiting");},
    	String { .. } => {println!("Option not valid");
    	choose_option();
    	},
    }
}

fn string_input()->String{
	let mut user_string = String::new();
    io::stdin()
    .read_line(&mut user_string)
    .expect("Failure To read line");
    let user_string=user_string.trim();
    user_string.to_string()
}
fn add_employee_to_department(){
	println!("Adding employee");
}

fn sort_map(){

}

fn list_all_alphabetic(){
	println!("Listing all employees");
}

fn list_by_department_alphabetic(deparment_value:String){
	println!("Listing employees in deparment");

}