fn main() {
    println!("Hello, world!");
    print_labeled_measurement(5,'h'); // argument since it doesnt change?
    let x = five();
    println!("the value of x is: {x}");
    let x = plus_one(x); 
    println!("the value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1  // expressions dont need semicolons, this will indeed confuse me in the future... lol
}
