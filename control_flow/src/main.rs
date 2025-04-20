//If else expressions


fn main() {
    let _age = 30;
    //condition if age is is more than or equal to 18

    // if age >= 18 { 
    //     println!("You are an adult.");
    // } else if age >= 13 {
    //     println!("You are a teenager.");
    // } else {
    //     println!("You are a child.");
    // }

    //multiple coonditions
    let number = 6;
    if number % 4 == 0 {
        println!("The number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2.");
    } else {
        println!("The number is not divisible by 4, 3, or 2.");
    }
    
}
