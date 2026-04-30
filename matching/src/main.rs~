#[derive(Debug)]
enum usState {
    Alabama, 
    Alaska, 
}

enum Coin {
    Penny, 
    Nickel, 
    Dime, 
    Quarter(usState), 
}

fn plus_one(x : Option<i32>) -> Option<i32> {  // so if option evaluates to none we return none?
                                               // else return some i + 1? I understand how but now
                                               // why. 
  match x {
      None => None, 
      Some(i) => Some(i +1),
  }
}
fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => {println!("Lucky Penny lol"); 1}, 
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { println!("State quarter from {state:?}!"); 25},
        }
    }

fn main() {
    println!("a quarter is {} cents", value_in_cents(Coin::Quarter(usState::Alaska)));

    let five = Some(5); 
    let six = plus_one(five);
    let none = plus_one(None); 
}
