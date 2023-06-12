#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let mut user1 = User{
        active:true,
        email:String::from("bimalkeeth@gmail.com"),
        username:String::from("bimal kaluarachchi"),
        sign_in_count:10,
    };


    user1.email=String::from("bimal.kaluarachchi@betmakers.com")

}