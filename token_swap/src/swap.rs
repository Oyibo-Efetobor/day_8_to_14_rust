// core logic

use crate::models::User;

const TOB_TO_SOL_RATE: f64 = 2.0; // 1 TOB = 2 SOL
const SOL_TO_TOB_RATE: f64 = 0.5; // 1 SOL = 0.5 TOB

pub fn perform_swap(user: &mut User, from_token: &str, amount: f64) {
    if amount <= 0.0 {
        println!("⚠️ Please enter a positive amount.");
        return;
    }

    match from_token {
        "TOB" => {
            if user.tob_balance >= amount {
                let received = amount * TOB_TO_SOL_RATE;
                user.tob_balance -= amount;
                user.sol_balance += received;
                println!(
                    "✅ Swapped {:.2} TOB for {:.2} SOL",
                    amount, received
                );
            } else {
                println!("❌ Not enough TOB to swap.");
            }
        }
        "SOL" => {
            if user.sol_balance >= amount {
                let received = amount * SOL_TO_TOB_RATE;
                user.sol_balance -= amount;
                user.sol_balance += received;
                println!(
                    "✅ Swapped {:.2} SOL for {:.2} TOB",
                    amount, received
                );
            } else {
                println!("❌ Not enough SOL to swap.");
            }
        }
        _ => {
            println!("❌ Invalid token selected.");
        }
    }
}
