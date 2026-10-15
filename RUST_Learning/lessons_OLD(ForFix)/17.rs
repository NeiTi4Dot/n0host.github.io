use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_arg = args[1].as_str();

    match input_arg {
    "help" => {
            println!("used: help");
            return;
        }
        _ => {}
    };

    match args.as_slice() {
        [_, g, w] if g == "-h" && w == "help" => {
            println!("used: -h & help");
            return;
        }
        _ => {}
    };

    eprintln!("It's not valid argument: {}", input_arg);
}
