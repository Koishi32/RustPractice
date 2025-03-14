use aggregator::trait_bounds_as_conditions;
pub fn example6(){
    let new_point = trait_bounds_as_conditions::Pair::new(2,3);
    //Will only allow to ust the cmp_display method if the 
    //Data has the traits of display and compare
    new_point.cmp_display();

    let new_strings = trait_bounds_as_conditions::Pair::new("Alan","Zack");
    //Will only allow to ust the cmp_display method if the 
    //Data has the traits of display and compare
    new_strings.cmp_display();
}