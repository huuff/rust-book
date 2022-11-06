use std::cmp::Ordering;
use crate::input::get_input;
use crate::mistakes::{MistakeTracker, GuessMistake};
use crate::levels::{LEVELS};
use crate::stuff::{Inventory, PowerUp};
use regex::Regex;

pub enum GameResult {
    Win(u32),
    Loss,
}

enum GameAction {
    Guess(u32),
    Quit,
    Help,
    Inventory,
    PowerUp(PowerUp),
}

pub fn play(level: usize, inventory: &mut Inventory) -> GameResult {
    println!("LEVEL {}", level + 1);
    println!("---------");
    let level = &LEVELS[level];
    println!("Find the secret number between 1 and {}", level.max_number);
    println!("You've got {} tries", level.max_tries);

    // TODO: try a smaller type for this
    let secret_number = level.create_secret_number();
    let mut tries: u32 = 0;
    let mut mistake_tracker = MistakeTracker::new();

    while tries < level.max_tries {
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
                println!("You've used {}/{} tries.", tries, level.max_tries);
                let optional_drop = level.get_maybe_drop();
                if let Some(drop) = optional_drop {
                    inventory.add(drop);
                }

            },
            GameAction::Quit => {
                println!("Okay!");
                return GameResult::Loss;
            },
            GameAction::Inventory => {
                inventory.print();
            },
            GameAction::Help => {
                println!("These are your options:");
                println!("* Input an integer to make a guess");
                println!("* Input 'inventory' to check your inventory");
                println!("* Input 'quit' to exit this level");
                println!("* Input 'help' to display this help");
            },
            GameAction::PowerUp(power_up) => {
                match power_up {
                    PowerUp::ExtraTry => {
                        if inventory.has(PowerUp::ExtraTry) {
                            println!("You used an ExtraTry!");
                            tries -= 1;
                            inventory.remove(PowerUp::ExtraTry);
                        } else {
                            println!("You don't have an ExtraTry!");
                        }
                    },
                }
            },
        }
        println!();
    }

    return if tries < level.max_tries { 
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
                "help" => {
                    action = Some(GameAction::Help);
                },
                "inventory" => {
                    action = Some(GameAction::Inventory);
                },
                _ => {
                    let power_up_regex = Regex::new(r"powerup (\w+)").unwrap();
                    if power_up_regex.is_match(input) {
                        let power_up = power_up_regex.captures(input)
                                                     .unwrap()
                                                     .get(1)
                                                     .unwrap()
                                                     .as_str(); 
                        match power_up {
                           "extratry" => {
                                action = Some(GameAction::PowerUp(PowerUp::ExtraTry));
                           }, 
                           _ => {
                                println!("No power up with name {} exists", power_up);
                           }
                        }
                    } else {
                        println!("Sorry, I don't know what {} means", input);
                    }
                },
            }
        }
    }

    action.unwrap()
}
