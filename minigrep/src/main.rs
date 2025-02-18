use std::env;
use std::process;
use minigrep::Config;
fn main() {
    println!("Can use a fourth command or an environment value, the fourth command takes priority");
    let args: Vec<String>=  env::args().collect();
    //let config =  Config::build(&args).unwrap();
    let config = Config::build(&args).unwrap_or_else(|err|
        {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);
    //run(config);
    //dbg!(args);
    if let Err(e) = minigrep::run(config){
        //panic!("Error: {}",e);
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}

