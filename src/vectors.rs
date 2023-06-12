#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    let mut v:Vec<i32>=Vec::new();

    v.push(1);
    v.push(2);


    let nx = vec![1,2,3,4,5];
    let third:&i32 = &nx[2];
    println!("the third element is {third}");

    let third:Option<&i32> =nx.get(2);
    match third {
        Some(third)=> println!("the third element is {}",third),
        None=> println!("no third element found.")
    }


    let mut v2 =vec![1,2,3,4,5];
    let first = &v2[0];



    println!("the first element is {} ",first);

    for val in &mut v2{
        *val +=1
    }

    println!("{:?}",v2)

}