// Traits as Parameters 
// How to use traits to define functions that accepts many different types 

pub trait Summary {
    fn summarize(&self) -> String;
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
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name.
// This parameter accepts any type that implements the specified trait
// In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize 
// We can call notify and pass in any instance of NewsArticle or Tweet 
// N/B - Code that calls the function with any other type, such as a String or an i32, 
// won’t compile because those types don’t implement Summary

























