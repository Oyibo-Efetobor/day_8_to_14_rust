//User and Token stuff

pub struct User {
    pub tob_balance: f64,
    pub sol_balance: f64,
}

impl User {
    pub fn new(tob: f64, sol: f64) -> Self {
        User {
            tob_balance: tob,
            sol_balance: sol,
        }
    }

    pub fn print_balances(&self) {
        println!("📊 Your Balances:");
        println!("TOB: {:.2}", self.tob_balance);
        println!("SOL: {:.2}", self.sol_balance);
    }
}
