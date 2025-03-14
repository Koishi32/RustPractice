use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub struct Guess {
        value: i32,
    }
    
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }
    
            Guess { value }
        }
    
        pub fn value(&self) -> i32 {
            self.value
        }
    }

pub fn guessing_game() {

    
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        // --snip--

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Non valid option"); 
            continue;
            },
        };

        let guess = Guess::new(guess);
        /*if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }*/

        match guess.value().cmp(&secret_number) {
            // --snip--
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}