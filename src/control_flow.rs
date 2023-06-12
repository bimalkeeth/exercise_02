fn main() {
    let condition = true;

    let number = if condition {
        5
    }else {
        6
    };

    println!("the value of the number is {}",number);

    let mut counter = 0;
    let result = loop{
        counter+=1;


        if counter == 100{
            break counter * 2;
        }
    };

    println!("the result is {}",result);


    let mut count = 0;

    'counting_loop: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_loop;
            }

            remaining -=1;
        }

        count+=1;
    }

    println!("end count = {count}")
}