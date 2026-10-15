use std::io;

fn main() {

    // Making muttable variable "input" with type "String".
    let mut input: String = String::new();
    // Reading input.
    io::stdin().read_line(&mut input).expect("Cant read input!.");
    // Removing white spaces and enters from input.
    let input = input.trim();
    // Parsing input into i64 (num).
    let hande: Result<i64, _> = input.parse();
    // Handling error and making operations print if Ok.
    match hande {
        Ok(kikim) => {
            println!("input (i64): {:?}", kikim);
        }
        Err(_) => {
            println!("This is string!.");
        }
    }
}
