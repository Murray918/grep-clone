#![allow(unused_variables)]
#![allow(unused_imports)]

use exitfailure::ExitFailure;
use failure::ResultExt;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() -> Result<(), ExitFailure> {
    // let path = "party.txt";
    let args = Cli::from_args();
    let path = Path::display(&args.path);
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", path))?;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("file content: {}", content);
        }
    }
    Ok(())
}
