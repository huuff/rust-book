use crate::words::WORDS;

pub struct Stats {
    pub wins: u32,
    pub min_tries: Option<u32>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            wins: 0,
            min_tries: None,
        }
    }

    pub fn record(&mut self, tries: u32) {
        self.wins += 1;
        match self.min_tries {
            Some(min) => {
                if tries < min {
                    self.min_tries = Some(tries);
                }
            },
            None => {
                self.min_tries = Some(tries);
            }
        };
    }

    pub fn print(&self) {
        println!(
            "You've won {} {}",
            self.wins,
            WORDS["win"].get_correct_form(self.wins)
        );
        if let Some(tries) = self.min_tries {
            println!(
                "Your best game ended in {} {}",
                tries,
                WORDS["try"].get_correct_form(tries)
            );
        }

    }
}
