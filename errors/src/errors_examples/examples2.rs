
/*
If this function succeeds without any problems, 
the code that calls this function will receive an 
Ok value that holds a String—the username that this 
function read from the file
 */
use std::fs::File;
use std::io::{self, Read};
 pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("name_save.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_with_operator() -> Result<String, io::Error> {
    let mut username_file = File::open("name_save.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


pub fn read_username_from_file_with_operator_short() -> Result<String, io::Error> {
    let mut username = String::new();
    // Operator only works when the return type is Result<Ok,Err> 
    //or Option<T> (Some or None) or another type
    // that implements FromResidual
    File::open("name_save.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

