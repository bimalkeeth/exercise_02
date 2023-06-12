#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};

fn main() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(handle) => handle,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("error")
            },
            other_error => {
                panic!("error")
            }
        }
    };
}


fn read_user_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut user_name_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut user_name = String::new();

    match user_name_file.read_to_string(&mut user_name) {
        Ok(_) => Ok(user_name),
        Err(e) => Err(e)
    }
}

fn read_user_from_file_next() -> Result<String, io::Error> {
    let mut user_name = String::new();

    File::open("hello.txt")?.
        read_to_string(&mut user_name)?;

    return Ok(user_name);
}

fn read_user_from_file_next_two() -> Result<String, io::Error> {
 fs::read_to_string("hello.txt")
}