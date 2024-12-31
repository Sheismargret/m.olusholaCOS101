use std::fs::File;
use std::io::Write;

fn main() {
    // Define categories
    let categories = [
        ("Lager", vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]),
        ("Stout", vec!["Legend", "Turbo King", "Williams"]),
        ("Non-Alcoholic", vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"]),
    ];

    // Create or overwrite the file
    let mut file = File::create("nigerian_breweries.txt").expect("Unable to create file");

    // Write data to the file
    for (category, drinks) in categories {
        writeln!(file, "{}:", category).expect("Unable to write to file");
        for drink in drinks {
            writeln!(file, "  - {}", drink).expect("Unable to write to file");
        }
    }

    println!("Data successfully written to nigerian_breweries.txt");
}
