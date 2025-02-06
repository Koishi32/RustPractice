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
	println!("Insert your employee's name followed by their deparment");
	let input_user = string_input();
	let vector_data : Vec<_>= input_user.trim().split(' ').collect();
	/*println!("Insert your employee's deparment: ");
	let dep = string_input();*/
	if vector_data.len()!= 2{
		println!("Insert valid data");
		choose_option(map.clone());
	}else{
		let name = vector_data[0];
	let dep = vector_data[1];
	let prev = map.insert(name.to_string(),dep.to_string());
	match prev{
		Some(dep_prev) =>{
			println!("Employee {:?} entry updated from {} to {}",name,dep_prev,map.get(name).unwrap());
		},
		None =>{
			println!("New employee entry added: {:?} : {}",name,map.get(name).unwrap());
		},
		
	}
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

fn sort_map_by_value<'a>(map : &'a HashMap<String,String>, value :&'a String ) -> Vec<(&'a String, &'a String)>{
	let mut new_vec:Vec<_> = map.into_iter().filter_map(|(k,v)| if v == value
	{Some((k,v))} else {None}).collect();
	new_vec.sort_by(|a, b| a.0.cmp(&b.0));
	new_vec
}

fn list_all_alphabetic(map :&HashMap<String,String>){
	println!("Listing all employees");
	//vec!=map.
	let new_vec=sort_map(&map);
	if new_vec.len()==0 {
		println!("No registered employees");
	}else {
		for (key,value) in new_vec{
			println!("{:?} : {:?}",key,value);
		}
	}
}

fn list_by_department_alphabetic(map :&HashMap<String,String>){
	println!("Listing employees in deparment");
	println!("Insert the department you want to check");
	let dep_to_search = string_input();
	let new_vec=sort_map_by_value(&map,&dep_to_search);
	if new_vec.len()==0 {
		println!("No registered employees in {} deparment",dep_to_search);
	}else {
		for (key,value) in new_vec{
			println!("{:?} : {:?}",key,value);
		}
	}
}