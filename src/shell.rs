use std::{
    collections::HashMap,
    io::{self, Write, stdout},
    process::{ChildStdout, Command, Stdio},
};

use nix::sys::signal::{SigSet, Signal};

use crate::{
    commands::{Cmd, builtins},
    scanner::Scanner,
};

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

            let mut scanner = Scanner::new(input.as_str());

            let tokens = match scanner.scan_tokens() {
                Ok(tokens) => tokens,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            let cmds = Cmd::process_input(tokens);

            // false -> exit
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
                Cmd::Cd(args) => builtins::cd(args),
                Cmd::Pwd(args) => builtins::pwd(args),
                Cmd::SetVar(k, v) => {
                    self.env_vars.insert(k.to_owned(), v.to_owned());
                }
                Cmd::GetVar(args) => self.bn_get(args),
                Cmd::ListVars => self.bn_lsv(),
                Cmd::External(cmd_tokens) => {
                    if let Some((name, args)) = cmd_tokens.split_first() {
                        let mut cmd = Command::new(name);
                        cmd.args(args);

                        if let Some(stdout) = prev_stdout.take() {
                            cmd.stdin(stdout);
                        }

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
                                break;
                            }
                        };
                    }
                }
            };
        }

        // wait for last command
        for mut child in children {
            match child.wait() {
                Ok(status) => {
                    if !status.success() {
                        println!(
                            "command [{}] exited with status: {:?}",
                            child.id(),
                            status.code()
                        );
                    }
                }
                Err(e) => {
                    println!("error waiting for command: {}", e);
                }
            }
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

                let val = self.get_var(&var_name);
                result.push_str(&val);
            } else {
                result.push(c)
            }
        }

        result
    }

    fn bn_get(&self, args: &[String]) {
        match args.len() {
            1 => println!("get: expected key"),
            2 => {
                let key = &args[1];
                println!("{}", self.get_var(key))
            }
            _ => print!("get: too many arguments"),
        }
    }

    fn bn_lsv(&self) {
        println!("{} items:", self.env_vars.len());
        for (k, v) in &self.env_vars {
            println!("{k}: {v}");
        }
    }

    fn get_var(&self, key: &str) -> String {
        if let Some(v) = self.env_vars.get(key) {
            return v.to_owned();
        } else {
            // check env vars
            let val = match std::env::var(key) {
                Ok(val) => val,
                Err(_) => "".to_owned(),
            };
            return val;
        }
    }
}
