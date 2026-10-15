/* Program description:
The following program prints text to screen, and explaining what it does.
To initialize function with name "main" we using here sentence "fn %name%() {%program code of function%}" to initialize function main.
*/

fn main() {

  /* Normal print */

    // Print with a new line (enter) at the end
    println!("println!();");

    // Print without a new line (enter) at the end
    print!("print!();");

  /* Error Print */

    // Print an Error with a new line (enter) at the end
    eprintln!("eprintln!();");

    // Print an Error without a new line (enter) at the end
    eprint!("eprint!();");

}

/* Output 

user@linux:~$ rustc main.rs && ./main
println!();
eprintln!();
eprint!();print!();user@linux:~$

*/