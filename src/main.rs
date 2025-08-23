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
    /// Path to the TOML file
    file: String,
    
    /// Name of the table to process
    table: String,
    
    /// Output format
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
    let toml_value = supertoml::load_toml_file(&args.file)?;
    let table = supertoml::extract_table(&toml_value, &args.table)?;
    
    match args.output {
        OutputFormat::Toml => supertoml::format_as_toml(&table),
        OutputFormat::Json => supertoml::format_as_json(&table),
        OutputFormat::Dotenv => supertoml::format_as_dotenv(&table),
        OutputFormat::Exports => supertoml::format_as_exports(&table),
    }
}
