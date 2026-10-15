/* Program description:
```
let result = String::from_utf8_lossy(&output.stdout);
```
Converts the byte result to a string.

Info!:
Its prints System info (version).
*/

use std::process::Command;

fn main() {
    match Command::new("uname")
        .arg("-a") 
        .output() 
    {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output.stdout);
            println!("System version: {}", result);
        }
        Err(error) => eprintln!("Err! while executing command: {}", error),
    }
}

/* Output:
System version: %output%
*/