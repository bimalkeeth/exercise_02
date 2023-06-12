#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]


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
        return format!("{},by {} ({})", self.headline, self.author, self.location);
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String {
        return format!("{}:{}",self.username,self.content)
    }
}

fn notify<T>(item:&T) where T:Summary {
    println!("{}",item.summarize());
}

fn return_summarizable() -> impl Summary{
    return Tweet{
        username:String::from("horse_ebook"),
        content:String::from("of course, as you probably already know , people"),
        reply:false,
        retweet:false
    }
}

fn main() {
    let tweet=Tweet{
        username:String::from("horse_ebook"),
        content:String::from("of course, as you probably already know , people"),
        reply:false,
        retweet:false
    };

    println!("1 new tweet: {}",&tweet.summarize());

    notify(&tweet);


    let summ = return_summarizable();

    println!("{}",summ.summarize())
}