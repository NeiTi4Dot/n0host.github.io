use std::process::Command;

fn main() {
    let uid_output = Command::new("id")
        .arg("-u")
        .output()
        .expect("Failed to execute id command");

    let uid_str = String::from_utf8_lossy(&uid_output.stdout);
    let uid = uid_str.trim().parse::<u32>().unwrap_or(1);

    if uid != 0 {
        eprintln!("[BŁĄD] Ta aplikacja wymaga uprawnień administratora (sudo)!");
        std::process::exit(1);
    }

    println!("Fixing problem: 'VT-x is being used by (...)'");

    let status1 = Command::new("rmmod")
        .arg("kvm_intel")
        .status()
        .unwrap();

    let status2 = Command::new("rmmod")
        .arg("kvm")
        .status()
        .unwrap();

    if status1.success() && status2.success() {
        println!("Problem fixed! ^ - ^");
    } else {
        eprintln!("[BŁĄD] Nie udało się usunąć modułów, mimo uprawnień root.");
    }
}