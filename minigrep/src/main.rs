use std::{env, fs, process, error::Error};
use minigrep::{search, search_case_insensitive}; 

fn main() {
    let args : Vec<String> = env::args().collect(); // collect all CLA into a vector of strings
    
    let config = Config::build(&args).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1); }); // return non zero
 
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> { 
    let contents = fs::read_to_string(config.file_path)?; // to exist, or not to exist
    let results = if config.ignore_case { 
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents) 
    };

    for line in results {  // for line in the returned vector
         println!("{line}"); 
    } 

    Ok(()) // return normal tuple if good
}

pub struct Config {
    query : String, 
    file_path : String, 
    ignore_case : bool, // ...which function to call... 
}

impl Config {
    fn build(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { // path, arg1, arg2 ...
            return Err("not enough arguments");
        }
        let query = args[1].clone(); 
        let file_path = args[2].clone(); // deep copy on heap
        
        let ignore_case = env::var("IGNORE_CASE").is_ok(); // returns true or false.
        Ok(Config { query, file_path, ignore_case }) // return struct 
    }   
}


