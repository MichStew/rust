pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a : u64) -> u64 {
    a + 2
}

struct Rectangle {
    height: u32, 
    width: u32,
}

impl Rectangle { 
    fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height // height and width are bigger? Hold
    }
    fn cant_hold(&self, other : &Rectangle ) -> bool {
        self.width < other.width && self.height < other.height
    }
}

struct Guess {
    input : u32 
}

impl Guess { 
    pub fn test(g : u32) {
        let number = g; 
        if number > 100 {
            panic!("guess value must be less than 100, not {number}");
        } else if number < 0{
            println!("guess value must be a positive number, not {number}");
        }}} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than")]
    fn panic_guess(){
        let outofbounds = Guess::test(200);
    }

    #[test] 
    fn test_two () {
        assert_eq!(add_two(2), 4); // can I just call this directly? 
    }

    #[test]
    fn exploration () {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn another () {
        let result = add(2,2);
        assert_eq!(result, 5);
    }
    #[test] 
    fn larger_can_hold_smaller() { 
        let larger = Rectangle {
            height : 15, 
            width : 20, 
        };
        let smaller = Rectangle { 
            height : 7, 
            width : 10,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test] 
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            height : 15, 
            width : 20, 
        };
        let smaller = Rectangle { 
            height : 7, 
            width : 10,
        };
        assert!(smaller.cant_hold(&larger));
    }

}
