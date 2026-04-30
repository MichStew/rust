#[derive(Debug)]

enum IPAddr {
    V4(u8, u8, u8, u8), // advantage over structs  
    V6(String), // both variants will have associated string values 
                // these take string and return struct?
}

enum IPAddrKind { V4, V6 }

#[derive(Debug)]
enum Message { 
    Quit, 
    Move { x: i32, y: i32 }, 
    Write(String), 
    ChangeColor(i32, i32, i32),
}

impl Message {
    
    fn call(&self) {
         // method of the message enum type 
         println!("I have something to say {self:#?}"); 
    }}


fn route (_ip_kind: IPAddrKind) {
 // do something
}


fn main() {
    let _four =  IPAddrKind::V4; 
    let _six = IPAddrKind::V6;

    //let home = IPAddr::V4(String::from("127.0.0.")); 

    //let test = Message::ChangeColor(34,34,34); // should output the numbers maybe??
    //test.call();
    
    let some_number = Some(5); 
    let some_char = Some('e'); 
    let absent_number: Option<i32> = None; 
}
