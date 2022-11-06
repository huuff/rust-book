use crate::GameResult;

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
            },
            GameResult::Loss => (),
        }
    }

    pub fn current_level(&self) -> usize {
        self.level_results.len()
    }

    pub fn print(&self) {
        println!("You're currently on level {}", self.current_level());
    }
}
