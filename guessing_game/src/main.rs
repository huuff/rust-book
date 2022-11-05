use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let mut wins: u32 = 0;
    loop {
        println!("Guess the number!");
        println!("================");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut tries: u32 = 0;

        loop {
            println!("Please input your guess.");
            print!("> ");
            let _ = io::stdout().flush();

            let guess = match get_guess() {
              Ok(num) => num,
              Err(_) => {
                println!("Not a valid number, try again.");
                continue;
              }
            };
            tries = tries + 1;

            println!("You guess: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You won in {tries} tries!");
                    break;
                }
            }
        }
        wins = wins + 1;
        println!("You've won {wins} times.");
        println!();
    }
}

fn get_guess() -> Result<u32, std::num::ParseIntError> {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line")
        ;

    return guess.trim().parse();
}
