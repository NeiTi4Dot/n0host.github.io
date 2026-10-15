use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.iter().map(|s| s.as_str()).collect::<Vec<&str>>().as_slice() {
        [_, "-s", "make"] => println!("Robienie czegoś..."),
        [_, "-s", "cook"] => println!("Gotowanie czegoś..."),
        [_, "-k", "doke"] => println!("Opcja -k z doke..."),
        [_, "-k", "make"] => println!("Opcja -k z make..."),
        [_, "--costed"] => println!("Sprawdzanie kosztów..."),
        _ => println!("Nieznana komenda lub złe argumenty!"),
    }
}