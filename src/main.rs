use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Toml,
    Json,
    Dotenv,
    Exports,
}

#[derive(Parser)]
#[command(name = "supertoml")]
#[command(about = "A super TOML tool")]
struct Args {
    file: String,
    table: String,
    #[arg(short, long, value_enum, default_value = "toml")]
    output: OutputFormat,
}

fn main() {
    let args = Args::parse();
    
    match run(&args) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run(args: &Args) -> Result<String, supertoml::SuperTomlError> {
    let mut resolver = supertoml::Resolver::new(vec![]);
    let resolved_values = resolver.resolve_table(&args.file, &args.table)?;
    
    match args.output {
        OutputFormat::Toml => supertoml::format_as_toml(&resolved_values),
        OutputFormat::Json => supertoml::format_as_json(&resolved_values),
        OutputFormat::Dotenv => supertoml::format_as_dotenv(&resolved_values),
        OutputFormat::Exports => supertoml::format_as_exports(&resolved_values),
    }
}
