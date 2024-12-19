use std::io;

fn main() {
    loop {
        println!("Select a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => calculate_trapezium(),
            "2" => calculate_rhombus(),
            "3" => calculate_parallelogram(),
            "4" => calculate_cube(),
            "5" => calculate_cylinder(),
            "6" => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn calculate_trapezium() {
    let (base1, base2, height) = read_three_inputs("base1", "base2", "height");
    let area = height * (base1 + base2) / 2.0;
    println!("Area of Trapezium: {}", area);
}

fn calculate_rhombus() {
    let (diagonal1, diagonal2) = read_two_inputs("diagonal1", "diagonal2");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus: {}", area);
}

fn calculate_parallelogram() {
    let (base, height) = read_two_inputs("base", "height");
    let area = base * height;
    println!("Area of Parallelogram: {}", area);
}

fn calculate_cube() {
    let side = read_one_input("side");
    let area = 6.0 * side * side;
    println!("Area of Cube: {}", area);
}

fn calculate_cylinder() {
    let (radius, height) = read_two_inputs("radius", "height");
    let volume = std::f64::consts::PI * radius * radius * height;
    println!("Volume of Cylinder: {}", volume);
}

fn read_one_input(label: &str) -> f64 {
    println!("Enter {}:", label);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

fn read_two_inputs(label1: &str, label2: &str) -> (f64, f64) {
    (read_one_input(label1), read_one_input(label2))
}

fn read_three_inputs(label1: &str, label2: &str, label3: &str) -> (f64, f64, f64) {
    (read_one_input(label1), read_one_input(label2), read_one_input(label3))
}
 