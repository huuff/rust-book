pub struct Bounds {
    pub lower: Option<u32>,
    pub upper: Option<u32>,
}

impl Bounds {
    pub fn print(&self) {
        if self.lower.is_some() && self.upper.is_some() {
            println!(
                "The answer is between {} and {}",
                self.lower.unwrap(),
                self.upper.unwrap(),
            );
        } else if self.lower.is_some() {
            println!(
                "The answer is bigger than {}",
                self.lower.unwrap(),
            );
        } else if self.upper.is_some() {
            println!(
                "The answer is smaller than {}",
                self.upper.unwrap(),
            ) 
        } else {
            println!("You haven't made any guesses yet, so bounds can't be calculated!");
        }
    }
}
