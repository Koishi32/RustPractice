mod example_closure_expensive;
mod example_closure_reference;
mod example_closure_fn_traits;
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) //this closure has no parametes
        //unwrap_or_else will return Some if it exits
        // otherwise it will call the closure || and returns the vulue
        //returnes by the clousure

        //the same but with match
        /*
        match user_preference {
            Some(color)=> color,
            None=> self.most_stocked(),
        }*/
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    iventory_example();
    examples();
}

fn iventory_example(){
    println!("Example Iventory");
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
fn examples(){
    example_closure_expensive::example1();
    example_closure_reference::example2_borrowing_inmutable();
    example_closure_reference::example3_borrow_mutuble_reference();
    example_closure_reference::example4_closure_take_ownership();
    example_closure_fn_traits::example5();
    example_closure_fn_traits::example6();
    example_closure_fn_traits::example7();
}