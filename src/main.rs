use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;

/// Search for a pattern in a filename and replace it
#[derive(Parser)]
#[command(name = "Fd&Replace")]
#[command(author, version, about)]
struct Cli {
    /// The path where to search
    path: std::path::PathBuf,
    /// The pattern to search for
    before: String,
    /// The pattern to replace with
    after: String,
}

fn main() {
    let args = Cli::parse();
    let re = Regex::new(&args.before).unwrap();
    let after = args.after;
    // Search dir args.path, capture files that match the Regex
    for entry in WalkDir::new(args.path) {
        let entry = entry.unwrap();
        if re.is_match(entry.path().to_str().unwrap()) {
            let new_path = re.replace_all(entry.path().to_str().unwrap(), after.clone());
            let new_path = format!("{}", new_path);
            let _result = std::fs::rename(entry.path(), new_path);
        }
    }
}
