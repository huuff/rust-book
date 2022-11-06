use std::cmp::Ordering;
use crate::input::get_input;
use crate::mistakes::{MistakeTracker, GuessMistake};
use crate::levels::{LEVELS, Level};

pub enum GameResult {
    Win(u32),
    Loss,
}

pub fn play(level: usize) -> GameResult {
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
        
        let comparison_to_secret = guess.cmp(&secret_number);
        if comparison_to_secret == Ordering::Equal {
            break;
        } else {
            mistake_tracker.record(GuessMistake::new(guess, comparison_to_secret));
        }
        println!("You've used {}/{} tries.", tries, max_tries);
    }

    return if tries < *max_tries { 
        GameResult::Win(tries)
    } else {
        GameResult::Loss
    };
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
