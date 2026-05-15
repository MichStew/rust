use std::fmt::Display;

fn main() {
    let x = String::from("abcd"); 
    let y = "abc"; 
    let result = longest( &x, &y );
    println!("{result}");

    let novel =  String::from("Call me Ishmael. Some years ago... example text");
    let first_sentence = novel.split('.').next().unwrap();
    let i = Excerpt {
        part : first_sentence,
    };
}

fn longest<'a>( x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){x } else { y } }

struct Excerpt<'a> {
    part: &'a str, 
}

fn longest_with_annnouncement<'a, T> ( 
    x : &'a str, 
    y : &'a str, 
    ann : T, 
) -> &'a str where T : Display, {
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y } }
