use std::io;

fn main() {

    // Input operation
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Cant read input!.");
    let input = input.trim();

    match input.parse::<i64>() {
        Ok(hande) => {
            println!("input (i64): {:?}", hande);
        }
        Err(_) => {
             match input.parse::<f64>() {
                 Ok(hande) => {
                     println!("input (f64): {:?}", hande);
                 }
                 Err(_) => {
                     println!("This is string!.");
                 }
            }
        }
    }
}