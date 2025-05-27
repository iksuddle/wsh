use std::{
    collections::HashMap,
    error::Error,
    io::{self, Write},
};

#[derive(Debug)]
enum Cmd {
    Exit,
    Echo(Vec<String>),
    SetVar(String, String),
    ListVars,
    Unknown(String),
}

pub struct Shell {
    prompt: String,
    env_vars: HashMap<String, String>,
}

impl Shell {
    pub fn new(prompt: String) -> Shell {
        Shell {
            prompt,
            env_vars: HashMap::new(),
        }
    }

    fn list_vars(&self) {
        println!("{} items:", self.env_vars.len());
        for (k, v) in &self.env_vars {
            println!("{k}: {v}");
        }
    }

    fn execute(&mut self, cmds: Vec<Cmd>) -> bool {
        for cmd in cmds {
            match cmd {
                Cmd::Exit => return false,
                Cmd::Echo(args) => echo(args),
                Cmd::SetVar(k, v) => _ = self.env_vars.insert(k, v),
                Cmd::ListVars => self.list_vars(),
                Cmd::Unknown(cmd) => println!("command not found: {}", cmd),
            };
        }

        true
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();

        loop {
            print!("{}", self.prompt);
            io::stdout().flush()?;

            input.clear();
            io::stdin().read_line(&mut input)?;

            input = self.expand(&input);

            let tokens = input.split_whitespace();

            let cmds = process_input(tokens);
            if !self.execute(cmds) {
                break;
            }
        }

        Ok(())
    }

    fn expand(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '$' {
                let mut var_name = String::new();

                while let Some(&next) = chars.peek() {
                    if next.is_whitespace() || "=$".contains(next) {
                        break;
                    }
                    var_name.push(next);
                    chars.next();
                }

                if let Some(v) = self.env_vars.get(&var_name) {
                    result.push_str(v);
                } else {
                    result.push('$');
                    result.push_str(&var_name);
                }
            } else {
                result.push(c)
            }
        }

        result
    }
}

fn process_input<'a>(mut input: impl Iterator<Item = &'a str>) -> Vec<Cmd> {
    let mut cmds = Vec::new();
    while let Some(token) = input.next() {
        if let Some((k, v)) = token.split_once("=") {
            cmds.push(Cmd::SetVar(k.to_owned(), v.to_owned()));
        } else {
            let cmd = match token {
                "exit" => Cmd::Exit,
                "echo" => Cmd::Echo(input.map(|s| s.to_owned()).collect()),
                "lsv" => Cmd::ListVars,
                _ => Cmd::Unknown(token.to_owned()),
            };
            cmds.push(cmd);
            break;
        }
    }

    cmds
}

fn echo(args: Vec<String>) {
    println!("{}", args.join(" "));
}
