pub struct Bounds {
    pub lower: u32,
    pub upper: u32,
}

impl Bounds {
    pub fn print(&self) {
        println!("The answer is between {} and {}", self.lower, self.upper);
    }
}
