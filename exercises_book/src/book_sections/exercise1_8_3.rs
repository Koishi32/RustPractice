/*
Given a list of integers, use a vector and return the median (when sorted, 
the value in the middle position) and mode (the value that occurs most 
often; a hash map will be helpful here) of the list.
*/
use std::collections::HashMap;
use num::integer::div_floor;

pub fn do_median_mode(){
	let mut integers = vec![1,5,6,3,2,5,6,12,32,12,-2,12,-14,4,2,4,7,8,2,1,2,2];
	integers.sort();
	println!("From this vector: {:?}",integers);
	let median = integers[div_floor(integers.len(),2)];
	println!("The median is: {}",median);
	let mut count_map = HashMap::new();
	let mut _highest_count=0;
	let mut _current_mod=0;
	for integer_instance in integers{
		
		let apearances = count_map.entry(integer_instance.to_string()).or_insert(0);
		*apearances+=1;

		if *apearances > _highest_count {
			_highest_count = *apearances;
			_current_mod=integer_instance;
		}
	}
	println!("The mode is: {}",_current_mod);

}