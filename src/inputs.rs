use std::io::{self, Write};

pub fn get_string(prompt: &str) -> String {
    let mut input = String::new();
    loop {
        print!("{}: ", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().len() > 0 {
            return input.trim().to_string();
        } else {
            println!("Please enter a valid input.");
        }
    }
}

pub fn get_int(prompt: &str) -> u32 {
    loop {
        let input = get_string(prompt);
        match input.trim().parse::<u32>() {
            Ok(i) => {
                return i;
            }
            Err(_) => {
                println!("Please enter a valid input.");
            }
        }
    }
}
