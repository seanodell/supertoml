// This is the binary entry point for supertoml

use clap::Parser;

#[derive(Parser)]
#[command(name = "supertoml")]
#[command(about = "A super TOML tool")]
struct Args {
    /// Name to greet
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    match args.name {
        Some(name) => println!("Hello {name}, from supertoml!"),
        None => println!("Hello from supertoml!"),
    }
}
