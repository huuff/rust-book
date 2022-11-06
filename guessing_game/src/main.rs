mod words;
mod mistakes;
mod levels;
mod stats;

use std::cmp::Ordering;
use std::io::{self, Write};
use mistakes::MistakeTracker;
use levels::{LEVELS, Level};
use stats::Stats;

enum Action {
    Play,
    Stats,
    Quit,
}

pub enum GameResult {
    Win(u32),
    Loss,
}

fn main() {
    let mut stats: Stats = Stats::new();

    println!("Welcome to guess the number!");
    println!("================");

    loop {
        println!("What do you want to do?");
        println!("* play");
        println!("* stats");
        println!("* quit");

        let untrimmed_input = get_input();
        let input = untrimmed_input.trim();
        let action: Action = match input {
            "play" =>  Action::Play,
            "stats" => Action::Stats,
            "quit" => Action::Quit,
            _ => {
                println!("Sorry, I don't know what {input} means.");
                continue;
            }
        };

        match action {
            Action::Play => {
                let result = play(stats.current_level());
                stats.record(result);
            }
            Action::Stats => {
                stats.print();
            },
            Action::Quit => {
                println!("Bye!");
                break;
            },
        };
        println!();
    }

}

fn play(level: usize) -> GameResult {
    println!("LEVEL {}", level + 1);
    println!("---------");
    let Level { max_number, max_tries } = &LEVELS[level];
    println!("Find the secret number between 1 and {}", max_number);
    println!("You've got {max_tries} tries");

    // TODO: try a smaller type for this
    let secret_number = LEVELS[level].create_secret_number();
    let mut tries: u32 = 0;
    let mut mistake_tracker = MistakeTracker::new();

    while tries < *max_tries {
        let guess = get_guess();
        tries += 1;

        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                mistake_tracker.record(Ordering::Less);
            }
            Ordering::Greater => {
                println!("Too big!");
                mistake_tracker.record(Ordering::Greater);
            }
            Ordering::Equal => {
                break;
            }
        }
        println!("You've used {}/{} tries.", tries, max_tries);
    }

    return if tries < *max_tries { 
        GameResult::Win(tries)
    } else {
        GameResult::Loss
    };
}

fn get_input() -> String {
    // TODO: Put it outside, reuse reference
    print!("> ");
    let _ = io::stdout().flush();
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line")
        ;

    println!();
    
    return input;
}

// TODO: Try a smaller type?
fn get_guess() -> u32 {
    let mut guess: Option<u32> = None;

    while guess == None {
        println!("Please input your guess.");

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
