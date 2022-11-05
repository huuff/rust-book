mod words;
mod mistakes;
mod levels;

use std::cmp::Ordering;
use std::io::{self, Write};
use words::WORDS;
use mistakes::MistakeTracker;
use levels::LEVELS;

enum Action {
    Play,
    Stats,
    Quit,
}

struct Stats {
    wins: u32,
    min_tries: Option<u32>,
}

fn main() {
    let mut stats: Stats = Stats {
        wins: 0,
        min_tries: None,
    };
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
                play(&mut stats, current_level);
            }
            Action::Stats => {
                print_stats(&stats);
            },
            Action::Quit => {
                println!("Bye!");
                break;
            },
        };
        println!();
        current_level += 1;
    }

}

// TODO: Print current number of tries
fn play(stats: &mut Stats, level: usize) {
    // TODO: try a smaller type for this
    let secret_number = LEVELS[level].create_secret_number();
    let mut tries: u32 = 0;
    let mut mistake_tracker = MistakeTracker::new();

    loop {
        let guess = get_guess();
        tries += 1;

        println!("You guess: {guess}");

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
    stats.wins += 1;
    match stats.min_tries {
        Some(min) => {
            if tries < min {
                stats.min_tries = Some(tries);
            }
        },
        None => {
            stats.min_tries = Some(tries);
        }
    };
}

fn print_stats(stats: &Stats) {
    println!(
        "You've won {} {}",
        stats.wins,
        WORDS["win"].get_correct_form(stats.wins)
    );
    if let Some(tries) = stats.min_tries {
        println!(
            "Your best game ended in {} {}",
            tries,
            WORDS["try"].get_correct_form(tries)
        );

    }

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
