//imports 
use std::io;

mod models;
mod swap;

use models::User;
use swap::perform_swap;

fn main() {
    println!("Welcome ToborSwap simulator! 🪙");

    let mut user = User::new(100.0, 100.0); // 100 $TOB & 100 $SOL

    loop {
        println!("\n======================================");
        println!("1. View Balances");
        println!("2. Swap $TOB to $SOL");
        println!("3. Swap $SOL to $TOB");
        println!("4. Exit");
        println!("Choose an Option");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                user.print_balances();
            }
            "2" => {
                println!("Enter amount of TOB to swap:");
                let amount = read_amount();
                perform_swap(&mut user, "TOB", amount);
            }
            "3" => {
                println!("Enter amount of SOL to swap:");
                let amount = read_amount();
                perform_swap(&mut user, "SOL", amount);
            }
            "4" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Try again.");
            }
        }

    }
}

fn read_amount() -> f64 {
    let mut amount_input = String::new();
    io::stdin().read_line(&mut amount_input).unwrap();
    amount_input.trim().parse::<f64>().unwrap_or(0.0)    
}