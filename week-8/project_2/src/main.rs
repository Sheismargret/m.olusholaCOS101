fn main() {
    // List of candidates with their experience
    let candidates = vec![
        ("Alice", 5),
        ("Bob", 10),
        ("Charlie", 8),
    ];

    // Finding the candidate with the highest experience
    let mut highest_experience = 0;
    let mut best_candidate = "";
    for candidate in &candidates {
        if candidate.1 > highest_experience {
            highest_experience = candidate.1;
            best_candidate = candidate.0;
        }
    }

    // Output
    println!(
        "The candidate with the highest experience is {} with {} years.",
        best_candidate, highest_experience
    );
}
