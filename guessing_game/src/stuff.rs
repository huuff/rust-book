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

    pub fn has(&self, power_up: PowerUp) -> bool {
        if let Some(amount) = self.power_ups.get(&power_up) {
            return *amount > 0;
        } else {
            return false;
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

    pub fn remove(&mut self, power_up: PowerUp) {
        if self.power_ups.contains_key(&power_up) {
            let previous_amount = self.power_ups[&power_up];
            self.power_ups.insert(power_up, previous_amount - 1);
        }
    }

    pub fn print(&self) {
        if self.power_ups.len() == 0 {
            println!("You don't have anything in your inventory");
        } else {
            println!("Inventory");
            println!("---------");
            println!("Extra Try: {}", self.power_ups[&PowerUp::ExtraTry])
        }
    }
}
