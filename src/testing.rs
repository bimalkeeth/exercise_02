#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {

}

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn can_hold(&self,other:&Rectangle)->bool{
        return self.width >other.width && self.height > other.height
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn it_works(){
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller(){
        let larger =Rectangle{
            height:7,
            width:8
        };

        let smaller =Rectangle{
            width:5,
            height:1
        };

        assert!(larger.can_hold(&smaller));
    }
}