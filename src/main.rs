use std::io::{self, Write};

mod commands;

fn main() {
    let prompt = "> ";

    let mut input = String::new();

    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("error flushing stdout");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("error reading input");

        let input = input.trim().split_whitespace();

        if !process_input(input) {
            break;
        }
    }
}

fn process_input<'a>(mut input: impl Iterator<Item = &'a str>) -> bool {
    let Some(cmd) = input.next() else {
        return false;
    };

    match cmd {
        "exit" => return false,
        "echo" => commands::echo(input),
        _ => println!("command not found"),
    };

    true
}
