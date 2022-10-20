use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let x = 5; // immutable; Points Rewarded: 5

    // Ask the user to guess a number
    println!("Guess the number!");
    let secert_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secert_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
