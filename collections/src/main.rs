fn main() {
let mut v = vec![1,2,3,4,5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third : Option<&i32> = v.get(2); 
match third {
  Some(third) => println!("the third element is {third}"), 
  None => println!("There is no third element"),
} 
for i in &mut v {
 *i += 50;
}


for i in &mut v {
  println!("{i}");
}

#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32), 
  Float(f64), 
  Text(String), 
  }


// storing different data types within a vector using enumerations
let row = vec![ 
  SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("blue")), 
  SpreadsheetCell::Float(10.12),
  ];

// how to print a vector with differente types? this throws an enum error :(
for i in &row {
  println!("{:?}", i);
  }

let data = "initial contents";

let s = data.to_string(); 

let s = "initial contents".to_string(); // also works inline 

let mut s = String::from("foo"); 
s.push_str("bar"); // super similar to vectors, slight changes 

let mut s1 = String::from("foo"); 
let s2 = "bar";
println!("s1 is currently {s1}");
s1.push_str(s2);
println!("s2 is {s2}, and s1 is {s1}"); // we can append without removing ownership


// string concat actually requires an owned string as left operand 
let s1 = String::from("hello, "); 
let s2 = String::from("world.");
let s3 = s1 + &s2; // &s1 throws compile error
println!("{}",s3);

// concatenating multiple strings using format! macro

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3})");






}
