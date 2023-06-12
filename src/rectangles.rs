#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width += 10;

        return self.width * self.height;
    }

    fn width(&self) -> u32 {
        return self.width;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width
            && self.height > other.height;
    }

    fn new(width: u32, height: u32) -> Self {
        return Self {
            height,
            width,
        };
    }
}


fn main() {
    let mut rect = Rectangle::
    new(50, 30);
    let rect1 = Rectangle::
    new(50, 30);
    let rect2 = Rectangle::
    new(10, 40);
    let rect3 = Rectangle::
    new(60, 45);


    println!("the are of rectangle is {} square pixels", rect.area());
    println!("new width {}", rect.width());
    println!("rectangle is {:#?}", rect);
}


