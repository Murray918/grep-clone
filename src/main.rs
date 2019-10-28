#![allow(unused_variables)]
#![allow(unused_imports)]



use failure::ResultExt;
use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    let path = "party.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
}

