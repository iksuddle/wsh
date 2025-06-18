use std::{
    collections::HashMap,
    io::{self, Write, stdout},
    process::{ChildStdout, Command, Stdio},
    vec,
};

use nix::sys::signal::{SigSet, Signal};

mod commands;
pub mod scanner;

use commands::Cmd;

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

    pub fn run(&mut self) -> Result<(), io::Error> {
        let mut to_block = SigSet::empty();
        to_block.add(Signal::SIGINT);
        to_block.add(Signal::SIGTERM);
        to_block.add(Signal::SIGQUIT);
        to_block.add(Signal::SIGTSTP);
        to_block
            .thread_block()
            .expect("error adding signals to mask");

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
        let mut prev_stdout: Option<ChildStdout> = None;
        let mut children = Vec::new();

        for (i, cmd) in cmds.iter().enumerate() {
            match cmd {
                Cmd::Error(msg) => println!("error: {}", msg),
                Cmd::Exit => return false,
                Cmd::Cd(args) => commands::cd(args),
                Cmd::Pwd(args) => commands::pwd(args),
                Cmd::SetVar(k, v) => {
                    self.env_vars.insert(k.to_owned(), v.to_owned());
                }
                Cmd::GetVar(args) => self.get_var(args),
                Cmd::ListVars => self.list_vars(),
                Cmd::External(cmd_tokens) => {
                    if let Some((name, args)) = cmd_tokens.split_first() {
                        let mut cmd = Command::new(name);
                        cmd.args(args);

                        // not first command
                        if let Some(stdout) = prev_stdout.take() {
                            cmd.stdin(stdout);
                        }

                        // not last command
                        if i != cmds.len() - 1 {
                            cmd.stdout(Stdio::piped());
                        }

                        match cmd.spawn() {
                            Ok(mut child) => {
                                prev_stdout = child.stdout.take();
                                children.push(child);
                            }
                            Err(e) => {
                                println!("error executing command {}: {}", name, e);
                            }
                        };
                    }
                }
            };
        }

        // wait for last command
        if let Some(mut last) = children.pop() {
            let _ = last.wait();
        }

        // wait for earlier commands
        for mut child in children {
            let _ = child.wait();
        }

        true
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

    fn get_var(&self, args: &[String]) {
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

fn process_input<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Cmd> {
    let mut cmds = vec![];
    let mut input = input.peekable();

    while let Some(token) = input.peek() {
        if let Some((k, v)) = token.split_once("=") {
            cmds.push(Cmd::SetVar(k.to_owned(), v.to_owned()));
            input.next();
        } else {
            cmds.extend(Cmd::build_piped_commands(input));
            break;
        }
    }

    cmds
}
