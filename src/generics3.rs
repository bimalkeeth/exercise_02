#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]


use std::fmt::Display;



struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { y, x }
    }
}

impl<T> Pair<T> where T: PartialOrd + Display {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest number is x = {}", self.x)
        } else {
            println!("the largest number is y = {}", self.y)
        }
    }
}

fn longest(x: &str, y: &str) ->  String {
    return if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}

fn first_word(s:&str)->&str{
    let bytes=s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    return &s[..];
}

struct ImportantExcerpt<'a>{
    part:&'a str
}

impl <'a> ImportantExcerpt<'a>{
    fn level(&self)-> i32{
        return 3
    }
    fn announce_return_part(&self,announcement:&str)->&str{
        println!("attention please: {}",announcement);

        return self.part
    }
}



fn main() {
    let string1 = String::from("abcd");

    let result;
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
    }

    println!("the longest string is {}", result);

    println!("word to find {} :",first_word("ffffffff"))
}