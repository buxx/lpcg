use lpcg::inspect::Inspector;
use std::{env, path::PathBuf};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("❌ You must provide argument".to_string());
    }

    for identifier in Inspector::new(PathBuf::from(&args[1])).identifiers() {
        println!("{}", identifier)
    }

    Ok(())
}
