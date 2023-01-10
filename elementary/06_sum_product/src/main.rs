// https://adriann.github.io/programming_problems.html
use std::io;

// ask user a number and let him choose if he wants a sum or a product

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut user_input).expect("Error reading line");
    let user_input_trimmed = user_input.trim();
    let user_input_num = user_input_trimmed.parse::<u32>().expect("Please enter an unsigned integer");


    println!("What do you want:
1: Compute product of numbers >= 0 && <= your number
2: Compute sum of numbers >= 0 && <= your number");
    let mut user_operation = String::new();
    io::stdin().read_line(&mut user_operation).expect("Error leading line");
    let user_operation_trimmed = user_operation.trim();
    match user_operation_trimmed.parse::<u32>() {
        Ok(i) => match i {
            1 => println!("the product is {}", compute_product(user_input_num)),
            2 => println!("the sum is {}", compute_sum(user_input_num)),
            _ => println!{"not supported"}
        },
        Err(..) => panic!("Plase enter an unsigned integer")
    }

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

#[cfg(test)]
mod tests {
    use crate::{compute_product, compute_sum};

    #[test]
    fn product() {
        assert_eq!(compute_product(3), 6);
    }

    #[test]
    fn sum() {
        assert_eq!(compute_sum(3), 6);
    }
}
