// https://adriann.github.io/programming_problems.html
use std::io;

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();

    println!("Enter your name:");

    if let Err(_e) = stdin.read_line(&mut user_input) {
        println!("Error reading line")
    }

    println!("Hello, {} ", user_input);

    Ok(())
}
