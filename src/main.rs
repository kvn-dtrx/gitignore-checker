// ---
// description:
// ---

// ---

mod decomposed_path;
mod rules;
mod utils;

use clap::Parser;
use rules::Rules;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Path to the .gitignore file
    gitignore: Option<PathBuf>,

    // Path to check against the gitignore rules
    // TODO: Decide whether this shall be a real or hypothetical path—
    // as the result for hypothetical directory paths is currently wrong.
    path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::init_logger();
    let args = Args::parse();

    if let (Some(gitignore), Some(path)) = (args.gitignore, args.path) {
        let rules = Rules::from_file(&gitignore)?;
        let _ignored = rules.ignore(&path);
        // println!("Is ignored? {}", _ignored);
    } else {
        println!("Usage: provide both <GITIGNORE> and <PATH> arguments");
    }

    Ok(())
}
