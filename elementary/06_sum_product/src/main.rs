// https://adriann.github.io/programming_problems.html
use std::io;

// ask user a number and let him choose if he wants a sum or a product

fn main() -> io::Result<()> {
    println!("{}",compute_sum(4));
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

        if user_input_num % 3 == 0 || user_input_num % 5 == 0 {
            sum += user_input_num;
        }

        user_input_num -= 1;

    }

    println!("{}", sum);

    Ok(())
}

fn compute_product_recursive(accumulator: u32, iterator: u32) -> u32 {
    if iterator <= 0 {
        return accumulator
    }
    compute_product_recursive(iterator * accumulator, iterator - 1)
}

fn compute_product(num: u32) -> u32 {
    compute_product_recursive(num, num - 1)
}

fn compute_sum(num: u32) -> u32 {
    compute_sum_recursive(num, num - 1)
}

fn compute_sum_recursive(accumulator: u32, iterator: u32) -> u32 {
    match iterator.cmp(&0) {
        std::cmp::Ordering::Greater => compute_sum_recursive(iterator + accumulator, iterator - 1),
        _ => accumulator
    }
}
