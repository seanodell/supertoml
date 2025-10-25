use clap::{Parser, ValueEnum};
use strum::{Display, EnumString};

#[derive(Clone, Debug, ValueEnum, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
enum OutputFormat {
    Toml,
    Json,
    Dotenv,
    Exports,
    Tfvars,
}

#[derive(Parser)]
#[command(name = "supertoml")]
#[command(about = "A super TOML tool")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    file: String,
    table: String,
    #[arg(short, long, value_enum, default_value = "toml")]
    output: OutputFormat,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = change_to_file_directory(&args.file) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    match run(&args) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn change_to_file_directory(file_path: &str) -> Result<(), String> {
    use std::path::Path;

    let path = Path::new(file_path);

    let absolute_path = path
        .canonicalize()
        .map_err(|e| format!("Failed to resolve file path '{}': {}", file_path, e))?;

    if let Some(parent) = absolute_path.parent() {
        std::env::set_current_dir(parent).map_err(|e| {
            format!(
                "Failed to change to directory '{}': {}",
                parent.display(),
                e
            )
        })?;
    }

    Ok(())
}

fn run(args: &Args) -> Result<String, supertoml::SuperTomlError> {
    use std::path::Path;

    let mut resolver = supertoml::Resolver::new(vec![
        &supertoml::plugins::BeforePlugin as &dyn supertoml::Plugin,
        &supertoml::plugins::ImportPlugin as &dyn supertoml::Plugin,
        &supertoml::plugins::TemplatingPlugin as &dyn supertoml::Plugin,
        &supertoml::plugins::AfterPlugin as &dyn supertoml::Plugin,
    ]);

    let filename = Path::new(&args.file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&args.file);

    let resolved_values =
        resolver.resolve_table_with_meta(filename, &args.table, &args.output.to_string())?;

    match args.output {
        OutputFormat::Toml => supertoml::format_as_toml(&resolved_values),
        OutputFormat::Json => supertoml::format_as_json(&resolved_values),
        OutputFormat::Dotenv => supertoml::format_as_dotenv(&resolved_values),
        OutputFormat::Exports => supertoml::format_as_exports(&resolved_values),
        OutputFormat::Tfvars => supertoml::format_as_tfvars(&resolved_values),
    }
}
