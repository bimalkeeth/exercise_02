#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

struct Point<T>{
    x:T,
    y:T
}

impl <T>Point<T>{
    fn x(&self)-> &T{
        return &self.x
    }
}

struct PointN<X,Y>{
    x:X,
    y:Y
}

impl <X,Y> PointN<X,Y>{

    fn mix_up<X1,Y1>(self,other:PointN<X1,Y1>)->PointN<X,Y1>{
        return PointN{
            x:self.x,
            y:other.y,
        }
    }
}

fn main() {
    let number_list=vec![34,50,25,100,65];
    let result =largest(&number_list);

    println!("the largest number {}",result);


    let p =Point{x:6,y:6};

    println!("p.x : {}",p.x())
}

fn largest<T>(list: &[T]) -> &T where T:PartialOrd{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}


