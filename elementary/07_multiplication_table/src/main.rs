// https://adriann.github.io/programming_problems.html
use std::io;

// Write a program that prints a multiplication table for numbers up to 12.

fn main() -> io::Result<()> {

    const UP_TO: u32 = 12; 

    for i in 1..UP_TO {
        print!("{0: <3}", i);
        for j in 1..UP_TO {
            print!(" {0: <3}", j*i);
        }
        print!("\n");
    }

    Ok(())    
}
