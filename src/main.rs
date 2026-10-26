use std::io::{self, Write};

fn main() {
    println!("Syntax example: ex PATH=/usr/bin");
    loop {
        print!("~> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        let parts = input.trim().replace("ex ", "");
        let mut parts_2nd = parts.split("=");
        let first = parts_2nd.next();
        let second = parts_2nd.next();
        if let (Some(var), Some(val)) = (first, second) {
            println!("Variable: {}", var.trim());
            println!("Value: {}", val.trim());
        } else {
            println!("Idiot")
        }
    }
}
