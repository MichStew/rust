fn first_word(s: &String) {
  let bytes = s.as_bytes(); 

  for(i, &item) in bytes.iter.enumerate() {
    if item == b' ' {
      return i; 
    }
  }
  s.len() // default return length of s 
}


fn main() {
    println!("Hello, world!");
}
