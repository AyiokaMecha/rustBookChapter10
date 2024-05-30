use std::fmt::{Display, format};
use std::iter::Sum;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        "Read more...".to_string()
    }

    fn summarize_author(&self) -> String {
        format!("{}",self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet by {}... {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}",self.username)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
}

//a function that takes an item that implements summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//trait bounds
fn some_function<T: Display + Summary>(item: T) {
    todo!()
}
// fn some_other_function<T, U>(item: T, u: U) -> impl Summary
//     where   T: Display + Summary,
//             U: Display + PartialOrd + Summary
//     {
//     todo!( )
// }

//blanket implementation
// impl <T: Display> ToString for T {
//
// }