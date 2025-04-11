//Borrowing & Referencing
// Safety and Performance 


fn main() {
    // let _x:i32 = 5;
    // let _r:&i32 = &_x; //r is a reference to x

    //immutable reference
    // *_r += 1; //this will throw an error because _r is a reference to _x
    // println!("x: {}, r: {}", _x, _r); //x: 5, r: 5

    //mutable reference
    let mut _x: i32 = 5;
    let _r: &mut i32 = &mut _x;
    
    *_r += 1;
    
    // Use _r instead of trying to use both _x and _r
    println!("x through r: {}", _r); // Will print 6
    
}
