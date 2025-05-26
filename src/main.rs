use std::io::{self, Write};

fn main() {
    let prompt = "> ";

    let mut input = String::new();

    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

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
    let Some(token) = input.next() else {
        return false;
    };

    if token == "exit" {
        return false;
    }

    true
}
