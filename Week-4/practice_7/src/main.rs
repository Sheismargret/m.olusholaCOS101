use std::io;

fn main() {
    println!("Enter a number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut num: i32 = input1.trim().parse().expect("Failed to parse input");

    while num > 10 {
        println!("Inside loop, number value is: {}", num);
        num -= 1;
    }
    println!("Outside loop, number value is: {}", num);

    // Counting values greater than 10 between 1 and 20
    let mut count = 0;
    for num in 1..21 {
        if num > 10 {
            count += 1;
        }
    }
    println!("The count of values greater than 10 (between 1 and 20) is: {}", count);
}


