use std::env;

pub fn cd(args: &[String]) {
    match args.len() {
        1 => {
            let home = env::var("HOME").expect("error: $HOME not set");
            if let Err(e) = env::set_current_dir(home) {
                println!("cd: operation failed: {}", e);
            }
        }

        2 => {
            if let Err(e) = env::set_current_dir(args.first().unwrap()) {
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
