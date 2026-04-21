fn main() {
    let mut counter = 0; 
    
    let result  = loop {
         counter += 1; 
         
         if counter == 10 {
             break counter * 2; // put the return we want after the break, this is how to get data
         }                       // outside of loops 
         };
    println!("the value of the result is: {result}");
}
