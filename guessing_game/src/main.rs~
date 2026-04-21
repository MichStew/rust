use std::io; // very c++ vibe, this = using namespace std;
use rand::Rng; // Rng is a trait? 
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game");
  
    let secret_number = rand::thread_rng().gen_range(1..=100); // generate random number between
                                                               // 1-100 I imagine if 1..100 the max
                                                               // would be 99?
    //println!("The secret number is {secret_number}");
    loop{

    let mut guess = String::new(); 
    println!("Please input your guess.");

    io::stdin() // so then, stdin is an associated function of io. 'this is the same at std::cin
        .read_line(&mut guess) // also returns a result enum, which has variant states
                               // for error handling, result has 2 states: Ok, Err. 
                               // if we dont use expect its possible that errors could occur
        .expect("Failed to read line");

    println!("You guessed: {guess}"); // we should use try catch except etc, but this will work to
                                      // just crash upon result returning err instead of ok.
    // this is about to be very different from other languages I have used 
    // this is almost like a case statement 
    // => defined the code that should be run if match arm returns true??

    // now tell rust that guess is an unsigned 32 bit number and take string covert
    let guess: u32 = match guess.trim().parse() { 
        Ok(num) => num,
        Err(_) => continue,
    };
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),  // so this is an arm? arm = case?
        Ordering::Greater => println!("Too big!"), // like if(){ return x;} 
        Ordering::Equal => { println!("You win!"); break; }
    }
    }
}

// rust has standard things in scope by default, this is called the prelude in rust 
// std::io is not in scope by default so we have to include it just like #include<stdio> , however
// in rust we use 'use' instead of '#include'
// :: indicates an associated function of a type so new is an associated function of string type


