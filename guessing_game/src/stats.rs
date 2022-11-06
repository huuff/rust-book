use crate::GameResult;

pub struct Stats {
    pub level: usize,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            level: 0,
        }
    }

    pub fn record(&mut self, result: GameResult) {
        match result {
            GameResult::Win(tries) => {
                self.level += 1;
            },
            GameResult::Loss => (),
        }
    }

    pub fn print(&self) {
        println!("You're currently on level {}", self.level + 1);
    }
}
