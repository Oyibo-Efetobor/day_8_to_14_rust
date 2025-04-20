//imports 
use std::io::{self};
use std::thread::sleep;
use std::time::Duration;

mod models;
mod swap;

use models::User;
use swap::perform_swap;

fn main() {
    println!("Welcome to ToborSwap simulator! 🪙");
    sleep(Duration::from_millis(600));

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
                println!("\nFetching balances...");
                sleep(Duration::from_millis(700));
                user.print_balances();
                sleep(Duration::from_millis(500));
            }
            "2" => {
                println!("Enter amount of TOB to swap:");
                let amount = read_amount();
                animated_swap_loading();
                sleep(Duration::from_millis(800));
                perform_swap(&mut user, "TOB", amount);
                sleep(Duration::from_millis(600));
            }
            "3" => {
                println!("Enter amount of SOL to swap:");
                let amount = read_amount();
                animated_swap_loading();
                sleep(Duration::from_millis(800));
                perform_swap(&mut user, "SOL", amount);
                sleep(Duration::from_millis(600));
            }
            "4" => {
                println!("👋 Goodbye!");
                sleep(Duration::from_millis(500));
                break;
            }
            _ => {
                println!("Invalid option. Try again.");
                sleep(Duration::from_millis(400));
            }
        }
    }
}


fn read_amount() -> f64 {
    let mut amount_input = String::new();
    io::stdin().read_line(&mut amount_input).unwrap();
    amount_input.trim().parse::<f64>().unwrap_or(0.0)    
}

fn animated_swap_loading() {
    let dots = ["Swapping.", "Swapping..", "Swapping..."];
    for dot in dots {
        print!("\r{}", dot);
        io::Write::flush(&mut io::stdout()).unwrap(); // force flush to terminal
        sleep(Duration::from_millis(400));
    }
    println!(); // move to new line
}
