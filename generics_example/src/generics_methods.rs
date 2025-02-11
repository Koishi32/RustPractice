struct Point<T> {
    x: T,
    y: T,
}
// implementing a function with generics on an enum
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// this imprimentation will only work with numbers of type f32  
impl Point<f32> { //
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointX<X1, Y1> {
    x: X1,
    y: Y1,
}
// Implementation of function with multiple enums on PointX
impl<X1, Y1> PointX<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointX<X2, Y2>) -> PointX<X1, Y2> {
        PointX {
            x: self.x, //My own type
            y: other.y, // Second type 
        }
    }
}

pub fn methods() {
    let p0 = Point{x:8,y:9};
    println!("Point0 x value: {}", p0.x());
    let p0f = Point{x:4.2,y:2.1};
    println!("Distance from org: {}",p0f.distance_from_origin());
    let p1 = PointX { x: 5, y: 10.4 };
    let p2 = PointX { x: "Hello", y: 'c' };

    println!("Original Data");
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);

    let p3 = p1.mixup(p2);
    println!("P3 Mixed up Data");
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}