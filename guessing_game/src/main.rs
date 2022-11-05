use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

enum Action {
    Play,
    Stats,
    Quit,
}

enum GrammaticalNumber {
    Singular,
    Plural,
}

fn main() {
    let mut wins: u32 = 0;
    let mut min_tries: Option<u32> = None;
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
                play(&mut wins, &mut min_tries);
            }
            Action::Stats => {
                stats(&wins, &min_tries);
            },
            Action::Quit => {
                println!("Bye!");
                break;
            },
        };
        println!();
    }

}

fn play(wins: &mut u32, min_tries: &mut Option<u32>) {
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
                let word = match get_grammatical_number(&tries) {
                    GrammaticalNumber::Singular => "try",
                    GrammaticalNumber::Plural => "tries",
                };
                println!("You won in {tries} {word}!");
                break;
            }
        }
    }
    *wins += 1;
    match min_tries {
        Some(min) => {
            if tries < *min {
                *min_tries = Some(tries);
            }
        },
        None => {
            *min_tries = Some(tries);
        }
    };
}

fn stats(wins: &u32, min_tries: &Option<u32>) {
    let wins_word = match get_grammatical_number(wins) {
        GrammaticalNumber::Singular => "time",
        GrammaticalNumber::Plural => "times",
    };
    println!("You've won {wins} {wins_word}");
    if let Some(tries) = min_tries {
        // TODO: I have to somehow merge this with the above use
        let min_tries_word = match get_grammatical_number(tries) {
            GrammaticalNumber::Singular => "try",
            GrammaticalNumber::Plural => "tries",
        };
        println!("Your best game ended in {tries} {min_tries_word}")

    }

}

fn get_grammatical_number(number: &u32) -> GrammaticalNumber {
    return match number {
        1 => GrammaticalNumber::Singular,
        _ => GrammaticalNumber::Plural,
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
