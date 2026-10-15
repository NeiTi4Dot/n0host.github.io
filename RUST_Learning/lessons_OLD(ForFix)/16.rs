/* Program description:

``` use std::process::Command; ```
Imports a module that allows you to run external programs and system processes from your code.
With this tool, you can execute any system command, pass arguments to it, and read the result of its execution.

``` .output() ```
Starts process and wait for it to complete, then returning the result.

``` .spawn() ```
Starts process in background, and allows the program to continue running.

``` .status() ```
Starts the process, waits for it to finish and returns only the exit code.

``` loop {...} ```
Loop is the code of program, that will start itself, again and again until you break it (leave from loop).
*/

use std::io::{self, Write}; 
use std::process::Command; 

fn main() {
    println!("=== MENU ===");
    println!("1. Print Hello");
    println!("2. Execute system command (ls / dir)");
    println!("3. Leave program");
    println!("4. Help (this)");

    loop {
        print!("nobody@linux:~$ ");
        
        io::stdout()
            .flush()
            .unwrap();

        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Err! cant read line.");

        let input = input.trim();

        match input {
            "1" => {
                println!("Hello");
            }
            "2" => {
                let _ = Command::new("ls") 
                    .arg("-la")
                    .arg("--color")
                    .status();
            }
            "3" => {
                println!();
                break;
            }
            "4" | "help" => {
                println!("=== MENU ===");
                println!("1. Print Hello");
                println!("2. Execute system command (ls / dir)");
                println!("3. Leave program");
                println!("4. Help (this)");
            }
            _ => {
                println!("bash: {}: command not found", input);
            }
        }
    }
}