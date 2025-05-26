use std::{
    collections::HashMap,
    error::Error,
    io::{self, Write},
};

#[derive(Debug)]
enum Cmd {
    ListVars,
    SetVar(String, String),
    Echo(Vec<String>),
    Exit,
    Unknown(String),
}

pub struct Shell {
    prompt: String,
    env_vars: HashMap<String, String>,
}

impl Shell {
    pub fn build(prompt: String) -> Shell {
        Shell {
            prompt,
            env_vars: HashMap::new(),
        }
    }

    fn list_vars(&self) {
        for (k, v) in &self.env_vars {
            println!("{}: {}", k, v);
        }
    }

    fn set_var(&mut self, key: String, val: String) {
        self.env_vars.insert(key, val);
    }

    fn execute(&mut self, cmds: Vec<Cmd>) -> bool {
        for cmd in cmds {
            match cmd {
                Cmd::Exit => return false,
                Cmd::Echo(args) => echo(args),
                Cmd::SetVar(k, v) => self.set_var(k, v),
                Cmd::ListVars => self.list_vars(),
                Cmd::Unknown(cmd) => println!("command not found: {}", cmd),
            };
        }

        true
    }
}

pub fn run(mut shell: Shell) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    loop {
        print!("{}", shell.prompt);
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().split_whitespace();

        let cmds = process_input(input);
        if !shell.execute(cmds) {
            break;
        }
    }

    Ok(())
}

fn process_input<'a>(mut input: impl Iterator<Item = &'a str>) -> Vec<Cmd> {
    let mut cmds = Vec::new();
    while let Some(token) = input.next() {
        if let Some((k, v)) = token.split_once("=") {
            cmds.push(Cmd::SetVar(k.to_owned(), v.to_owned()));
        } else {
            match token {
                "exit" => cmds.push(Cmd::Exit),
                "echo" => cmds.push(Cmd::Echo(input.map(|s| s.to_owned()).collect())),
                "listvars" => cmds.push(Cmd::ListVars),
                _ => cmds.push(Cmd::Unknown(token.to_owned())),
            }
            break;
        }
    }

    cmds
}

fn echo<'a>(mut args: Vec<String>) {
    let mut args_iter = args.iter_mut();

    if let Some(first) = args_iter.next() {
        print!("{}", first);
        for arg in args_iter {
            print!(" {}", arg);
        }
    }
    println!();
}
