use std::env;
use std::fs; 

fn main() {
    let args : Vec<String> = env::args().collect(); // collect all CLA into a vector of strings
    
    let (query, file_path) = parse_arguments(&args); 
    
    println!("{query} and {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("File not accessible"); 
    println!("with text :\n{contents}");
}

fn parse_arguments( arguments : &[String]) -> (&str, &str) {
    let query = &arguments[1];
    let file_path = &arguments[2];

    (query, file_path)
}

