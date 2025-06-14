use std::env;

#[derive(Debug)]
pub enum Cmd {
    Exit,
    Cd(Vec<String>),
    Pwd(Vec<String>),
    SetVar(String, String),
    GetVar(Vec<String>),
    ListVars,
    External(Vec<String>),
    Error(String),
}

impl Cmd {
    pub fn from(cmd: Vec<String>) -> Cmd {
        match cmd.first().unwrap().as_str() {
            "exit" => Cmd::Exit,
            "cd" => Cmd::Cd(cmd),
            "pwd" => Cmd::Pwd(cmd),
            "lsv" => Cmd::ListVars,
            "get" => Cmd::GetVar(cmd),
            _ => Cmd::External(cmd),
        }
    }

    pub fn build_piped_commands<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Cmd> {
        let mut cmds = vec![];
        let mut curr_cmd = vec![];

        for tok in input {
            if tok == "|" {
                if !curr_cmd.is_empty() {
                    cmds.push(Cmd::from(curr_cmd.clone()));
                    curr_cmd.clear();
                } else {
                    // error -> | |
                    cmds.push(Cmd::Error("syntax error: | |".to_owned()));
                    curr_cmd.clear();
                    break;
                }
            } else {
                curr_cmd.push(tok.to_owned());
            }
        }

        // last one
        if !curr_cmd.is_empty() {
            cmds.push(Cmd::from(curr_cmd));
        }

        cmds
    }
}

pub fn cd(args: &[String]) {
    match args.len() {
        1 => {
            let home = env::var("HOME").expect("error: $HOME not set");
            if let Err(e) = env::set_current_dir(home) {
                println!("cd: operation failed: {}", e);
            }
        }

        2 => {
            if let Err(e) = env::set_current_dir(&args[1]) {
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
