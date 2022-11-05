use rand::Rng;

// TODO: Try to not make it a global. Create it in the main and pass it down
pub static LEVELS: [Level; 5] = [
    Level {
        max_number: 10,
        max_tries: 10,
    },
    Level {
        max_number: 20,
        max_tries: 9,
    },
    Level {
        max_number: 30,
        max_tries: 8,
    },
    Level {
        max_number: 40,
        max_tries: 7,
    },
    Level {
        max_number: 50,
        max_tries: 6,
    },
];

pub struct Level {
    pub max_number: u32,
    pub max_tries: u32,
}

impl Level {
    pub fn create_secret_number(&self) -> u32 {
        return rand::thread_rng().gen_range(1..=self.max_number);
    }
}
