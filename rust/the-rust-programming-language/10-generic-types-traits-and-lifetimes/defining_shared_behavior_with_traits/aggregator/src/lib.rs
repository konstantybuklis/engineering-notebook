use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

// Blanket implementation
impl<T: ToString> Summary for T {
    fn summarize_author(&self) -> String {
        String::from("some value")
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("author: {}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify(item: &(impl Summary + Debug))
pub fn notify<T: Summary + Debug>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// #[allow(dead_code)]
// #[allow(unused_variables)]
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     0
// }

#[allow(dead_code)]
#[allow(unused_variables)]
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

#[allow(dead_code)]
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}
