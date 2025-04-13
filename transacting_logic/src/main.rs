use std::io;

fn main() {
    let mut account: CryptoAccount = CryptoAccount {
        owner: String::from("Alice"),
        balance: 0.55,
    };

    println!("GM! Welcome to the Crypto Bank!");
    println!("Would you like to Deposit (D), Withdraw (W) or check balance (B)");

    let mut user_input = String::new();  //mutable input

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input = user_input.trim().to_lowercase();
    
    if user_input == "d" {
        println!("How much would you like to deposit?");
        let mut deposit_amount = String::new();
        io::stdin()
            .read_line(&mut deposit_amount)
            .expect("Failed to read line");
        
        let deposit_amount: f64 = deposit_amount.trim().parse().expect("Please enter a number!");
        account.deposit(deposit_amount);
    } else if user_input == "w" {
        println!("How much would you like to withdraw?");
        let mut withdraw_amount = String::new();
        io::stdin()
            .read_line(&mut withdraw_amount)
            .expect("Failed to read line");
        
        let withdraw_amount: f64 = withdraw_amount.trim().parse().expect("Please enter a number!");
        account.withdraw(withdraw_amount);
    } else if user_input == "b" {
        account.check_balance();
    } else {
        println!("Invalid input! Please enter D, W or B.");
    }
    
}

struct CryptoAccount {
    owner: String,
    balance: f64,
}

impl CryptoAccount {
    fn deposit(&mut self, amount:f64) {
        println!("Depositing {} to {}'s account", amount, self.owner);
        self.balance += amount;
    }

    fn withdraw(&mut self, amount:f64) {
        if amount > self.balance {
            println!("Insufficient funds! Cannot withdraw {} from {}'s account", amount, self.owner);
        } else {
            println!("Withdrawing {} from {}'s account", amount, self.owner);
            self.balance -= amount;
        }

    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {} USDC", self.owner, self.balance);
    }
}
