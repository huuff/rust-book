use std::io::{self, Write};

pub fn get_input() -> String {
    // TODO: Put it outside, reuse reference
    print!("> ");
    let _ = io::stdout().flush();
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line")
        ;

    println!();
    
    return input;
}
