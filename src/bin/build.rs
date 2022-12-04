use std::env;

use lpcg::{builder::Builder, input::Input};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let in_ = &args[1];
    let last = &args.last();
    let out = match last {
        Some(out) => out,
        None => {
            return Err("❌ You must provide arguments".to_string());
        }
    };
    if !out.ends_with(".png") {
        return Err(format!(
            "❌ given output filename ('{}') must end with .png",
            out
        ));
    }

    let input = match Input::from_str(&args[2..args.len() - 1].join(" ")) {
        Ok(input) => input,
        Err(error) => {
            return Err(format!(
                "❌ Unable to parse given input : '{}'",
                error.to_string()
            ))
        }
    };
    let build_result = Builder::new(in_.to_string()).build(input);

    for error in &build_result.errors {
        println!("❗ {}", error);
    }

    if let Some(image) = build_result.output {
        image.save(&out).unwrap();
        println!("✅ Generated file at '{}'", out);
        Ok(())
    } else {
        Err("❌ No output builded".to_string())
    }
}
