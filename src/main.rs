use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate secret num
    let secret_num = rand::thread_rng().gen_range(1..=100);

    // println!("The secret no. is: {secret_num}");

    println!("Please input your guess.");

    // Collect user guess
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    // Compare guess with secret
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Equal"),
    }

    println!("You guessed: {guess}");
}
