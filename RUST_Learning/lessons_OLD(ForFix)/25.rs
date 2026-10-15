- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// Zmienne:

/* Integer (i8, i16, i32, i64, i128, isize) - Liczby Dodatnie i Ujemne */
let integer: i32 = 10;
let integer2: i32 = -10;

/* Unsigned Integer (u8, u16, u32, u64, u128, usize) - Liczby tylko Dodatnie */
let uinteger: u32 = 20;

/* Float (f32, f64) - Liczby Zmiennoprzecinkowe ( Ujemne i Dodatnie ) */ 
let float: f32 = 3.14;
let float2: f32 = -3.14;
let float3: f32 = 314.0;
let float4: f32 = -314.0;

/* Bool (bool) - Prawda (true) i Fałsz (false) */
let is_true: bool = true;
let is_false: bool = false;

/* Character (char) - Jeden znak Unicode (char) */
let letter: char = 'R';
let emoji: char = '🦀';

/* String (String) - Dynamiczny Tekst - Tekst (String) */
let name: String = String::from("Adamowicz");
let name2: String = String::from("Bartek");

/* String Slice (&str) - Referencja do ciągu znaków (Nie mutowalny widok na tekst.) */
let text: &str = "Text";

- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// Mutowalność:

/* Nie mutowalna zmienna (Not Mutable) */
let not_mutable: String = String::from("I'm David");
let a: i32 = 42;

/* Mutowalna zmienna (Mutable) */
let mut mutable: String = String::from("I'm ");
mutable.push_str("Michael");
let mut b: i32 = 20;
b = 40;

/* Shadowing - Tworzy NOWĄ zmienną o tej samej nazwie, nie mutuje starej */
let shadow: i32 = 2;
let shadow: i32 = shadow + 2;

- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// Stałą (Static) zmienna:

/* Ta zmienna jest stała, musi mieć jawny typ, nie może być muttowalna, wartość znana w compile-time */
const NOWA_ZMIENNA: f32 = 3.14154;

- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/* Program description:
The following program prints text to screen, and explaining what it does.
To initialize function with name "main" we use the syntax "fn %name%() {%program code of function%}" to initialize function main.
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

( The output is result of buffor )

user@linux:~$ rustc main.rs && ./main
println!();
eprintln!();
eprint!();print!();user@linux:~$

*/

- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

