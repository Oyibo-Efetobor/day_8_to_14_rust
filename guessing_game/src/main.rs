//GUESSING GAME

use std::io;
// rand = '0.9.0' // add this line to your Cargo.toml file
use rand::Rng; // random number generator

fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = rand::rng().random_range(1..=100); // generate a random number between 1 and 100

    println!("The secret number is: {secret_number}");

    println!("Please enter your guess:");

    let mut guess = String::new(); // mutable variable
    io::stdin() // standard input
        .read_line(&mut guess) // read line from standard input
        .expect("Failed to read line"); // expect will panic if read_line fails
    println!("You guessed: {guess}");
}
