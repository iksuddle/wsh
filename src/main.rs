use std::process;

use wsh::{Config, Shell};

fn main() {
    // todo: parse --prompt $ from args
    let config = Config::build(None).unwrap_or_else(|err| {
        println!("{}", err);
        Config::default()
    });

    let mut shell = Shell::new(config);

    if let Err(e) = shell.run() {
        eprintln!("application error: {}", e);
        process::exit(1);
    }
}
