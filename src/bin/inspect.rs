use std::env;
use walkdir::WalkDir;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("❌ You must provide argument".to_string());
    }
    let in_ = &args[1];

    for entry in WalkDir::new(in_)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".png") {
            let mut layer_names = vec![];
            for part in entry.path().iter() {
                layer_names.push(part.to_string_lossy())
            }
            println!(
                "{}",
                layer_names
                    .join("::")
                    .strip_prefix(&format!("{}::", in_))
                    .expect("must start with given folder")
                    .strip_suffix(".png")
                    .expect("must finish by .png")
            );
        }
    }

    Ok(())
}
