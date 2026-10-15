/* Program description:
This shows how we can execute multiple commands, by using vectors (lists) and 'for (...) in (...)' mechanism.
'for (cmd, args) in commands' -!- For each iteration, 'cmd' (e.g., "echo") is passed to 'Command::new(cmd)'.
And 'args' (the vector of arguments -- the secound element of variable ''commands'' -> ''(%cmd%, vec![%args%])'') is passed to '.args(&args)'. Both are extracted directly from the 'commands' variable.
*/

use std::process::Command;

fn main() {
    let commands = vec![
        ("echo", vec!["First line code of command."]),
        ("echo", vec!["Second line code of command."]),
    ];

    for (cmd, args) in commands {
        Command::new(cmd)
            .args(&args)
            .status()
            .unwrap();
    }
}

/* Output:
First line code of command.
Second line code of command.
*/