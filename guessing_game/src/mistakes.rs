use std::cmp::Ordering;

const DISTRACTED_MESSAGE: &str = "Are you paying attention?";

pub struct MistakeTracker {
    last_mistake: Option<Ordering>,
    in_a_row: u32,
}

impl MistakeTracker {
    pub fn new() -> MistakeTracker {
        MistakeTracker {
            last_mistake: None,
            in_a_row: 0,
        }
    }

    pub fn record(&mut self, mistake: Ordering) {
        match self.last_mistake {
            Some(last) => {
                if last == mistake {
                    self.in_a_row += 1;
                } else {
                    self.last_mistake = Some(mistake);
                    self.in_a_row = 0;
                }
            },
            None => {
                self.last_mistake = Some(mistake);
                self.in_a_row = 1;
            }
        }
        if self.is_distracted() {
            println!("{DISTRACTED_MESSAGE}");
        }
    }
    fn is_distracted(&self) -> bool {
        return self.in_a_row >= 3;
    }
}

