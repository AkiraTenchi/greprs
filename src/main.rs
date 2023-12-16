use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read the file");

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
}
