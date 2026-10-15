use std::io::{self, Write};
use std::fs::{self, OpenOptions};

fn main() {

	print!("File name: ");
	io::stdout()
		.flush()
		.unwrap();

    let mut create_file: String = String::new();
	io::stdin()
		.read_line(&mut create_file)
		.expect("Error");

	let create_file = create_file.trim();

	let mut file = OpenOptions::new()
		.create(true)
		.append(true)
		.open(&create_file)
		.unwrap();

	loop {

		let mut create_note: String = String::new();
		
		io::stdin()
			.read_line(&mut create_note)
			.expect("Error");

		let create_note = create_note.trim_end().to_string();

		match create_note.as_str() {
			"CLEAR;" => {
				fs::write(create_file, "").unwrap();
				continue;
			}
			"END;" => {
				return;
			}
			_ => {
				let create_note = format!("{}", create_note.trim_end());
				write!(file, "{}\n", create_note).unwrap();
			}
		}
	}
}
