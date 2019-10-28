use std::path::PathBuf;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    // using a for in loop we loop through the lines of content and if the line contains one of the arguments we print it to stdout
    for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
}
}   
