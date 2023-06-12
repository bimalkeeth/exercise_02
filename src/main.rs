use std::io;

fn main() {
    println!("Guess the number");

    print!("please input your guess");

    let mut guess = String::new();

    match io::stdin().read_line(&mut guess){
        Ok(n)=>{
            println!("{n} bytes read");
        },
        Err(err)=>{
            println!("error: {err}")
        }
    }

    println!("yoy have guess {}", guess)
}
