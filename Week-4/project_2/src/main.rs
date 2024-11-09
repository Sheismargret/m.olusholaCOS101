use std::io;

fn main() {
    // Input experience and age
    println!("Is the employee experienced? (yes/no):");
    let experienced = input_string().to_lowercase() == "yes";

    println!("Enter the age of the employee:");
    let age = input_number();

    // Determine the incentive based on criteria
    let incentive = if experienced {
        if age >= 40 {
            1_560_000
        } else if age >= 30 && age < 40 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            0 // In case there's an age range not covered
        }
    } else {
        100_000
    };

    // Display the incentive
    println!("The incentive for the employee is: N{}", incentive);
}

// Helper function for string input
fn input_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// Helper function for number input
fn input_number() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}
