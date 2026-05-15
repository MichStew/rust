use std::fmt::Display; 

pub trait Summary {
    fn summarize_author(&self) -> String; 

    fn summarize(&self) -> String {
        format!("Read More from {}", self.summarize_author())
    }

}


fn returns_summarizable() -> impl Summary {
    SocialPost {
        username : String::from("Horse Ebooks"), 
        content : String::from("of course, as you probably already know, people"), 
        reply : false, 
        repost : false, 
    }
}


pub fn notify(item1 : &impl Summary, item2 : &impl Summary ) {
   println!("Breaking news! {},{}: ", item1.summarize(), item2.summarize());
}

pub struct default {
    pub headline: String,
    pub content: String, 
    pub read: bool, 
}

impl Summary for default {
  fn summarize_author(&self) -> String {
        String::from("unused")
    }

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String, 
    pub author: String, 
    pub content: String, 
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        String::from("this should not get used")
    }
}

pub struct SocialPost { 
    pub username: String, 
    pub content: String,
    pub reply: bool, 
    pub repost: bool, 
}

pub struct Pair<T> { 
    x : T,
    y : T, 
}

impl<T> Pair<T> {
    pub fn new(x : T, y : T) -> Self { // return insatnce of same generic type 
       Self {x, y} 
    }}

impl<T : Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y { // partial ord is needed for this !
            println!("the largest member is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
}}



impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username) 
    }
}

