#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

enum IPAddress {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> String {
        let mut str_val = String::new();

        match self {
            Message::Write(val) => {
                str_val = val.to_string();
            }
            _ => {
                str_val = String::from("");
            }
        }

        return str_val;
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    NewJersey
}

fn main() {
    let four = IPAddress::V4(String::from("127.0.0.1"));
    let six = IPAddress::V6(String::from("::1"));

    let m = Message::Write(String::from("Hi"));
    let s = m.call();

    println!("{}", s);

    let xd=value_in_cents(Coin::Quarter(USState::Alabama));
    println!("selected coin {}",xd);

    let val=plus_one(Some(10));
    println!("value from plus_one {:?}",val.unwrap())

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State of the quarter {:?}",state);

            25
        }
    }
}

fn plus_one(x:Option<i32>)->Option<i32>{
    if let Some(val)=x{
       println!("{}",val)
    }

    match x {
        Some(val)=> Some(val +1),
        None=> None
    }
}