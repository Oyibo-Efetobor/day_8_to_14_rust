// how to add timer in rust
// use std::time::{Duration, Instant};

use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let mut account: CryptoAccount = CryptoAccount {
        owner: String::from("Tobor.Dev"),
        balance: 0.55,
    };
    println!(" ");
    println!("GM! Tobor.dev Welcome to the Crypto Bank!");
    loop {
    println!(" ");
    println!("Would you like to Deposit (D), Withdraw (W) , check balance (B) or Exit (E)?");
    println!("Please enter D, W, B or E");
    println!(" ");

    

    
    let mut user_input = String::new();  //mutable input

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input = user_input.trim().to_lowercase();
    
    if user_input == "d" {
        println!("How much USDC would you like to deposit?");
        let mut deposit_amount = String::new();
        io::stdin()
            .read_line(&mut deposit_amount)
            .expect("Failed to read line");
        
        let deposit_amount: f64 = deposit_amount.trim().parse().expect("Please enter a number!");
        account.deposit(deposit_amount);
    } else if user_input == "w" {
        println!("How much USDC would you like to withdraw?");
        let mut withdraw_amount = String::new();
        io::stdin()
            .read_line(&mut withdraw_amount)
            .expect("Failed to read line");
        
        let withdraw_amount: f64 = withdraw_amount.trim().parse().expect("Please enter a number!");
        account.withdraw(withdraw_amount);
    } else if user_input == "b" {
        account.check_balance();
    } else {
        println!("EXITING");
    }
    if user_input == "e" {
        break;
    }
}
    println!(" ");
    println!("Your final balance is {} USDC", account.balance);
    println!("Have a great day!");
}

struct CryptoAccount {
    owner: String,
    balance: f64,
}

impl CryptoAccount {
    fn deposit(&mut self, amount:f64) {
        println!("Depositing {} USDC to {}'s account, please wait ⌛", amount, self.owner);
        thread::sleep(Duration::from_secs(2));
        println!("Deposit successful! ✅");
        
        println!(" ");
        self.balance += amount;
        thread::sleep(Duration::from_secs(2));

    }

    fn withdraw(&mut self, amount:f64) {
        if amount > self.balance {
            println!("Insufficient funds! Cannot withdraw {} from {}'s account", amount, self.owner);
        } else {
            println!("Withdrawing {} USDC from {}'s account, please wait ⌛", amount, self.owner);
            thread::sleep(Duration::from_secs(2));
            println!("Withdrawal successful! ✅");
            println!(" ");
            self.balance -= amount;
            thread::sleep(Duration::from_secs(2));
        }

    }

    fn check_balance(&self) {
        println!(" ");
        thread::sleep(Duration::from_secs(2));
        println!("Checking balance for {}'s account, please wait ⌛", self.owner);
        thread::sleep(Duration::from_secs(2));
        println!(" ");
        println!(" {} your balance is {} USDC", self.owner, self.balance);
    }
}
