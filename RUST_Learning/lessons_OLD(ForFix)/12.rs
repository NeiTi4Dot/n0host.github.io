/* Program description:
We use sentence "mod %file_name%;" to connect other files to this main program file.
We use "match" to add options and output for this options.
At the end "_" mean "anything else".
To link self function, you need to write "%function_name%()".
To link external function, like from file "external.rs", you need to write "%name_of_file_(mod)%::%function_name%()".
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
user@linux:~$ rustc main.rs && ./main
Program menu introduction.
Available options:
'self'      == Calling function 'main_self_func()' from file main.rs
'external'  == Calling function 'file_func()' from file external.rs
input >> self
File main.rs works correctly!.
File main.rs funtion 'main_self_func()' works correctly!.
user@linux:~$ rustc main.rs && ./main
Program menu introduction.
Available options:
'self'      == Calling function 'main_self_func()' from file main.rs
'external'  == Calling function 'file_func()' from file external.rs
input >> external
File external.rs works correctly!.
Function 'file_func()' from file external.rs (self) works correctly!.
user@linux:~$
*/ 