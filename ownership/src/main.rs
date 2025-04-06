//Ownership
fn main() {
    let s1 = String::from("RUST");
    //the 'RUST' value is owned by the s1 owner

    let len = calculate_length(&s1); //not passing the owner but a ref to the owner
    println!("Length of '{}' is {}.", s1, len);
}


// s in calculate_length is not going to
//be an owner but a reference to s1
// and reference is proceeded by an ampersand sign & 
fn calculate_length(s: &String) -> usize{
    s.len()
}