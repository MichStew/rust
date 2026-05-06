fn main() {
    let number_list = vec![33, 65, 27, 223, 63];
    
    let result = largest_finder(&number_list); // returnst largest
    
    println!("the largest number is {result}");
  }

fn largest_finder(list: &[i32]) -> &i32 {
  let mut largest = &list[0]; // address of an address, no?
  for number in list {
      if number > largest {
          largest = number;
     }}
  largest
  }
