//imports 
use std::io;

mod models;
mod swap;

use models::User;
use swap::perform_swap;

fn main() {
    println!("Welcome ToborSwap simulator!");

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
        }

    }
}