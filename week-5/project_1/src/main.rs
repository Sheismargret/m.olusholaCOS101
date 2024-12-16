use std::io;

fn main() {
    // Menu prices
    let menu_prices = [
        ("P", "Poundo Yam/Edinkaiko Soup", 3200),
        ("F", "Fried Rice & Chicken", 3000),
        ("A", "Amala & Ewedu Soup", 2500),
        ("E", "Eba & Egusi Soup", 2000),
        ("W", "White Rice & Stew", 2500),
    ];

    println!("Welcome to the Food Ordering System!");
    println!("Here is the menu:\n");

    // Display the menu
    for item in &menu_prices {
        println!("{} = {} - N{}", item.0, item.1, item.2);
    }

    let mut total_cost = 0;

    // Take multiple orders
    loop {
        println!("\nEnter the food code (P, F, A, E, W) or type 'done' to finish ordering:");
        let mut code = String::new();
        io::stdin().read_line(&mut code).expect("Failed to read input");
        let code = code.trim().to_uppercase();

        if code == "DONE" {
            break;
        }

        let mut quantity = String::new();
        println!("Enter the quantity for {}:", code);
        io::stdin()
            .read_line(&mut quantity)
            .expect("Failed to read quantity");
        let quantity: i32 = quantity.trim().parse().expect("Please enter a valid number");

        // Find the price for the given code
        let mut item_found = false;
        for item in &menu_prices {
            if code == item.0 {
                total_cost += item.2 * quantity;
                println!(
                    "Added {} x {} (N{}) to your order.",
                    quantity, item.1, item.2
                );
                item_found = true;
                break;
            }
        }

        if !item_found {
            println!("Invalid food code! Please try again.");
        }
    }

    // Apply discount if applicable
    if total_cost > 10_000 {
        let discount = total_cost as f32 * 0.05;
        total_cost -= discount as i32;
        println!("\nYou received a 5% discount of N{:.2}!", discount);
    }

    // Display the total order cost
    println!("\nYour total order cost is: N{}", total_cost);
    println!("Thank you for ordering with us!");
}
