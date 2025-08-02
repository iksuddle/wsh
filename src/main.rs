use std::process;

use wsh::{Config, Shell};

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let config = Config::build(None).unwrap_or_default();

    let mut shell = Shell::new(config);

    if let Err(e) = shell.run().await {
        eprintln!("application error: {}", e);
        process::exit(1);
    }
}
