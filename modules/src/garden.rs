use std::fmt;
pub mod vegetables;
#[derive(Debug)]
pub struct GardenInfo {
	pub name: String,
	pub number_id: u32,
}


impl fmt::Display for GardenInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nWith: Id number: {}", self.name, self.number_id)
    }
}

impl GardenInfo{
	pub fn especial_sale(&self) -> String{
	let message = String::from("Potatoes 34% off");
	message
	}
}

