use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        // more secure, default is immutable
        let mut guess = String::new();

        // stream caller style, code more concise
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // `trait` exciting type, not need to change the source code, can extend the ability
        // shadowing variable, can change the type of variable, not have multiple defined error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // give a ability keep away form the much if else
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
