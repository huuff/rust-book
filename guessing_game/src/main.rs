mod words;
mod mistakes;
mod levels;
mod stats;
mod game;
mod input;
mod stuff;
mod bounds;

use levels::LEVELS;
use stats::Stats;
use game::{GameResult, play};
use input::get_input;
use stuff::Inventory;

enum Action {
    Play,
    Stats,
    Quit,
    Inventory,
}

fn main() {
    let mut stats: Stats = Stats::new();
    let mut inventory: Inventory = Inventory::new();

    println!("Welcome to guess the number!");
    println!("================");

    loop {
        println!("What do you want to do?");
        println!("* play");
        println!("* stats");
        println!("* quit");
        println!("* inventory");

        let untrimmed_input = get_input();
        let input = untrimmed_input.trim();
        let action: Action = match input {
            "play" =>  Action::Play,
            "stats" => Action::Stats,
            "quit" => Action::Quit,
            "inventory" => Action::Inventory,
            _ => {
                println!("Sorry, I don't know what {input} means.");
                continue;
            }
        };

        match action {
            Action::Play => {
                let result = play(stats.current_level(), &mut inventory);
                stats.record(result);
                if stats.current_level() >= LEVELS.len() {
                    println!("You've won the game! Congratulations!");
                    break;
                }
            }
            Action::Stats => {
                stats.print();
            },
            Action::Quit => {
                println!("Bye!");
                break;
            },
            Action::Inventory => {
                inventory.print(); 
            },
        };
        println!();
    }

}

