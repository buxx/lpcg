use std::path::PathBuf;

use lpcg::{builder::Builder, input::Input};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "build character spritesheet")]
struct Args {
    #[structopt(parse(from_os_str))]
    spritesheet: PathBuf,

    #[structopt(required = true)]
    identifiers: Vec<String>,

    #[structopt(parse(from_os_str))]
    output: PathBuf,

    #[structopt(short, long)]
    variant: Option<String>,
}

fn main() -> Result<(), String> {
    let opt = Args::from_args();

    if !opt.output.to_string_lossy().ends_with(".png") {
        return Err(format!(
            "❌ given output filename ('{}') must end with .png",
            &opt.output.to_string_lossy()
        ));
    }

    let input = match Input::from_str(&opt.identifiers.to_vec().join(" "), opt.variant) {
        Ok(input) => input,
        Err(error) => {
            return Err(format!(
                "❌ Unable to parse given input : '{}'",
                error.to_string()
            ))
        }
    };
    let build_result = Builder::new(opt.spritesheet).build(input);

    for error in &build_result.errors {
        println!("❗ {}", error);
    }

    if let Some(image) = build_result.output {
        image.save(&opt.output).unwrap();
        println!("✅ Generated file at '{}'", opt.output.to_string_lossy());
        Ok(())
    } else {
        Err("❌ No output builded".to_string())
    }
}
