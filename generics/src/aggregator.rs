pub trait Summary {
    fn summarize_author(&self) -> String; 

    fn summarize(&self) -> String{
        format!("(Read More from {})", self.summarize_author())
    }

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
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username) 
    }
}

