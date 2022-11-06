use std::cmp::Ordering;

const DISTRACTED_MESSAGE: &str = "Are you paying attention?";

pub struct GuessMistake {
    guess: u32,
    direction: Ordering,
}

impl GuessMistake {
    pub fn new(guess: u32, direction: Ordering) -> GuessMistake {
        GuessMistake { guess, direction }
    }
}

pub struct MistakeTracker {
    last_mistake: Option<GuessMistake>,
}

impl MistakeTracker {
    pub fn new() -> MistakeTracker {
        MistakeTracker {
            last_mistake: None,
        }
    }

    pub fn record(&mut self, new_mistake: GuessMistake) {
        match new_mistake.direction {
            Ordering::Less => {
                println!("Too small!");
            },
            Ordering::Greater => {
                println!("Too big!");
            },
            Ordering::Equal => {
                panic!("Can't have a mistake that's a correct answer");
            },
        }

        if let Some(last_mistake) = &self.last_mistake {
            let mistake_comparison = new_mistake.guess.cmp(&last_mistake.guess);
            if mistake_comparison == last_mistake.direction {
                println!("{DISTRACTED_MESSAGE}");
            }
        }
        self.last_mistake = Some(new_mistake);
    }
}

