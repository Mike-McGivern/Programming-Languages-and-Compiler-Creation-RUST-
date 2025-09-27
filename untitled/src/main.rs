mod ownership;

use std::io;
fn main() {
    let mut x:i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x:i32 = 5;
    let x:i32 = x + 1;
    {
        let x:i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("Tje value of x is: {x}");

    let tup: (i32, f64, u8) = (580, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {y}");
    let five_hundred = tup.0;
    println!("The value of x is {five_hundred}");


    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for ele in a {
        println!("{ele}");
    }

    for num in (1..4).rev() {
        println!("{num}!");
    }

    // get help for input

    println!("Enter 1 for true or anything else for false");

    let mut index = String::new();
    let mut condition: usize = io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let bool_condition: bool = if condition == 1 {true} else {false};
    let num:i32 = if bool_condition {5} else {6};
    println!("your number is {num}");
    let num:i32 = plus_one(num);
    println!("your number plus one is {num}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
