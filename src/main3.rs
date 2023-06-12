use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("please enter array index...");

    let mut index =String::new();

    match io::stdin().read_line(&mut index){
        Ok(n) => println!("read {} bytes",n),
        Err(_) => println!("error occurred")
    }

    let index:usize = index.parse().expect("error parsing error");

    let element = a[index];


    println!("the value of element of the index {}",element);


    let y = {
        let x =3;
        x +1
    };

    println!("value is {}",y)
}