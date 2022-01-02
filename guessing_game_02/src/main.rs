#[macro_use]
extern crate fstrings;
extern crate rand;

use std::io; 
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println_f!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println_f!("The secret_number is: {secret_number}");

    loop {
        println_f!("Plz input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        // type conversion 
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println_f!("[E] guess is not a number!");
                continue;
            }
        };

        println_f!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less    => println_f!("Too small!"),
            Ordering::Greater => println_f!("Too big!"),
            Ordering::Equal   => { 
                println_f!("You win!");
                break;
            }
        }
    }
}
