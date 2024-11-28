use clap::Parser;
use pulldown_html_ext::HtmlConfig;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Convert markdown to HTML with custom styling"
)]
struct Args {
    /// Input markdown file (omit for stdin)
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Output HTML file (omit for stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Config file in TOML format
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Read input
    let input = match args.input {
        Some(path) => fs::read_to_string(path)?,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    // Load config
    let config = match args.config {
        Some(path) => {
            let config_str = fs::read_to_string(path)?;
            toml::from_str(&config_str).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to parse config: {}", e),
                )
            })?
        }
        None => HtmlConfig::default(),
    };

    // Convert markdown to HTML
    let html = pulldown_html_ext::push_html(&input, &config);

    // Write output
    match args.output {
        Some(path) => fs::write(path, html)?,
        None => println!("{}", html),
    }

    Ok(())
}
