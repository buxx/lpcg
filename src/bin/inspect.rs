use lpcg::inspect::Inspector;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "list spritesheet identifiers")]
struct Args {
    #[structopt(parse(from_os_str))]
    spritesheet: PathBuf,
}

fn main() -> Result<(), String> {
    let opt = Args::from_args();

    for identifier in Inspector::new(PathBuf::from(&opt.spritesheet)).identifiers() {
        println!("{}", identifier)
    }

    Ok(())
}
