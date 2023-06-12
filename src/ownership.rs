fn main() {
    let x = String::from("hello");

    let _y = &x;

    print!("{}",x);

    let mut contents =String::from("hello ");

    change(&mut contents);

    let _reference_to_nothing =dangle();

}

fn change(some_string:&mut String){
    some_string.push_str(", tail")
}

fn dangle() ->  String{
    let s =String::from("hello");

    s
}