use crate::GameResult;
use crate::words::WORDS;
use colored::Colorize;

pub struct Stats {
    pub level_results: Vec<u32>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            level_results: Vec::new(),
        }
    }

    pub fn record(&mut self, result: GameResult) {
        match result {
            GameResult::Win(tries) => {
                self.level_results.push(tries);
                println!(
                    "You won in {} {}!",
                    tries,
                    WORDS["try"].get_correct_form(tries)
                );
                if tries == 1 {
                    println!("Awesome!");
                }
            },
            GameResult::Loss => {
                println!("You lost!");
            },
        }
    }

    pub fn current_level(&self) -> usize {
        self.level_results.len()
    }

    pub fn print(&self) {
        println!("You're currently on level {}", self.current_level() + 1);
        if self.current_level() > 0 {
            println!("Previous results:");
            println!("----------------");
            for (i, tries) in self.level_results.iter().enumerate() {
                let level_string = format!(
                    "Level {}: {} {}",
                    i+1,
                    tries,
                    WORDS["try"].get_correct_form(*tries),
                );
                println!(
                    "{}",
                    if *tries == 1 {
                        level_string.green().bold()
                    } else { 
                        level_string.white()
                    }
                );

            }
        }
    }
}
