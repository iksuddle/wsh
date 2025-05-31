use std::env;

pub fn cd(args: &[String]) {
    match args.len() {
        0 => {
            if let Some(home) = env::var_os("HOME") {
                if let Err(e) = env::set_current_dir(home) {
                    println!("cd: operation failed: {}", e);
                }
            } else {
                println!("cd: HOME not set");
            }
        }
        1 => {
            if let Err(e) = env::set_current_dir(args.first().unwrap()) {
                println!("cd: operation failed: {}", e);
            }
        }
        _ => println!("cd: too many arguments"),
    };
}

pub fn pwd(args: &[String]) {
    match args.len() {
        0 => {
            let curr_dir = match env::current_dir() {
                Ok(dir) => dir,
                Err(e) => {
                    println!("pwd: {}", e);
                    return;
                }
            };

            println!("{}", curr_dir.display());
        }
        _ => println!("pwd: too many arguments"),
    }
}
