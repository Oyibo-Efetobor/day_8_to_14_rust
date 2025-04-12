//GUESSING GAME

use std::io;
// rand = '0.9.0' // add this line to your Cargo.toml file
use rand::Rng; // random number generator

use std::cmp::Ordering; // for comparing numbers

fn main() {
    println!("Welcome to the guessing game!");
    loop{
        println!("Please input a guess between 1 and 10");
        // generate a random number between 1 and 10
        let secret_number = rand::rng().random_range(1..=10); 
        
        let mut guess = String::new(); // mutable variable
        io::stdin() // standard input
            .read_line(&mut guess) // read line from standard input
            .expect("Failed to read line"); // expect will panic if read_line fails
        println!("You guessed: {guess}");

        // parse the string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // if parsing is successful, assign the number to guess
            // if parsing fails, print an error message and continue to the next iteration
            // of the loop
            Err(_) => {
                println!("Please enter a number!");
                continue; // continue to the next iteration of the loop
            }
        }; 

        // comparing the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
