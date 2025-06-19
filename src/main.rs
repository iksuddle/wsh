use std::process;

use wsh::Shell;

fn main() {
    let mut shell = Shell::new("$ ".to_owned());

    if let Err(e) = shell.run() {
        eprintln!("application error: {}", e);
        process::exit(1);
    }
}
