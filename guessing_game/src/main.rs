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
            let guess = get_guess();
            tries += 1;

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
        wins += 1;
        println!("You've won {wins} times.");
        println!();
    }
}

fn get_guess() -> u32 {
    let mut guess: Option<u32> = None;

    while guess == None {
        println!("Please input your guess.");
        print!("> ");
        let _ = io::stdout().flush();

        // TODO: Put it outside, reuse reference
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line")
            ;

        match input.trim().parse() {
            Ok(num) => {
                guess = Some(num);
            },
            Err(_) => {
                println!("Not a valid number, try again.");
                continue;
            }
        };
    };

    guess.unwrap()
}
