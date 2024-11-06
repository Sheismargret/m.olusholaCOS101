fn main() {
    let mut count = 0; // Initialize count to zero

    // Loop through numbers from 1 to 20
    for num in 1..21 {
        if num > 10 {
            println!("Counting: {}", num); // Print numbers greater than 10
            count += 1; // Increment count
        }
    }

    println!("The count of values greater than 10 (between 1 and 20) is: {}", count);
}
