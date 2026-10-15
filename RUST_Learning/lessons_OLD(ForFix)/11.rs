/* Program description:
Info!:
This program show almost any (begginer) options, to use inside match. 
*/

use std::io::{self, Write};
mod external;

fn main_self_func() {
    println!("File main.rs works correctly!.");
    println!("File main.rs funtion 'main_self_func()' works correctly!.");
}

fn main() {
  loop {
    println!("To exit program please type: 'exit'.");
    print!("input >> ");

    io::stdout()
        .flush()
        .unwrap();

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Err! something goes wrong!");

    let input = input.trim();
    let input_num = &input.parse::<u32>().ok();

    match (input, input_num) {
        (_, Some(0_u32..=10_u32)) => {
            println!("If number is from 0 to 10. Including 10");
        }
        (_, Some(11_u32..=15_u32)) => {
            println!("If number is from 11 to 15. Including 15");
        }
        (_, Some(num @ 16_u32..=20_u32)) => {
            println!("Matched (true): {:?} to range from 16 to 20 (included).", num);
        }
        ("self", _) => {
            main_self_func();
            continue;
        }
        ("external", _) => {
            external::file_func();
            continue;
        }
        ("100" | "text" | "Y", _) => {
            println!("Execute if: Number '0' or String 'text' or Char 'Y'.");
        }
        ("exit", _) => {
            break;
        }
        _ => {
            println!("Unknown option, something goes wrong!. Please try again ...");
            continue;
        }
    };
  };
}

/* Example:
0-10             >> 7
11-15            >> 15
16-20            >> 19
self             >> self
external         >> external
Y | 100 | text   >> Y
_                >> random
*/

/* Output:
input >> 7
If number is from 0 to 10. Including 10
input >> 15
If number is from 11 to 15. Including 15
input >> 19
Matched (true): 19 to range from 16 to 20 (included).
input >> self
File main.rs works correctly!.
File main.rs funtion 'main_self_func()' works correctly!.
input >> external
File external.rs works correctly!.
Function 'file_func()' from file external.rs (self) works correctly!.
input >> Y
Execute if: Number '0' or String 'text' or Char 'Y'.
input >> random
Unknown option, something goes wrong!. Please try again ...
*/