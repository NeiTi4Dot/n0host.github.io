use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    for i in 1..=5 {
        print!("\rLicznik: {}", i);
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
    
    print!("\rGotowe!          \n");
    io::stdout().flush().unwrap();
}
     