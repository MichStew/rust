#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height 
    }
}

fn main() {
    let rect1 = (30,50); // tuple struct 
    let scale = 3;
    
    let rect2 = Rectangle {
        width : 30,
        height: dbg!(50 * scale),
    };

    // println!("rect2 is {rect2:#?}");
    println!("The area of the rectangle is {} square pixels", Rectangle::area(&rect2));
    // dbg!(&rect2);
}

fn area(rect: &Rectangle) -> u32 {  // struct is type now
    rect.width * rect.height // return final expression 
}
