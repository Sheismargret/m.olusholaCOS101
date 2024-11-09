use std::io;

fn main() {
    // Taking input for a, b, and c
    println!("Enter the value of a:");
    let a = input_number();
    
    println!("Enter the value of b:");
    let b = input_number();
    
    println!("Enter the value of c:");
    let c = input_number();
    
    // Calculating the discriminant
    let discriminant = b * b - 4.0 * a * c;
    
    // Determining the nature and number of roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("There are two distinct roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("There is one real root: {}", root);
    } else {
        println!("There are no real roots.");
    }
}

// Helper function to handle user input
fn input_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}
