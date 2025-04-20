//If else expressions


// fn main() {
//     // let _age = 30;
//     //condition if age is is more than or equal to 18

//     // if age >= 18 { 
//     //     println!("You are an adult.");
//     // } else if age >= 13 {
//     //     println!("You are a teenager.");
//     // } else {
//     //     println!("You are a child.");
//     // }

//     //multiple coonditions
//     // let number = 6;
//     // if number % 4 == 0 {
//     //     println!("The number is divisible by 4.");
//     // } else if number % 3 == 0 {
//     //     println!("The number is divisible by 3.");
//     // } else if number % 2 == 0 {
//     //     println!("The number is divisible by 2.");
//     // } else {
//     //     println!("The number is not divisible by 4, 3, or 2.");
//     // }

//     //using if in a let statement
//     // let condition = false;
//     // let number = if condition {5} else {6};

//     // println!("The value of number is :{number}");

//     //Returning values from loops

//     // let mut counter =0;

//     // let result = loop {
//     //     counter += 1;

//     //     if counter == 10 {
//     //         break counter * 2;
//     //     }
//     // };
//     // println!("The result is {result}")
    
    

// }

// fn main() {
//     //loop labels to disambiguiate between multiple loops

//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }

//             if count == 2 {
//                 break 'counting_up; //exits the outer loop
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }


// fn main() {
//     let mut number = 5;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }
//     print!("BLSATOFF !!!")
// }

