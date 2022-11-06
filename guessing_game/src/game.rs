use std::cmp::Ordering;
use crate::input::get_input;
use crate::mistakes::{MistakeTracker, GuessMistake};
use crate::levels::{LEVELS, Level};

pub enum GameResult {
    Win(u32),
    Loss,
}

enum GameAction {
    Guess(u32),
    Quit,
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
        let action = get_game_input();
    
        match action {
            GameAction::Guess(guess) => {
                tries += 1;

                println!("Your guess: {guess}");
                
                let comparison_to_secret = guess.cmp(&secret_number);
                if comparison_to_secret == Ordering::Equal {
                    break;
                } else {
                    mistake_tracker.record(GuessMistake::new(guess, comparison_to_secret));
                }
                println!("You've used {}/{} tries.", tries, max_tries);
            },
            GameAction::Quit => {
                println!("Okay!");
                return GameResult::Loss;
            },
        }
    }

    return if tries < *max_tries { 
        GameResult::Win(tries)
    } else {
        GameResult::Loss
    };
}

fn get_game_input() -> GameAction {
    let mut action: Option<GameAction> = None;

    while action.is_none() {
        println!("Please input a guess or 'help' to see available actions");
        let untrimmed_input = get_input();
        let input = untrimmed_input.trim();
        if let Ok(guess) = input.parse::<u32>() {
            action = Some(GameAction::Guess(guess));
        } else {
            match input {
                "quit" => {
                    action = Some(GameAction::Quit);  
                },
                _ => {
                    println!("Sorry, I don't know what {} means", input);
                },
            }
        }
    }

    action.unwrap()
}
