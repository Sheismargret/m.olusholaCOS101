use std::fs::File;
use std::io::Write;

fn main() {
    // Provided student data
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328288", "Computer", 200),
        ("Adekunle Gold", "EEE11022020", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    // Display the student details
    println!("Student Details:");
    for student in &students {
        println!(
            "Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}\n",
            student.0, student.1, student.2, student.3
        );
    }

    // Create or overwrite the file
    let mut file = File::create("pau_smis.txt").expect("Unable to create file");

    // Write the student data into the file in the specified format
    for student in &students {
        writeln!(file, "Name: {}", student.0).expect("Unable to write to file");
        writeln!(file, "Matric Number: {}", student.1).expect("Unable to write to file");
        writeln!(file, "Department: {}", student.2).expect("Unable to write to file");
        writeln!(file, "Level: {}\n", student.3).expect("Unable to write to file");
    }

    println!("Student Information successfully saved to pau_smis.txt");
}
