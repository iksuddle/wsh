use std::process;

fn main() {
    let shell = wsh::Shell::build("> ".to_owned());

    if let Err(e) = wsh::run(shell) {
        eprintln!("application error: {}", e);
        process::exit(1);
    }
}
