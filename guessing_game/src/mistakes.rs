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
        if self.mistakes.len() == 0 {
            println!("You haven't made any guesses yet, so bounds can't be calculated!");
            return;
        }

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

        if lower_bound.is_some() && upper_bound.is_some() {
            println!(
                "The answer is between {} and {}",
                lower_bound.unwrap(),
                upper_bound.unwrap(),
            );
        } else if lower_bound.is_some() {
            println!(
                "The answer is bigger than {}",
                lower_bound.unwrap(),
            );
        } else if upper_bound.is_some() {
            println!(
                "The answer is smaller than {}",
                upper_bound.unwrap(),
            ) 
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

        if let Some(last_mistake) = &self.last_mistake() {
            let mistake_comparison = new_mistake.guess.cmp(&last_mistake.guess);
            if mistake_comparison == last_mistake.direction {
                println!("{DISTRACTED_MESSAGE}");
            }
        }
        self.mistakes.push(new_mistake);
    }
}

