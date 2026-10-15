/* Program description:
We use sentence "mod %file_name%;" to connect other files to this main program file.
We use "match" to add options and output for this options.
At the end "_" mean "anything else".
To link self function, you need to write "%function_name%()".
To link external function, like from file "external.rs", you need to write "%name_of_file_(mod)%::%function_name%()".
We have options:

```
use std::io::{self, Write};
mod external;

fn main_self_func() {
    println!("File main.rs works correctly!.");
    println!("File main.rs funtion 'main_self_func()' works correctly!.");
}

fn main() {
  loop {
    print!("input >> ");

    io::stdout()
        .flush()
        .unwrap();

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Bład programu!");

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
        }
        ("external", _) => {
            external::file_func();
        }
        ("100" | "text" | "Y", _) => {
            println!("Execute if: Number '0' or String 'text' or Char 'Y'.");
        }
        _ => {
            println!("That not a String or Number, something goes wrong!. Reloading...");
            continue;
        }
    };


    
  };
}
```
*/

use std::io::{self, Write};
mod external;

fn main_self_func() {
    println!("File main.rs works correctly!.");
    println!("File main.rs funtion 'main_self_func()' works correctly!.");
}

fn main() {
    println!("Program menu introduction.");
    println!("Available options:");
    println!("'self'      == Calling function 'main_self_func()' from file main.rs");
    println!("'external'  == Calling function 'file_func()' from file external.rs");

    print!("input >> ");

    io::stdout()
        .flush()
        .unwrap();

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Bład programu!");

    let input = input.trim();

    match input {
        "self" => {
            main_self_func();
        }
        "external" => {
            external::file_func();
        }
        _ => {
            // This line below exit program. But to exit loop not the program (because we have program out of the loop too) we need to use "break" instead of "return".
            return;
        }
    };
}

/* Output (example):
Program menu introduction.
Available options:
'self'      == Calling function 'main_self_func()' from file main.rs
'external'  == Calling function 'file_func()' from file external.rs
input >> self
File main.rs works correctly!.
File main.rs funtion 'main_self_func()' works correctly!.
*/ 