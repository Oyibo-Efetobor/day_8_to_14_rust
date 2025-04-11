//Ownership
// fn main() {
//     let s1 = String::from("RUST");
//     //the 'RUST' value is owned by the s1 owner

//     let len = calculate_length(&s1); //not passing the owner but a ref to the owner
//     println!("Length of '{}' is {}.", s1, len);

//     main_n();
// }


// s in calculate_length is not going to
//be an owner but a reference to s1
// and reference is proceeded by an ampersand sign & 



//There can only be one owner of a value at a time

// fn main_n(){
//     let s1 = String::from("RUST");
//     let s2 = s1; //s1 is moved to s2 

//     // println!("s1: {}", s1); //this will throw an error
//     println!("s2: {}", s2); //this will work

// }

//When the owner goes out of scope, the value will be dropped
fn main(){
    let s1 = String::from("RUST");
    let len = calculate_length(&s1); //not passing the owner but a ref to the owner
    println!("Length of '{}' is {}.", s1, len);
    //s1 is the owner of the value
    //when s1 goes out of scope, the value will be dropped
    //and the memory will be freed
}

fn calculate_length(s: &String) -> usize{
    s.len()
}