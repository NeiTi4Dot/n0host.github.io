use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn test(tekst: &str, pauza_s: u64) {
    println!("{}", tekst);
    io::stdout().flush().unwrap();
    sleep(Duration::from_secs(pauza_s));
}

fn main() {
    test("Adam djdjdiej", 3);
    test("jdndi djdidien", 0);
}