use std::process;

fn main() {
    let mut shell = wsh::Shell::new("> ".to_owned());

    if let Err(e) = shell.run() {
        eprintln!("application error: {}", e);
        process::exit(1);
    }
}
