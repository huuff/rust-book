use std::cmp::Ordering;
use crate::bounds::Bounds;

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
    mistakes: Vec<GuessMistake>,
}

impl MistakeTracker {
    pub fn new() -> MistakeTracker {
        MistakeTracker {
            mistakes: Vec::new(),
        }
    }

    pub fn last_mistake(&self) -> Option<&GuessMistake> {
        return self.mistakes.last();
    }

    pub fn print_bounds(&self) {
        let lower_bound = self.mistakes
            .iter()
            .filter(|it| it.direction == Ordering::Less)
            .map(|it| it.guess)
            .max()
            ;
        let upper_bound = self.mistakes
            .iter()
            .filter(|it| it.direction == Ordering::Greater)
            .map(|it| it.guess)
            .min()
            ;

        let bounds = Bounds {
            lower: lower_bound,
            upper: upper_bound,
        };

        bounds.print();
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

        if let Some(last_mistake) = &self.last_mistake() {
            let mistake_comparison = new_mistake.guess.cmp(&last_mistake.guess);
            if mistake_comparison == last_mistake.direction {
                println!("{DISTRACTED_MESSAGE}");
            }
        }
        self.mistakes.push(new_mistake);
    }
}

