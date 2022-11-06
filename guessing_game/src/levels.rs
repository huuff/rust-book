use rand::Rng;
use crate::stuff::PowerUp;

pub static LEVELS: [Level; 10] = [
    Level {
        max_number: 10,
        max_tries: 10,
        possible_drops: [
            PossibleDrop {
                power_up: PowerUp::ExtraTry,
                chance: 0.05,
            }
        ]
    },
    Level {
        max_number: 20,
        max_tries: 9,
        possible_drops: [
            PossibleDrop {
                power_up: PowerUp::ExtraTry,
                chance: 0.07,
            }
        ]
    },
    Level {
        max_number: 30,
        max_tries: 8,
        possible_drops: [
            PossibleDrop {
                power_up: PowerUp::ExtraTry,
                chance: 0.09,
            }
        ]
    },
    Level {
        max_number: 40,
        max_tries: 7,
        possible_drops: [
            PossibleDrop {
                power_up: PowerUp::ExtraTry,
                chance: 0.11,
            }
        ]
    },
    Level {
        max_number: 50,
        max_tries: 6,
        possible_drops: [
            PossibleDrop {
                power_up: PowerUp::ExtraTry,
                chance: 0.13,
            }
        ]
    },
    Level {
        max_number: 60,
        max_tries: 5,
        possible_drops: [
            PossibleDrop {
                power_up: PowerUp::ExtraTry,
                chance: 0.15,
            }
        ]
    },
    Level { 
        max_number: 70,
        max_tries: 4,
        possible_drops: [
            PossibleDrop {
                power_up: PowerUp::ExtraTry,
                chance: 0.17,
            }
        ]
    },
    Level { 
        max_number: 80,
        max_tries: 3,
        possible_drops: [
            PossibleDrop {
                power_up: PowerUp::ExtraTry,
                chance: 0.19,
            }
        ]
    },
    Level {
        max_number: 90,
        max_tries: 2,
        possible_drops: [
            PossibleDrop {
                power_up: PowerUp::ExtraTry,
                chance: 0.21,
            }
        ]
    },
    Level {
        max_number: 100,
        max_tries: 1,
        possible_drops: [
            PossibleDrop {
                power_up: PowerUp::ExtraTry,
                chance: 0.23,
            }
        ]
    }
];

pub struct PossibleDrop {
    pub power_up: PowerUp,
    pub chance: f64,
}

pub struct Level {
    pub max_number: u32,
    pub max_tries: u32,
    // The size of the possible_drops array is the number of existent drops
    pub possible_drops: [PossibleDrop; 1],
}

impl Level {
    pub fn create_secret_number(&self) -> u32 {
        return rand::thread_rng().gen_range(1..=self.max_number);
    }

    pub fn get_maybe_drop(&self) -> Option<PowerUp> {
        let mut rng = rand::thread_rng();
        for possible_drop in self.possible_drops.iter() {
            let random = rng.gen::<f64>();
            if random < possible_drop.chance {
                match possible_drop.power_up {
                    PowerUp::ExtraTry => {
                        println!("You found an ExtraTry!");
                    }
                };
                return Some(possible_drop.power_up.clone());
            }
        }

        return None;
    }
}
