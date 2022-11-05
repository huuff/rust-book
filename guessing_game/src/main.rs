mod words;
mod mistakes;
mod levels;
mod stats;

use std::cmp::Ordering;
use std::io::{self, Write};
use words::WORDS;
use mistakes::MistakeTracker;
use levels::{LEVELS, Level};
use stats::Stats;

enum Action {
    Play,
    Stats,
    Quit,
}

fn main() {
    let mut stats: Stats = Stats::new();
    let mut current_level: usize = 0;

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
                play(&mut stats, &mut current_level);
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

// TODO: Print current number of tries
fn play(stats: &mut Stats, level: &mut usize) {
    println!("LEVEL {}", *level + 1);
    println!("---------");
    let Level { max_number, max_tries } = &LEVELS[*level];
    println!("Find the secret number between 1 and {}", max_number);

    // TODO: try a smaller type for this
    let secret_number = LEVELS[*level].create_secret_number();
    let mut tries: u32 = 0;
    let mut mistake_tracker = MistakeTracker::new();

    loop {
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
                println!(
                    "You won in {} {}!",
                    tries,
                    WORDS["try"].get_correct_form(tries)
                );
                break;
            }
        }
    }
    *level += 1;
    
    stats.record(tries);
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
