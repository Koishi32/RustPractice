// like this the x,y data has to be of the same type
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
} 
#[derive(Debug)]
struct PointHybrid<T, U> {
    x: T,
    y: U,
}

// For diferent types of data we should use multiple generic type parameters.

pub fn test_points() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = PointHybrid { x: 5, y: 4.0 };
    println!("Generic Integer: {:?}",integer);
    println!("Generic float: {:?}",float);
    println!("Generic hybrid points: {:?}",integer_and_float);
}