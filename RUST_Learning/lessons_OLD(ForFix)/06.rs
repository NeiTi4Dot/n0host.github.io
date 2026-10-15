use std::io::{self, Write};

fn input() -> String {
	let mut prompt: String = String::new();
	io::stdin()
	    .read_line(&mut prompt)
	    .unwrap();
	prompt = prompt.trim().to_string();
    prompt
}

fn ask(prompt: &str) {
	print!("\x1b[0m{}: \x1b[31m", prompt);
	io::stdout()
	    .flush()
	    .unwrap();
}

fn main() {
	ask("Imie");
	let imie  = input();
			
	ask("Nazwisko");
	let nazwisko = input();
			
	ask("Data Urodzenia");
	let dataurodzenia = input();
			
	println!("\x1b[0mTwoje dane to:");
	println!("Imie: {imie}");
	println!("Nazwisko: {nazwisko}");
	println!("Data Urodzenia: {dataurodzenia}");
}
