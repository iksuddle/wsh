use std::process;

use wsh::{Config, Shell};

fn main() {
    let config = Config::build(None).unwrap_or_default();

    let mut shell = Shell::new(config);

    if let Err(e) = shell.run() {
        eprintln!("application error: {}", e);
        process::exit(1);
    }
}
