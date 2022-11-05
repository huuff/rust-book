use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

enum Action {
    Play,
    Stats,
    Quit,
}

// TODO: Correctly use singular/plural
fn main() {
    let mut wins: u32 = 0;
    println!("Welcome to guess the number!");
    println!("================");

    loop {
        println!("What do you want to do?");
        println!("* play");
        println!("* stats");
        println!("* quit");

        let input = get_input();
        // TODO: Try to remove the \n's with `trim()`
        let action: Action = match input.as_str() {
            "play\n" =>  Action::Play,
            "stats\n" => Action::Stats,
            "quit\n" => Action::Quit,
            _ => {
                println!("Sorry, I don't know what {input} means.");
                continue;
            }
        };

        match action {
            Action::Play => {
                play(&mut wins);
                println!("You've won {wins} times");
                println!();
            }
            Action::Quit => {
                println!("Bye!");
                break;
            },
            _ => {
                panic!();
            },
        };
    }

}

fn play(wins: &mut u32) {
    // TODO: try a smaller type for this
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
    *wins += 1;
}

fn get_input() -> String {
    // TODO: Put prompt here
    // TODO: Put it outside, reuse reference
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line")
        ;
    
    return input;
}

// TODO: Try a smaller type?
fn get_guess() -> u32 {
    let mut guess: Option<u32> = None;

    while guess == None {
        println!("Please input your guess.");
        print!("> ");
        let _ = io::stdout().flush();


        match get_input().trim().parse() {
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
