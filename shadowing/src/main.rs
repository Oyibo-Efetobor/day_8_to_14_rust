//shadowing

fn main() {
    let x = 5;
    let x = x + 1; // shadowing
    {
        let x = x * 2; // shadowing
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);
}
