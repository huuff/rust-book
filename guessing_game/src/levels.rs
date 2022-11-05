use rand::Rng;

pub static LEVELS: [Level; 5] = [
    Level {
        max_number: 10,
        max_mistakes: 10,
    },
    Level {
        max_number: 20,
        max_mistakes: 9,
    },
    Level {
        max_number: 30,
        max_mistakes: 8,
    },
    Level {
        max_number: 40,
        max_mistakes: 7,
    },
    Level {
        max_number: 50,
        max_mistakes: 6,
    },
];

pub struct Level {
    max_number: u32,
    max_mistakes: u32,
}

impl Level {
    pub fn create_secret_number(&self) -> u32 {
        return rand::thread_rng().gen_range(1..=self.max_number);
    }
}
