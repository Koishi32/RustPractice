use crate::garden::vegetables::Asparagus;
use crate::garden::vegetables::Potato;
//use crate::garden::vegetables::Potato;
use crate::garden::GardenInfo;
pub mod garden;
fn main() {
    let current_garden = GardenInfo{
        name:String::from("Cheap Veggies"),
        number_id: 2,
    };
    println!("Hello my garden is \n{current_garden}");
    let mes =current_garden.especial_sale();
    println!("This garden's special sale is :\n{}",mes);
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
    let plant2 = Potato {
        name:String::from("Patata1"),
        size:12,

    };
    println!("I'm growing the follwing!\n{plant2}");
}