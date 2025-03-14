mod original_code;
mod generic_implement;
mod generic_enums;
mod generic_struct;
mod generics_methods;
fn main (){
    println!("Orignal example / no enums");
    original_code::gen_example();
    println!("Generic Points Example");
    generic_struct::test_points();
    println!("Genric implement example - largest number");
    generic_implement::generic_type_operation();
    println!("Generic enum example");
    generic_enums::do_stuff();
    println!("Genric methods example");
    generics_methods::methods();
}
