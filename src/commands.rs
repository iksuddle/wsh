use std::{env, path::PathBuf};

use crate::scanner::Token;

#[derive(Debug)]
pub enum CommandIO {
    Std,
    File(PathBuf),
}

#[derive(Debug)]
pub enum Command {
    Exit,
    Cd(Vec<String>),
    Pwd(Vec<String>),
    SetVar(String, String),
    GetVar(Vec<String>),
    ListVars,
    External {
        args: Vec<String>,
        input: CommandIO,
        output: CommandIO,
    },
    Error(String),
}

impl Command {
    pub fn from(args: Vec<String>, input: CommandIO, output: CommandIO) -> Command {
        match args.first().unwrap().as_str() {
            "exit" => Command::Exit,
            "cd" => Command::Cd(args),
            "pwd" => Command::Pwd(args),
            "lsv" => Command::ListVars,
            "get" => Command::GetVar(args),
            _ => Command::External {
                args,
                input,
                output,
            },
        }
    }

    pub fn process_input(tokens: Vec<Token>) -> Vec<Command> {
        let mut cmds = vec![];

        let mut tokens = tokens.iter().peekable();

        // consume all variable assignments
        while let Some(Token::Literal(l)) = tokens.peek() {
            if let Some((k, v)) = l.split_once("=") {
                cmds.push(Command::SetVar(k.to_owned(), v.to_owned()));
                tokens.next();
            } else {
                break;
            }
        }

        cmds.extend(Self::build_piped_commands(tokens));

        cmds
    }

    fn build_piped_commands<'a>(tokens: impl Iterator<Item = &'a Token>) -> Vec<Command> {
        let mut cmds = vec![];

        let mut curr_cmd_args = vec![];
        let mut input = CommandIO::Std;
        let mut output = CommandIO::Std;

        let mut tokens = tokens.peekable();

        while let Some(token) = tokens.next() {
            match token {
                Token::Pipe => {
                    if curr_cmd_args.is_empty() {
                        cmds.push(Command::Error("syntax error: | |".to_owned()));
                        curr_cmd_args.clear();
                        break;
                    }
                    cmds.push(Command::from(curr_cmd_args.clone(), input, output));
                    curr_cmd_args.clear();
                    input = CommandIO::Std;
                    output = CommandIO::Std;
                }
                Token::Literal(l) => {
                    curr_cmd_args.push(l.to_owned());
                }
                Token::Greater => {
                    if let Some(Token::Literal(path)) = tokens.peek() {
                        // path
                        let path = PathBuf::from(path);
                        output = CommandIO::File(path);
                        tokens.next();
                    } else {
                        cmds.push(Command::Error(
                            "syntax error: no path provided after >".to_owned(),
                        ));
                        curr_cmd_args.clear();
                        break;
                    }
                }
                Token::Less => {
                    if let Some(Token::Literal(path)) = tokens.peek() {
                        // path
                        let path = PathBuf::from(path);
                        input = CommandIO::File(path);
                        tokens.next();
                    } else {
                        cmds.push(Command::Error(
                            "syntax error: no path provided after <".to_owned(),
                        ));
                        curr_cmd_args.clear();
                        break;
                    }
                }
                Token::Eof => break,
            }
        }
        // last one
        if !curr_cmd_args.is_empty() {
            cmds.push(Command::from(curr_cmd_args, input, output));
        }

        cmds
    }
}

pub mod builtins {
    use super::*;

    pub fn cd(args: &[String]) {
        match args {
            [_] => {
                let home = env::var("HOME").expect("error: $HOME not set");
                if let Err(e) = env::set_current_dir(home) {
                    println!("cd: operation failed: {}", e);
                }
            }
            [_, directory] => {
                if let Err(e) = env::set_current_dir(directory) {
                    println!("cd: operation failed: {}", e);
                }
            }
            _ => println!("cd: too many arguments"),
        };
    }

    pub fn pwd(args: &[String]) {
        if args.len() > 1 {
            println!("pwd: too many arguments");
            return;
        }

        let curr_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                println!("pwd: {}", e);
                return;
            }
        };

        println!("{}", curr_dir.display());
    }
}
