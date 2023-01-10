// https://adriann.github.io/programming_problems.html
use std::io;

// ask user number and print sum of numbers up to user's number

fn main() -> io::Result<()> {
    let mut user_input = String::new();

    println!("Enter a number:");

    io::stdin().read_line(&mut user_input).expect("Error reading line");

    let user_input_trimmed = user_input.trim();
    let mut user_input_num = user_input_trimmed.parse::<u32>().expect("Please enter an unsigned integer");

    let mut sum = 0;

    loop {
        if user_input_num <= 0 {
            break;
        }

        sum += user_input_num;
        user_input_num -= 1;

    }

    println!("{}", sum);

    Ok(())
}
