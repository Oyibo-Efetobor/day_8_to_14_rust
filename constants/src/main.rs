//constants

// constants are values that are bound to a name and are not allowed to change
// diff b/w constants and variables
// you are not allowed to use keyword 'mut' with constants
// constants are always immutable



fn main() {
    let  x  = 5;
    // const mut y = 10; //this will throw an error because y is a constant
    // also declaring const should be in capital letter
    const Y: i32 = 10; //you are also to add the type of the constant (e.g i32)
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", Y);
    println!("The value of PI is: {}", PI);
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
}

// you can declare a const outside the function
// including a global scope

const PI : f64 = 3.14159; //this is a constant
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //this is a constant