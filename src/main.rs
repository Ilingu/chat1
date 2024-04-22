mod args;

use std::fs;

use args::AppArgs;
use chat1::chat1;

fn main() {
    let app_args = AppArgs::parse();
    let sha1_digest = match app_args {
        AppArgs::File(filepath) => chat1(fs::read(filepath).expect("Failed to open file")),
        AppArgs::Text(text) => chat1(text.into_bytes()),
    };
    println!("{sha1_digest}");
}
