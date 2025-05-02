// Inputs a library used for user input + output
use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Self explanatory
    println!("Guess the number!");
    println!("Please input your guess.");

    // Creates a new, mutable variable 'guess', bound to a function
    // that returns a new instance of a String
    let mut guess = String::new();

    io::stdin()
        // Takes whatever the user types in
        // as input, and append that into a string
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
