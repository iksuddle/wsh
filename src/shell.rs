use std::{
    collections::HashMap,
    fs::File,
    io,
    process::{self, ChildStdout, Stdio},
};

use rustyline::{DefaultEditor, error::ReadlineError};

use crate::{
    Config,
    commands::{Command, CommandIO, builtins},
    scanner::Scanner,
    wish::{CmdGen, WishError},
};

enum ExecError {
    Exit,
    CommandNotFound(String),
    PermissionError,
}

enum ShellMode {
    Normal,
    Wish,
}

pub struct Shell {
    prompt: String,
    line_reader: DefaultEditor,
    env_vars: HashMap<String, String>,
    cmd_gen: CmdGen,
    mode: ShellMode,
}

impl Shell {
    pub fn new(config: Config) -> Shell {
        Shell {
            prompt: config.prompt,
            line_reader: DefaultEditor::new().expect("error creating line editor"),
            env_vars: HashMap::new(),
            cmd_gen: CmdGen::new(),
            mode: ShellMode::Normal,
        }
    }

    pub async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            match self.mode {
                ShellMode::Normal => {
                    let input = match self.line_reader.readline(&self.prompt) {
                        Ok(line) => line,
                        Err(ReadlineError::Interrupted) => continue,
                        Err(ReadlineError::Eof) => break,
                        Err(err) => {
                            println!("error: {:?}", err);
                            break;
                        }
                    };

                    let input = self.expand(&input);

                    let mut scanner = Scanner::new(input.as_str());

                    let tokens = match scanner.scan_tokens() {
                        Ok(tokens) => tokens,
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                    };

                    let cmds = Command::process_input(tokens);

                    match self.execute(cmds).await {
                        Err(ExecError::Exit) => break,
                        Err(ExecError::PermissionError) => println!("permission denied"),
                        Err(ExecError::CommandNotFound(cmd)) => {
                            println!("command not found: {cmd}")
                        }
                        _ => (),
                    };
                }
                ShellMode::Wish => {
                    let input = match self.line_reader.readline(">> ") {
                        Ok(line) => line,
                        Err(err) => {
                            println!("error: {:?}", err);
                            self.mode = ShellMode::Normal;
                            continue;
                        }
                    };

                    if input.trim() == "exit" {
                        self.mode = ShellMode::Normal;
                        continue;
                    }

                    println!("generating commands...");

                    let res = self.cmd_gen.generate_commands(input).await;
                    match res {
                        Ok(commands) => {
                            self.request_commands_execution(commands).await;
                        }
                        Err(WishError::Gemini(e)) => println!("{e}"),
                        Err(e) => println!("{e}"),
                    }
                }
            }
        }
        Ok(())
    }

    async fn request_commands_execution(&mut self, commands: Vec<String>) {
        for c in commands {
            println!("\n-> {c}");
            let decision = match self.line_reader.readline("Execute? [y/N] ") {
                Ok(decision) => decision,
                Err(_) => {
                    println!("stopping execution");
                    break;
                }
            };
            println!();
            match decision.to_lowercase().as_str() {
                "y" | "yes" => {
                    let mut scanner = Scanner::new(c.as_str());

                    let tokens = match scanner.scan_tokens() {
                        Ok(tokens) => tokens,
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                    };

                    let cmds = Command::process_input(tokens);

                    match self.execute(cmds).await {
                        Err(ExecError::Exit) => break,
                        Err(ExecError::CommandNotFound(cmd)) => {
                            println!("command not found: {cmd}")
                        }

                        Err(ExecError::PermissionError) => println!("permission denied"),
                        _ => (),
                    }
                }
                // not yes, break
                _ => break,
            };
        }
    }

    async fn execute(&mut self, cmds: Vec<Command>) -> Result<(), ExecError> {
        let mut prev_stdout: Option<ChildStdout> = None;
        let mut children = Vec::new();

        for (i, cmd) in cmds.iter().enumerate() {
            match cmd {
                Command::Error(msg) => println!("error: {}", msg),
                Command::Exit => return Err(ExecError::Exit),
                Command::Cd(args) => builtins::cd(args),
                Command::Pwd(args) => builtins::pwd(args),
                Command::Help => builtins::help(),
                Command::SetVar(k, v) => {
                    self.env_vars.insert(k.to_owned(), v.to_owned());
                }
                Command::GetVar(args) => self.bn_get(args),
                Command::ListVars => self.bn_lsv(),
                Command::Wish => {
                    println!("entering wish mode...");
                    self.mode = ShellMode::Wish;
                }
                Command::External {
                    args,
                    input,
                    output,
                } => {
                    if let Some((name, args)) = args.split_first() {
                        let mut cmd = process::Command::new(name);
                        cmd.args(args);

                        if let Some(stdout) = prev_stdout.take() {
                            cmd.stdin(stdout);
                        }

                        if i != cmds.len() - 1 {
                            cmd.stdout(Stdio::piped());
                        }

                        if let CommandIO::File(path) = input {
                            cmd.stdin(match File::open(path) {
                                Ok(file) => file,
                                Err(e) => {
                                    println!("error reading file: {}", e);
                                    return Err(ExecError::PermissionError);
                                }
                            });
                        }

                        if let CommandIO::File(path) = output {
                            cmd.stdout(File::create(path).expect("error creating file"));
                        }

                        match cmd.spawn() {
                            Ok(mut child) => {
                                prev_stdout = child.stdout.take();
                                children.push(child);
                            }
                            Err(_) => return Err(ExecError::CommandNotFound(name.to_owned())),
                        };
                    }
                }
            };
        }

        // wait for commands
        for mut child in children {
            match child.wait() {
                Ok(status) => {
                    if !status.success() {
                        println!(
                            "command {} exited with status {:?}",
                            child.id(),
                            status.code().unwrap_or_default()
                        );
                    }
                }
                Err(e) => {
                    println!("error waiting for command: {}", e);
                }
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
        }
        // check env vars
        match std::env::var(key) {
            Ok(val) => val,
            Err(_) => "".to_owned(),
        }
    }
}
