mod aggregator; 
use aggregator::{SocialPost, Summary, default};
fn main() {
    let number_list = vec![33, 65, 27, 223, 63];
    
    let result = largest_type(&number_list); // returnst largest
    
    println!("the largest number is {result}");
    
    // now has the mixup method
    // lol so i defined a struct with 2 diff types and then passed 2 of the same type
    let point1 = Point {
        x: 2, 
        y: 3.0
    };

    let point2 = Point {
        x : "hello",
        y : -45,
    };


    let result = point1.mixup(point2); // can I pass a struct?
    println!("the result of the mixup method from point 1 with point 2 is: {result:?}"); 


let default_test = default {
    headline : String::from("none"),
    content : String::from("none"),
    read : false, 
};

println!("the usage of the default trait: {}", default_test.summarize()); 

let test = SocialPost {
    username : String::from("Jackson Dawson"), 
    content : String::from("The sea otter jumps from the 23rd story into the glistening green lake at the bottom of the rivene. 123 feet of pure mystery. questions echo...how did an otter get up there...how will he land...only on tubi."),
    reply : false, 
    repost : false,
};

println!("1 new post: {}",test.summarize());


  }

#[derive(Debug)]
struct Point <X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1,Y1> {
    fn mixup<X2, Y2> (self, other : Point<X2,Y2>) -> Point<X1,Y2> {
      Point{
        x : self.x,
        y : other.y,
      }
    }
} 
    
fn largest_finder(list: &[i32]) -> &i32 { // borrow the vector
  let mut largest = &list[0]; // address of the first elemente
  for number in list {
      if number > largest {
          largest = number;
     }}
  largest
  }
fn largest_char(list : &[char]) -> &char {
  let mut largest = &list[0];
  
  for i in list{
      if i > largest {
          largest = i;
      }}

  largest
}

use std::cmp::PartialOrd;
fn largest_type<T: PartialOrd>(list : &[T]) -> &T { //only real difference in convention in the <T> the rest is
                                   // just typing syntax replaced by whatever I want it to be 
    let mut largest = &list[0]; 
    for item in list {
        if item > largest { 
            largest = item; 
        }}
    largest
}

