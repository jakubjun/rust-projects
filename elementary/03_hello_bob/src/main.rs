// https://adriann.github.io/programming_problems.html
use std::io;

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();

    println!("Enter your name:");

    stdin.read_line(&mut user_input).expect("Error reading line");
    let user_input_trimmed = user_input.trim();

    if user_input_trimmed.eq("Bob") || user_input_trimmed.eq("Alice") {
        println!("Hello, {} ", user_input);
    }

    Ok(())
}
