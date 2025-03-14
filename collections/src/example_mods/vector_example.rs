#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vector_examples(){
    vector_creation();
    get_element_vector();
    test_iterations();
    vectors_with_enum_for_diferrent_types();
    
}

fn vector_creation(){
    println!("VECTOR EXAMPLE\n");
    let mut v: Vec<i32> = Vec::new();
    v = vec![1, 2, 3];
    println!("{:?}",v);
    v.push(4);
    v.push(5);
    v.push(6);
    println!("After push: {:?}",v);
}

fn get_element_vector(){
    let v = vec![1, 2, 3, 4, 5];

    let second: &i32 = &v[1];
    println!("The second element is {second}");

    //let v = vec![1, 2, 3, 4, 5];
    let third: Option<&i32> = v.get(2);
    match third {
        Some(&third) if third == 4 => println!("You found a 4"),
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    let value: Option<&i32> = v.get(3);
    match value {
        Some(&value) if value == 4 => println!("You found a 4"),
        Some(value) => println!("The element is {value}"),
        None => println!("There is no element."),
    }
}

fn test_iterations(){
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        println!("Value: {i}");
        *i += 50; //Note the use of a deference operator
        println!("New value: {i}");
    }
}

fn vectors_with_enum_for_diferrent_types(){
        let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row{
        println!("Element: {:?}",i);
        match i{
            SpreadsheetCell::Int(value) => println!("{} It's an Int type in the vector",value),
            SpreadsheetCell::Text(value) => println!("{} It's a Text type in the vector",value),
            SpreadsheetCell::Float(value) => println!("{} It's a Float type in the vector",value),
        }
    }
}