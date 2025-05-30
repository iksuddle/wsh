use std::{
    collections::HashMap,
    error::Error,
    io::{self, Write, stdout},
    process,
};

mod commands;

#[derive(Debug)]
enum Cmd {
    Exit,
    Cd(Vec<String>),
    Pwd(Vec<String>),
    SetVar(String, String),
    GetVar(Vec<String>),
    ListVars,
    External(String, Vec<String>),
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

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // ignore ctrl+c
        let prompt = self.prompt.clone();
        ctrlc::set_handler(move || {
            print!("\n{}", prompt);
            stdout().flush().ok();
        })?;

        let mut input = String::new();
        loop {
            print!("{}", self.prompt);
            stdout().flush()?;

            input.clear();
            if io::stdin().read_line(&mut input)? == 0 {
                // 0 bytes read => ctrl+d
                println!();
                break;
            }

            let input = self.expand(&input);
            let tokens = input.split_whitespace();

            let cmds = process_input(tokens);

            if !self.execute(cmds) {
                break;
            }
        }

        Ok(())
    }

    fn execute(&mut self, cmds: Vec<Cmd>) -> bool {
        for cmd in cmds {
            match cmd {
                Cmd::Exit => return false,
                Cmd::Cd(args) => commands::cd(args),
                Cmd::Pwd(args) => commands::pwd(args),
                Cmd::SetVar(k, v) => {
                    self.env_vars.insert(k, v);
                }
                Cmd::GetVar(name) => self.get_var(name),
                Cmd::ListVars => self.list_vars(),
                Cmd::External(name, args) => self.execute_external(name, args),
            };
        }

        true
    }

    fn execute_external(&self, name: String, args: Vec<String>) {
        let status = process::Command::new(&name).args(args).status();

        match status {
            Ok(s) => {
                if !s.success() {
                    println!("{} exited with code {}", name, s);
                }
            }
            Err(_) => println!("command not found: {}", name),
        };
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

    fn get_var(&self, args: Vec<String>) {
        match args.len() {
            0 => println!("cd: expected key"),
            1 => {
                let key = args.first().unwrap();
                match self.env_vars.get(key) {
                    Some(val) => println!("{}", val),
                    None => println!("cd: key '{}' not found", key),
                }
            }
            _ => print!("cd: too many arguments"),
        }
    }

    fn list_vars(&self) {
        println!("{} items:", self.env_vars.len());
        for (k, v) in &self.env_vars {
            println!("{k}: {v}");
        }
    }
}

fn process_input<'a>(mut input: impl Iterator<Item = &'a str>) -> Vec<Cmd> {
    let mut cmds = vec![];
    while let Some(token) = input.next() {
        if let Some((k, v)) = token.split_once("=") {
            cmds.push(Cmd::SetVar(k.to_owned(), v.to_owned()));
        } else {
            let args: Vec<String> = input.map(|s| s.to_owned()).collect();

            cmds.push(match token {
                "exit" => Cmd::Exit,
                "get" => Cmd::GetVar(args),
                "lsv" => Cmd::ListVars,
                "cd" => Cmd::Cd(args),
                "pwd" => Cmd::Pwd(args),
                _ => Cmd::External(token.to_owned(), args),
            });

            break;
        }
    }

    cmds
}
