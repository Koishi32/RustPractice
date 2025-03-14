#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn example5() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|any_name_reference_to_rectangule| any_name_reference_to_rectangule.width);
    //above implements FnMut since the closure is called multiple times
    //according to the number of keys
    println!("{list:#?}");
}


pub fn example6() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    //37 line is an error since the first call to the closure
        //takes owenership of value and moves it to sort_operations, 
        //making the trait of the closure FnOnce
        //for the next calls of the enclosure, value is no longer avalaible
        //sort_operations.push(value);

    list.sort_by_key(|r| {
        //sort_operations.push(value);
        sort_operations.push(&value);
        //the line above indicate a reference is being used and no owenership
        //is being takes, the trait od the closure is determined to be 
        // FnMut and multiple uses of value is allowed
        r.width
    });
    println!("{list:#?}");
}

pub fn example7() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        // New body of closure doesn’t move values out of the environment. 
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}