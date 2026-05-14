pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new(); // this must be mutable to append
    
    for line in contents.lines() { // contents is a struct returned my main? 
        if line.contains(query) { // mostly just unsure how to append to this vector 
            matches.push(line) // holy smokes I actually kinda know what im doing  
        } 
    }
    matches // return the vector with all matching elements 
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); 
    let mut matches = Vec::new(); 
    for line in contents.lines() { 
        if line.to_lowercase().contains(&query) {
            matches.push(line)
        }}
    matches 
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents)); 
    }
    #[test]
    fn case_sensitive() {
        let query = "rUsT"; 
        let contents = "\
Rust:
Safe, fast, productive. 
Pick three. 
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}
