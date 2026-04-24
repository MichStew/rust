fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes(); 

  for(i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i]; // return the slice 
    }
  }
  &s // default return address of string 
}


fn main() {
    println!("Hello, world!");
    let s = String::from("hello world"); 

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("first word is {}. the second word is {}", hello, world);

    let first = first_word(&s); 
    println!("{} is the first word, as found by first_word function",first);
    
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]); // so slice will hold 2 & 3, why do we pass &[2,3] ? just saying
                               // references 2 and 3? 

}
