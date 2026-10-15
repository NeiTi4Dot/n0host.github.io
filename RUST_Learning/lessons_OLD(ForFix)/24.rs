use std::io::{self, Write};
use std::fs::{self, OpenOptions};

fn main() {

	print!("Podaj imie : ");
	io::stdout()
	    .flush()
	    .unwrap();

	let mut pim: String = String::new();
	io::stdin()
	    .read_line(&mut pim)
	    .expect("Error!");

	let pim = pim.trim();

    print!("Podaj drugie imie : ");
    	io::stdout()
    	.flush()
    	.unwrap();

	let mut pdi: String = String::new();
	io::stdin()
	    .read_line(&mut pdi)
	    .expect("Error!");

	let pdi = pdi.trim();

    print!("Podaj nazwisko : ");
	io::stdout()
	    .flush()
	    .unwrap();

	let mut pna: String = String::new();
	io::stdin()
	    .read_line(&mut pna)
	    .expect("Error!");

	let pna = pna.trim();

    print!("Podaj date urodzenia : ");
	io::stdout()
	    .flush()
	    .unwrap();

	let mut pdu: String = String::new();
	io::stdin()
	    .read_line(&mut pdu)
	    .expect("Error!");

	let pdu = pdu.trim();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("info.cfg")
        .unwrap();

    fs::write("info.cfg", "" ).unwrap();
    writeln!(file, "-- START --" ).unwrap();
    writeln!(file, "Imie: {}", pim).unwrap();
    writeln!(file, "Drugie Imie: {}", pdi).unwrap();
    writeln!(file, "Nazwisko: {}", pna).unwrap();
    writeln!(file, "Data Urodzenia: {}", pdu).unwrap();
    writeln!(file, "--- END ---" ).unwrap();
}
