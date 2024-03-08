// https://github.com/grantshandy/htmxchat/blob/main/build.rs

use std::process::Command;

fn main() {
    // TODO: will probably need to add more paths
    println!("cargo:rerun-if-changed=./src");

    let output = format!("{}/output.css", std::env::var("OUT_DIR").unwrap());

    Command::new("tailwindcss")
        .arg("--minify")
        .args(["-i", "src/input.css"])
        .args(["-o", output.as_str()])
        .status()
        .expect("error running tailwind");
}
