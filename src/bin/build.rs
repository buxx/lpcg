use lpcg::{builder::Builder, input::Input};

fn main() {
    let input = Input::new()
        .layer(vec!["body", "bodies", "female", "fur_tan"])
        .layer(vec!["head", "heads", "rabbit", "fur_tan"]);
    let image = Builder::new("./spritesheets".to_string()).build(input);
    image.save("out.png").unwrap();
}
