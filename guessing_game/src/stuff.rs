use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PowerUp {
    ExtraTry,
}

pub struct Inventory {
    power_ups: HashMap<PowerUp, u32>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            power_ups: HashMap::new()
        }
    }

    pub fn add(&mut self, power_up: PowerUp) {
        if self.power_ups.contains_key(&power_up) {
            let previous_amount = self.power_ups[&power_up];
            self.power_ups.insert(power_up, previous_amount + 1);
        } else {
            self.power_ups.insert(power_up, 1);
        }
    }

    // TODO: Actually print them
    pub fn print(&self) {
        if self.power_ups.len() == 0 {
            println!("You don't have anything in your inventory");
        }
    }
}
