#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    dbg!(&rect1.width);
    dbg!(&rect1.height);
    println!("rect1 is {rect1:#?}");
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    dbg!(rectangle.width * rectangle.height)
}