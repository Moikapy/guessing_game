use std::io;
use rand::Rng;
fn main() {
    let x = 5; // immutable; Points Rewarded: 5

    // Ask the user to guess a number
    println!("Guess the number!");
    let secert_number = 
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    println!("You Get {} points!", x);
}
