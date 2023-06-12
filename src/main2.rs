use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("guessing the number!");

    let secret_numer = rand::thread_rng().
        gen_range(1..=100);

    println!("the secret number is :{}", secret_numer);

    loop {
        println!("please input your guess");

        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(n) => {
                print!("read {} bytes", n);
            }
            Err(err) => {
                println!("error in reading input {}", err);
                continue
            }
        };

        let guess:u32=match guess.trim().
            parse(){
            Ok(num) => num,
            Err(_)=>{
                continue
            }
        };

        match guess.cmp(&secret_numer) {
            Ordering::Less=> println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");


                
                break;
            }
        }
    }

}