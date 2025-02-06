use std::io::ErrorKind;
mod errors_examples;
mod  user_input;
use std::fs::{self,File};
use std::io::Write;
use std::io::Error;

use errors_examples::{examples3, examples4};
fn main() {
    /*println!("Choose example: ");
    print!("1) ejercisies 1");
    let option = user_input::string_input();*/
    //errors_examples::examples1::verify_for_errors_with_match();
    println!("propagandize error");
    let result_read = errors_examples::examples2::read_username_from_file();
    read_result_from_file(result_read);
    let result_read = errors_examples::examples2::read_username_from_file_with_operator();
    read_result_from_file(result_read);
    let result_read = errors_examples::examples2::read_username_from_file_with_operator_short();
    read_result_from_file(result_read);
    //using the standard library implementation
    let result_read =  fs::read_to_string("name_save.txt");
    read_result_from_file(result_read);
    let test = examples3::main_example();
    println!("Testing if file hello.txt can be found");
    match test {
        Ok(character) => println!("Found: {:?}",character),
        Err(err)=> println!("{err:?}"),
    }
    println!("Use of never empty result: ");
    examples3::result_always_ok();
    println!("Handling errors with our own data types: ");
    examples4::guessing_game();

}
fn read_result_from_file(result_read: Result<String,Error>){
    match result_read {
        Ok(name) => {
            println!("we got the {} name",name);
            let option = errors_examples::examples3::last_char_of_first_line(&name);
            match option {
                Some(character) => println!("{}",character),
                None=> println!("Empty"),
            }
            //println!("Last char is {}",option?);
        },
        Err(error)=>{
            match error.kind() {
                ErrorKind::NotFound =>{
                    println!("File not found\nmaking new file with name");
                    make_name_file();
                },
                other_error=>{
                    panic!("name couldn't be recovered {other_error:?}")
                },
            };
        },
    }
}

fn make_name_file()->String{
    let name = user_input::string_input();
    let mut file = match File::create("name_save.txt"){
        Ok(file) => file,
        Err(err)=>panic!("Could not create file: {err:?}"),
    };
    match file.write_all(name.as_bytes()){
        Ok(_)=>println!("Name written into file succesfully"),
        Err(err)=>panic!("Name couldn't be written: {err:?}"),
    }
    name
}
