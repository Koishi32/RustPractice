use std::fmt;

//#[derive(Debug)]
struct ColorJacket(i32, i32, i32);
impl fmt::Display for ColorJacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R: {}\nG: {}\nB: {}", self.0, self.1,self.2)
    }
}
//#[derive(Debug)]
struct Location(i32, i32, i32);
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {}\nY: {}\nZ: {}", self.0, self.1,self.2)
    }
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    my_location: Location,
    my_color: ColorJacket,
}
fn build_user(email: String, username: String) -> User {
    let temp_lo = Location(2,3,2);
    let temp_col=ColorJacket(1,1,1);
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
        my_location : temp_lo,
        my_color : temp_col,
    }
}

fn main() {
    let mut user1 = build_user(String::from("a@mail.com"),String::from("Myusername"));
    user1.email = String::from("anotheremail@example.com");
    println!("First User Data");
    println!("{}",user1.username);
    println!("{}",user1.email);
    println!("{}",user1.active);
    println!("{}",user1.sign_in_count);
    println!("{}",user1.my_location);
    println!("{}",user1.my_color);
    
    let temp_col = ColorJacket(0,1,0);
    let temp_loc = Location(1,1,-5);
    let user2 = User{
    	email:String::from("NewEmail@mail.com"),
    	my_color :temp_col,
    	my_location:temp_loc,
    	//username:String::from("NewUserName"),
    	..user1
    };
   
    println!("User Data Updated");
    println!("{}",user2.username);
    println!("{}",user2.email);
    println!("{}",user2.active);
    println!("{}",user2.sign_in_count);
    println!("{}",user2.my_location);
    println!("{}",user2.my_color);
}