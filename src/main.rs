// ---
// description:
// ---

// ---

mod decomposed_path;
mod rules;
use rules::Rules;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .gitignore file
    gitignore: Option<PathBuf>,

    /// Path to check against the gitignore rules
    path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    if let (Some(gitignore), Some(path)) = (args.gitignore, args.path) {
        // Both arguments provided, do your processing
        let rules = Rules::from_file(&gitignore)?;
        let ignored = rules.ignore(&path);

        println!("Is ignored? {}", ignored);
    } else {
        // One or both arguments missing, handle gracefully
        println!("Usage: provide both <GITIGNORE> and <PATH> arguments");
    }

    Ok(())
}
