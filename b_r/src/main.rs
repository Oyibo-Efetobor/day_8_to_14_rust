//Borrowing & Referencing
// Safety and Performance 


// fn main() {
//     // let _x:i32 = 5;
//     // let _r:&i32 = &_x; //r is a reference to x

//     //immutable reference
//     // *_r += 1; //this will throw an error because _r is a reference to _x
//     // println!("x: {}, r: {}", _x, _r); //x: 5, r: 5

//     //mutable reference
//     let mut _x: i32 = 5;
//     let _r: &mut i32 = &mut _x;
    
//     *_r += 1;
//     *_r -= 3;
    
//     // Use _r instead of trying to use both _x and _r
//     println!("x through r: {}", _r); // Will print 6
    
// }

fn main(){
    let mut account = BankAccount {
        owner: String::from("Alice"),
        balance: 150.55,
    };
    //Immutable Borrow to check balance
    account.check_balance();

    //Mutable Borrow to withdraw money
    account.withdraw(45.5);
    //Immutable Borrow to check balance again
    account.check_balance();

}
//structs
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount { 
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from {}'s account", amount, self.owner);
        self.balance -= amount;
    }
    
    

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}
