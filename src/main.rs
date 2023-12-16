use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            let mut parts = line.split(&args.pattern).peekable();
            while let Some(part) = parts.next() {
                print!("{}", part,);
                if parts.peek() != None {
                    print!("{}", args.pattern.red());
                }
            }
            println!();
        }
    }
    Ok(())
}
