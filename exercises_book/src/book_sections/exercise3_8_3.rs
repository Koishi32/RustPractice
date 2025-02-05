/*
Using a hash map and vectors, create a text interface to allow a 
user to add employee names to a department in a company; for example, 
“Add Sally to Engineering” or “Add Amir to Sales.” Then let the user 
retrieve a list of all people in a department or all people in the 
company by department, sorted alphabetically. (;_;)
*/
use std::collections::HashMap;

use crate::input_mod::string_input;
use crate::input_mod::wait_for_enter;
pub fn text_interface(){
	println!("Welcome\nChoose an option:");
	let global_map: HashMap<String, String> = HashMap::new();
	choose_option(global_map);
}

fn choose_option(mut map : HashMap<String,String>){
	
	println!("1) Add employee to deparment / Update employee deparment");
	println!("2) List all employees");
	println!("3) List employees in deparment");
	println!("4) Exit");
	let user_string = string_input();
    match user_string{
    	val if val == "1".to_string() => {
    		add_employee_to_department(&mut map);
    		wait_for_enter();
    		choose_option(map);
    	},
    	val if val == "2".to_string() => {
    		list_all_alphabetic(&map);
    		wait_for_enter();
    		choose_option(map);
    	},
    	val if val == "3".to_string() => {
    		list_by_department_alphabetic(&map);
    		wait_for_enter();
    		choose_option(map);
    	},
    	val if val == "4".to_string() => {
    		println!("Exiting");
    	},
    	String { .. } => {println!("Option not valid");
    	choose_option(map);
    	},
    }
}

fn add_employee_to_department(map :&mut HashMap<String,String>){
	println!("Adding employee...");
	println!("Insert your employee's name: ");
	let name = string_input();
	println!("Insert your employee's deparment: ");
	let dep = string_input();
	let prev = map.insert(name.clone(),dep);
	match prev{
		Some(dep_prev) =>{
			println!("Employee {:?} entry updated from {} to {:?}",name,dep_prev,map.get(&name));
		},
		None =>{
			println!("New employee entry added: {:?} : {:?}",name,map.get(&name));
		},
		
	}
	/*let new_entry =map.entry(name.clone()).or_insert(dep);
	println!("Entry added {:?} : {:?}",name,new_entry);*/
	
	//println!("Entry added {:?} , {:?}",name,map.get(&name));
}

fn sort_map(map :&HashMap<String,String>) -> Vec<(&String, &String)>{
	let mut vec:Vec<_> = map.into_iter().collect();
	vec.sort_by(|a, b| a.0.cmp(&b.0));
	vec
}

fn list_all_alphabetic(map :&HashMap<String,String>){
	println!("Listing all employees");
	//vec!=map.
	let new_vec=sort_map(&map);
	for (key,value) in new_vec{
		println!("{:?} : {:?}",key,value);
	}
}

fn list_by_department_alphabetic(map :&HashMap<String,String>){
	println!("Listing employees in deparment");
	println!("Insert the department you want to check");
	let dep_to_search = string_input();
	let new_vec=sort_map(&map);
	for (key,value) in new_vec{
		if value.to_string() == dep_to_search{
			println!("{:?} : {:?}",key,value);
		}
	}
}