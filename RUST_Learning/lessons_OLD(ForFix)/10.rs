/* Program description:
The following program uses the "std::io::{self, Write};" library, to execute the following code.
Libraries are introduced with the "use" sentence at the very beginning of the program line, where it should look like this:

```
use %Crate::Module::Module_Optional::Module_Optional%;

%program%
```

To make the program ask you correctly and make it look nice, you need to do this:

```
print!("Input >> ");
```

Because we want our text to be next to "Input >> %your_input_goes_here%", not below it. We use:

```
io::stdout()
    .flush()
    .unwrap();
```

We use ".flush()" and ".unwrap()" to print this text first at screen, before input mechanism starts.
But before we start entering text, we need to create a variable named "input" that is modifiable (so we can both change it and enter text, it must be modifiable) by using the word "mut" after "let.".
We also need to specify the type of text being entered, and we want it to be a text (String -- Must be capitalized):

```
let mut input: String = String::new();
```

Once we have that, we move on to the mechanism that will allow us to enter text, and push it into "variable "input":

```
io::stdin()
    .read_line(&mut input)
    .expect("Err! wrong input.");
```

And to have cleaned input, we need to remove enter from our input, to do that we need to do:

```
let input = input.trim();
```

Where ".trim()" removes enter's.
Then at the end we print the variable "input" to the screen, to see what have we writen.
*/

use std::io::{self, Write};

fn main() {
    print!("Input >> ");

    // Print text above at first, before going forward
    io::stdout()
        .flush()
        .unwrap();

    // We make variable with name "input" muttable (modifiable) by adding "mut" before viariable name.
    // We giving "new()" because we want "empty" input, which we can modify like push (add) pop (delete) a anything into/from variable.
    let mut input: String = String::new();
    
    io::stdin()
        // This code line read input from variable named "input"
        .read_line(&mut input)
        // This code line adds your error message to the program error
        .expect("Err! -!- something goes wrong!.");

    // It's delete white spaces and enters from the end of variable content
    let input = input.trim();

    println!("You entered the following text (String): {}", input);
}

/* Example input:
Rust programming
*/

/* Output:
user@linux:~$ rustc main.rs && ./main
Input >> Rust programming
You entered the following text (String): Rust programming
user@linux:~$
*/