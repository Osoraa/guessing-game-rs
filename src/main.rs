use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("***GUESS THE NUMBER***\n");

    // Generate secret num
    let secret_num = rand::thread_rng().gen_range(1..=100);

    // println!("The secret no. is: {secret_num}");

    loop {
        println!("Please input your guess.");

        // Collect user guess
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing in Rust
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };


        // Compare guess with secret
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small\n"),
            Ordering::Greater => println!("Too big\n"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
        // println!("You guessed: {guess}");
    }
}
