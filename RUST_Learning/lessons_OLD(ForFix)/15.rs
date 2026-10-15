/*
Info!:
    usize = u32
    s1  -!- input
    len -!- length of input (connected with function length() linked to s1)

We remove the enter by .trim().
Then linking s1 (input) to function 'length()'.
Then print the Result of input (s1) and the length of input (len).
*/

use std::io::{self, Write};

fn main() {
    print!("Input your word/s: ");

    io::stdout()
        .flush()
        .unwrap();

    let mut s1 = String::new();

    io::stdin()
        .read_line(&mut s1)
        .expect("Err!");

    let s1 = s1.trim();
    let len = length(s1);

    println!("Your input: {}", s1);
    println!("Length of '{}' is {} characters.", s1, len);
}

fn length(s: &str) -> usize {
    s.len()
}

/* Example:
i'm rust teacher (16 characters)
*/

/* Output:
Input your word/s: i'm rust teacher
Your input: i'm rust teacher
Length of 'i'm rust teacher' is 16 characters.
*/