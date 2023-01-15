// https://adriann.github.io/programming_problems.html
use std::{io, u32};

// Write a program that prints all prime numbers. (Note: if your programming language does not support arbitrary size numbers, printing all primes up to the largest number you can easily represent is fine too.)

fn main() -> io::Result<()> {

    for i in 1..u32::MAX {

        println!("{}", i)
    }

    Ok(())    
}
