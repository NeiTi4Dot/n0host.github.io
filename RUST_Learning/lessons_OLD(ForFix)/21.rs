use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn test(tekst: &str, predkosc_ms: u64, pauza_s: u64) {
    for litera in tekst.chars() {
        print!("{}", litera);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(predkosc_ms));
    }
    println!();
    
    sleep(Duration::from_secs(pauza_s));
}

fn main() {
    test("Adam djdjdiej", 50, 3);
    test("jdndi djdidien", 50, 0);
}     