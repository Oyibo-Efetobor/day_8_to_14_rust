//variables & mutability 
fn main() {
    
    let _a: i32 = 5;
    println!("a: {}", _a);

    // _a = 10; //this will throw an error because _a is immutable

    let mut _b: i32 = 5;
    println!("b: {}", _b);
    _b = 10; //this will work because _b is mutable
    println!("b: {}", _b);
}