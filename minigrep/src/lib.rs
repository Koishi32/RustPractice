use std::error::Error;
use std::fs;
use std::env;
pub struct  Config{
    pub query:String,
    pub file_path:String,
    pub ignore_case : bool,
}
impl Config {
   pub fn build( mut args: impl Iterator<Item = String>,)->Result<Config,&'static str>{
    let _ignore_case = env::var("IGNORE_CASE").is_ok(); // get from env_var
    if args.size_hint()<(3,Some(3)){
        return Err("Not enough arguments");
    }
    println!("Y => case sensitive search\nN => case Insensitive search");
    /* 
    if args.size_hint()<(4,Some(4)){
        println!("Y => case sensitive search\nN => case Insensitive search");
            if vec[3] == "N" || vec[3]== "Y"{
                //fourth_argument=true;
                let option = &vec[3];
                println!("Option choosed: {}",option);
                ignore_case= match option.as_str() {
                    "N" => true,
                    "Y" => false,
                    _ => true,
                }
            }else{
                println!("4th option not valid");
            }
    }*/
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case:bool = match args.next() {
            Some(arg) => {
                if arg=="Y".to_string(){
                    false
                }else if  arg=="N".to_string(){
                    true
                }else{
                    return Err("Wrong argument");
                }
            },
            None => return Err("Didn't get a file path"),
        };

        

        /* 
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        if vec.len()<3{
            return Err("Not enough arguments");
        }
        if vec.len()==4{
            println!("Y => case sensitive search\nN => case Insensitive search");
            if vec[3] == "N" || vec[3]== "Y"{
                //fourth_argument=true;
                let option = &vec[3];
                println!("Option choosed: {}",option);
                ignore_case= match option.as_str() {
                    "N" => true,
                    "Y" => false,
                    _ => true,
                }
            }else{
                println!("4th option not valid");
            }
        }
        let query = vec[1].to_string();
        let file_path = vec[2].to_string();
        
        */
        Ok(
            Config{
                query,
            file_path,
            ignore_case,
            }
        )
    }
}

pub fn run (config: Config) -> Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    //println!("With text:\n{}", content);
    let results =  if !config.ignore_case {
        search(&config.query,&content)
    }else{
        search_case_insensitive(&config.query, &content)
    };
    for line in results{
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    /*
    let mut results= Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query.to_lowercase()){
            results.push(line);
        }
    }
    results
    */
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));

    }
    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query, contents))
    }
}
