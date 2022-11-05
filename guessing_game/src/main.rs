use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref WORDS: HashMap<&'static str, NumberedWord> = {
        let mut map = HashMap::new();
        map.insert(
            "try",
            NumberedWord {
                singular: "try".to_string(),
                plural: "tries".to_string(),
            }
        );
        map.insert(
            "win",
            NumberedWord {
                singular: "win".to_string(),
                plural: "wins".to_string(),
            }
        );
        map
    }; 
}

enum Action {
    Play,
    Stats,
    Quit,
}

struct NumberedWord {
    singular: String,
    plural: String,
}

impl NumberedWord {
    fn get_correct_form(&self, number: u32) -> &str {
        return if number == 1 { 
            self.singular.as_str()
        } else {
            self.plural.as_str()
        }
    }
}

struct Stats {
    wins: u32,
    min_tries: Option<u32>,
}

struct MistakeTracker {
    last_mistake: Option<Ordering>,
    in_a_row: u32,
}

impl MistakeTracker {
    fn record(&mut self, mistake: Ordering) {
        match self.last_mistake {
            Some(last) => {
                if last == mistake {
                    self.in_a_row += 1;
                } else {
                    self.last_mistake = Some(mistake);
                    self.in_a_row = 0;
                }
            },
            None => {
                self.last_mistake = Some(mistake);
                self.in_a_row = 1;
            }
        }
    }
    fn is_distracted(&self) -> bool {
        return self.in_a_row >= 3;
    }
}

const DISTRACTED_MESSAGE: &str = "Are you paying attention?";

fn main() {
    let mut stats: Stats = Stats {
        wins: 0,
        min_tries: None,
    };

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
                play(&mut stats);
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
    }

}

// TODO: Print current number of tries
fn play(stats: &mut Stats) {
    // TODO: try a smaller type for this
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries: u32 = 0;
    let mut mistake_tracker = MistakeTracker {
        last_mistake: None,
        in_a_row: 0,
    };

    loop {
        let guess = get_guess();
        tries += 1;

        println!("You guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                mistake_tracker.record(Ordering::Less);
                if !mistake_tracker.is_distracted() {
                    println!("Too small!");
                } else {
                    println!("{DISTRACTED_MESSAGE}");
                };
            }
            Ordering::Greater => {
                mistake_tracker.record(Ordering::Greater);
                if !mistake_tracker.is_distracted() {
                    println!("Too big!");
                } else {
                    println!("{DISTRACTED_MESSAGE}");
                };
            }
            Ordering::Equal => {
                println!(
                    "You won in {} {}!",
                    tries,
                    WORDS.get("try")
                         .unwrap()
                         .get_correct_form(tries)
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
        WORDS.get("win")
             .unwrap()
             .get_correct_form(stats.wins)
    );
    if let Some(tries) = stats.min_tries {
        println!(
            "Your best game ended in {} {}",
            tries,
            WORDS.get("try")
                 .unwrap()
                 .get_correct_form(tries)
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
